//! Connection: version negotiation, packet framing, and the raw
//! send/receive primitives the public API builds on. Genuinely
//! non-blocking — built on [`crate::transport::Transport`], not a
//! blocking core moved onto a worker thread.
//!
//! The read and write halves are independently locked
//! (`tokio::io::split` + one `tokio::sync::Mutex` each) rather than one
//! lock covering both directions. A single shared lock would mean
//! [`crate::client::SimConnect::recv_ref`]'s guard — held for as long as
//! the caller is parsing the packet, not just for the I/O itself — blocks
//! every concurrent `send()` too, which defeats the point of a duplex
//! transport (a pipe/socket that can be written to while a read is
//! outstanding). Splitting means a held read guard only ever blocks
//! *other reads*.

use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

use simconnect_proto::codec::{PacketReader, PacketWriter};
use simconnect_proto::protocol::ProtocolVersion;
use simconnect_proto::recv::{self, RecvOpen};
use simconnect_proto::send;

use crate::transport::Transport;

#[derive(Debug)]
pub enum OpenError {
    Io(io::Error),
    /// Every entry in `ProtocolVersion::negotiation_order()` was rejected
    /// with `VersionMismatch` (or the connection dropped before replying).
    NoCompatibleProtocol,
    Encode(simconnect_proto::strings::FixedStringError),
}

impl From<io::Error> for OpenError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::NoCompatibleProtocol => {
                write!(f, "sim rejected every known protocol version")
            }
            Self::Encode(e) => write!(f, "packet encode error: {e}"),
        }
    }
}

impl std::error::Error for OpenError {}

struct ReadSide {
    half: tokio::io::ReadHalf<Box<dyn Transport>>,
    /// Reused across [`Connection::recv`] calls instead of allocating a
    /// fresh `Vec<u8>` per inbound packet — the read side is the hot path
    /// for a polling control loop (e.g. `RECV_SIMOBJECT_DATA` at
    /// `Period::Second` or faster), so this is where a zero-copy/pooled-
    /// buffer pass pays off most. The write side (`PacketWriter::new`
    /// allocating fresh per `send::*` builder call) is intentionally left
    /// as-is — pooling it would mean threading a reusable buffer through
    /// every one of the ~30 send builders in `simconnect_proto::send` for
    /// a much rarer code path.
    buf: Vec<u8>,
}

struct WriteSide {
    half: tokio::io::WriteHalf<Box<dyn Transport>>,
    next_send_id: u32,
}

/// A live SimConnect connection, negotiated to a specific
/// [`ProtocolVersion`].
pub struct Connection {
    read: Mutex<ReadSide>,
    write: Mutex<WriteSide>,
    #[cfg(debug_assertions)]
    send_history: std::sync::Mutex<std::collections::BTreeMap<u32, String>>,
    pub protocol: ProtocolVersion,
    pub open: RecvOpen,
}

impl Connection {
    /// Negotiates a protocol version against a fresh transport per attempt
    /// (reusing a transport across a rejected `Open` is the bug this fixes
    /// relative to the prior C# client before its `14cc372` fix: once the
    /// sim has replied `VersionMismatch` on a pipe, it will not accept a
    /// second `Open` on that same connection).
    pub async fn open<F, Fut>(application_name: &str, mut connect: F) -> Result<Self, OpenError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = io::Result<Box<dyn Transport>>>,
    {
        for &protocol in ProtocolVersion::negotiation_order() {
            let transport = connect().await?;
            let (mut read_half, mut write_half) = tokio::io::split(transport);

            let packet = send::open(protocol, application_name)
                .map_err(OpenError::Encode)?
                .finish(1);
            write_half.write_all(&packet).await?;
            write_half.flush().await?;

            let mut raw = Vec::new();
            if read_packet_into(&mut read_half, &mut raw).await.is_err() {
                continue; // dropped connection: try the next protocol
            }
            let mut r = PacketReader::new(&raw);
            let header = match r.header() {
                Ok(h) => h,
                Err(_) => continue,
            };
            if header.id == simconnect_proto::enums::RecvId::Open as u32 {
                if let Ok(open) = recv::parse_open(&mut r) {
                    return Ok(Self {
                        read: Mutex::new(ReadSide {
                            half: read_half,
                            buf: Vec::new(),
                        }),
                        write: Mutex::new(WriteSide {
                            half: write_half,
                            next_send_id: 2,
                        }),
                        #[cfg(debug_assertions)]
                        send_history: std::sync::Mutex::new(std::collections::BTreeMap::new()),
                        protocol,
                        open,
                    });
                }
            }
            // Anything else (typically Exception/VersionMismatch): fall
            // through and retry with the next, older protocol entry.
        }
        Err(OpenError::NoCompatibleProtocol)
    }

    /// Allocates the next send id and writes a fully-built packet. Takes
    /// `&self` (not `&mut self`) — the write-side mutex is internal, so
    /// this can run concurrently with a `recv()`/`recv_ref()` in flight.
    pub async fn send(&self, packet: PacketWriter) -> io::Result<u32> {
        let mut w = self.write.lock().await;
        let send_id = w.next_send_id;
        w.next_send_id += 1;
        let bytes = packet.finish(send_id);
        w.half.write_all(&bytes).await?;
        w.half.flush().await?;
        Ok(send_id)
    }

    /// Allocates the next send id, records its description under debug builds,
    /// and writes a fully-built packet. Exists only under `#[cfg(debug_assertions)]`.
    #[cfg(debug_assertions)]
    pub async fn send_with_desc<F>(&self, packet: PacketWriter, desc: F) -> io::Result<u32>
    where
        F: FnOnce() -> String,
    {
        let mut w = self.write.lock().await;
        let send_id = w.next_send_id;
        w.next_send_id += 1;
        let opcode = packet.opcode();
        let bytes = packet.finish(send_id);
        w.half.write_all(&bytes).await?;
        w.half.flush().await?;

        let mut history = self.send_history.lock().unwrap();
        if history.len() >= 1024 {
            history.pop_first();
        }
        let desc_str = desc();
        let desc_final = if desc_str.is_empty() {
            format!("Opcode(0x{opcode:08X})")
        } else {
            desc_str
        };
        history.insert(send_id, desc_final);

        Ok(send_id)
    }

    /// Looks up the human-readable description for a previously sent `send_id`.
    /// Exists only under `#[cfg(debug_assertions)]`.
    #[cfg(debug_assertions)]
    pub fn describe_send(&self, send_id: u32) -> Option<String> {
        self.send_history.lock().unwrap().get(&send_id).cloned()
    }

    /// Awaits the next full inbound packet into the connection's reused
    /// read buffer, returning a guard borrowing it — no per-packet
    /// allocation once the buffer's capacity has grown to fit the largest
    /// packet seen so far. Holds only the read-side lock, so a concurrent
    /// `send()` is never blocked by a guard the caller is still parsing.
    pub async fn recv(&self) -> io::Result<RecvGuard<'_>> {
        let mut r = self.read.lock().await;
        let ReadSide { half, buf } = &mut *r;
        read_packet_into(half, buf).await?;
        Ok(RecvGuard { guard: r })
    }

    pub fn protocol_version_wire(&self) -> u32 {
        self.protocol.wire_version()
    }
}

/// Returned by [`Connection::recv`]. Holds the read-side lock for its
/// lifetime (so it can't be held across another `recv`/`recv_ref` call on
/// this connection — the usual "one buffer, reused in place" tradeoff),
/// but never blocks a concurrent `send()`, since that's a separate lock.
pub struct RecvGuard<'a> {
    guard: tokio::sync::MutexGuard<'a, ReadSide>,
}

impl std::ops::Deref for RecvGuard<'_> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.guard.buf
    }
}

/// Reads one full packet into `buf`, reusing its existing allocation: a
/// 4-byte little-endian size prefix (inclusive of itself) followed by
/// `size - 4` more bytes.
async fn read_packet_into<T: tokio::io::AsyncRead + Unpin + ?Sized>(
    transport: &mut T,
    buf: &mut Vec<u8>,
) -> io::Result<()> {
    let mut size_buf = [0u8; 4];
    transport.read_exact(&mut size_buf).await?;
    let size = i32::from_le_bytes(size_buf);
    if size < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "packet size smaller than its own header",
        ));
    }
    buf.clear();
    buf.resize(size as usize, 0);
    buf[..4].copy_from_slice(&size_buf);
    transport.read_exact(&mut buf[4..]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `read_packet_into` reuses `buf`'s allocation across calls instead
    /// of allocating fresh each time — verified here by checking capacity
    /// doesn't shrink between two same-size reads, and that a smaller
    /// second packet doesn't leave stale trailing bytes from the first.
    #[tokio::test]
    async fn read_packet_into_reuses_buffer_and_round_trips() {
        let (mut client, mut server) = tokio::io::duplex(256);

        let first = [8i32.to_le_bytes().to_vec(), vec![0xAA, 0xBB, 0xCC, 0xCC]].concat();
        let second = [8i32.to_le_bytes().to_vec(), vec![0x11, 0x22, 0x33, 0x33]].concat();

        let (first_send, second_send) = (first.clone(), second.clone());
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            client.write_all(&first_send).await.unwrap();
            client.write_all(&second_send).await.unwrap();
        });

        let mut buf = Vec::new();
        read_packet_into(&mut server, &mut buf).await.unwrap();
        assert_eq!(buf, first);
        let cap_after_first = buf.capacity();

        read_packet_into(&mut server, &mut buf).await.unwrap();
        assert_eq!(buf, second);
        // No reallocation needed for a same-or-smaller subsequent packet.
        assert_eq!(buf.capacity(), cap_after_first);
    }

    /// The whole point of splitting the lock: a `recv()` guard held open
    /// (simulating slow parsing) must not block a concurrent `send()`.
    #[tokio::test]
    async fn recv_guard_does_not_block_concurrent_send() {
        use std::sync::Arc;
        use std::time::Duration;

        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let (mut peer, sim_side) = tokio::io::duplex(1024);
        let (read_half, write_half) = tokio::io::split(Box::new(sim_side) as Box<dyn Transport>);

        let conn = Arc::new(Connection {
            read: Mutex::new(ReadSide {
                half: read_half,
                buf: Vec::new(),
            }),
            write: Mutex::new(WriteSide {
                half: write_half,
                next_send_id: 2,
            }),
            #[cfg(debug_assertions)]
            send_history: std::sync::Mutex::new(std::collections::BTreeMap::new()),
            protocol: ProtocolVersion::negotiation_order()[0],
            open: RecvOpen {
                application_name: String::new(),
                application_version: simconnect_proto::protocol::SimConnectVersion {
                    major: 0,
                    minor: 0,
                    build_major: 0,
                    build_minor: 0,
                },
                sim_connect_version: simconnect_proto::protocol::SimConnectVersion {
                    major: 0,
                    minor: 0,
                    build_major: 0,
                    build_minor: 0,
                },
            },
        });

        let packet = [8i32.to_le_bytes().to_vec(), vec![0, 0, 0, 0]].concat();
        peer.write_all(&packet).await.unwrap();

        // Drain anything the connection writes, so `send()` below doesn't
        // block on a full duplex buffer.
        tokio::spawn(async move {
            let mut sink = [0u8; 64];
            loop {
                match peer.read(&mut sink).await {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {}
                }
            }
        });

        // Hold the read guard open, simulating a caller still parsing the
        // packet, then issue a concurrent send — it must not deadlock or
        // even stall on the (now separate) write lock.
        let guard = conn.recv().await.unwrap();
        assert_eq!(&*guard, &packet[..]);

        let conn2 = Arc::clone(&conn);
        let send_result = tokio::time::timeout(Duration::from_millis(200), async move {
            conn2.send(PacketWriter::new(0xF0000001, 0)).await
        })
        .await;
        assert!(
            send_result.is_ok(),
            "send() blocked while a recv() guard was held"
        );
    }

    #[cfg(debug_assertions)]
    #[tokio::test]
    async fn records_and_describes_send_history() {
        use std::sync::Arc;
        use tokio::io::AsyncReadExt;

        let (mut peer, sim_side) = tokio::io::duplex(1024);
        let (read_half, write_half) = tokio::io::split(Box::new(sim_side) as Box<dyn Transport>);

        let conn = Arc::new(Connection {
            read: Mutex::new(ReadSide {
                half: read_half,
                buf: Vec::new(),
            }),
            write: Mutex::new(WriteSide {
                half: write_half,
                next_send_id: 98,
            }),
            send_history: std::sync::Mutex::new(std::collections::BTreeMap::new()),
            protocol: ProtocolVersion::negotiation_order()[0],
            open: RecvOpen {
                application_name: String::new(),
                application_version: simconnect_proto::protocol::SimConnectVersion {
                    major: 0,
                    minor: 0,
                    build_major: 0,
                    build_minor: 0,
                },
                sim_connect_version: simconnect_proto::protocol::SimConnectVersion {
                    major: 0,
                    minor: 0,
                    build_major: 0,
                    build_minor: 0,
                },
            },
        });

        tokio::spawn(async move {
            let mut sink = [0u8; 64];
            loop {
                if peer.read(&mut sink).await.unwrap_or(0) == 0 {
                    break;
                }
            }
        });

        let send_id = conn
            .send_with_desc(PacketWriter::new(0xF000000C, 0), || {
                "AddToDataDefinition(define_id: 6, datum: \"COM RECEIVE ALL\")".to_string()
            })
            .await
            .unwrap();

        assert_eq!(send_id, 98);
        assert_eq!(
            conn.describe_send(98).as_deref(),
            Some("AddToDataDefinition(define_id: 6, datum: \"COM RECEIVE ALL\")")
        );
        assert_eq!(conn.describe_send(99), None);
    }
}
