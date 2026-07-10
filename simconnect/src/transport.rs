//! Transport abstraction: local named pipe or remote TCP. Genuinely
//! non-blocking — `tokio::net::TcpStream` (already real async), and on
//! Windows, tokio's own IOCP-backed
//! `tokio::net::windows::named_pipe::NamedPipeClient` (no unsafe/FFI
//! needed) — not a blocking transport moved onto a worker thread.

use std::io;
use std::time::Duration;

/// A duplex byte stream to the sim.
pub trait Transport: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send + Unpin {}

impl Transport for tokio::net::TcpStream {}

#[cfg(windows)]
impl Transport for tokio::net::windows::named_pipe::NamedPipeClient {}

/// Test-only: lets `connection.rs`'s tests drive a `Connection` over an
/// in-memory `tokio::io::duplex` pair instead of a real pipe/socket.
#[cfg(test)]
impl Transport for tokio::io::DuplexStream {}

/// Default local pipe name used by the official SimConnect client/server
/// (`SimConnect.cs`'s hardcoded path in the prior C# client).
pub const DEFAULT_PIPE_NAME: &str = r"\\.\pipe\Microsoft Flight Simulator\SimConnect";

/// Connects a TCP transport to a remote (or loopback) SimConnect server.
pub async fn connect_tcp(host: &str, port: u16) -> io::Result<tokio::net::TcpStream> {
    let stream = tokio::net::TcpStream::connect((host, port)).await?;
    stream.set_nodelay(true)?;
    Ok(stream)
}

/// Connects a named-pipe transport (Windows only — the sim itself only
/// runs on Windows, so this is the common local-connection path),
/// retrying briefly if the pipe exists but its listener backlog is
/// momentarily full (`ERROR_PIPE_BUSY`).
#[cfg(windows)]
pub async fn connect_pipe(pipe_name: &str) -> io::Result<tokio::net::windows::named_pipe::NamedPipeClient> {
    use tokio::net::windows::named_pipe::ClientOptions;

    let mut last_err = None;
    for _ in 0..20 {
        match ClientOptions::new().open(pipe_name) {
            Ok(client) => return Ok(client),
            Err(e) if e.raw_os_error() == Some(231) /* ERROR_PIPE_BUSY */ => {
                last_err = Some(e);
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Err(e) => return Err(e),
        }
    }
    Err(last_err.unwrap_or_else(|| io::Error::new(io::ErrorKind::NotFound, pipe_name)))
}
