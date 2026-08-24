//! Connects to a running simulator, reports which protocol version was
//! negotiated, and prints the first few inbound packets it receives. Used
//! as a manual smoke test against a live MSFS 2020/2024 instance — there is
//! no way to exercise the real named pipe in CI.

use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app_name = "simconnect-cli";

    #[cfg(windows)]
    let sim = match simconnect::SimConnect::open_local(app_name).await {
        Ok(sim) => sim,
        Err(e) => {
            eprintln!("failed to connect over the local named pipe: {e}");
            if let Some(port) = env::args().nth(1) {
                eprintln!("retrying over TCP on 127.0.0.1:{port}");
                match port.parse() {
                    Ok(port) => {
                        match simconnect::SimConnect::open_tcp(app_name, "127.0.0.1", port).await {
                            Ok(sim) => sim,
                            Err(e) => {
                                eprintln!("TCP connection also failed: {e}");
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("invalid port argument");
                        std::process::exit(1);
                    }
                }
            } else {
                std::process::exit(1);
            }
        }
    };

    #[cfg(not(windows))]
    let sim = {
        let port: u16 = env::args()
            .nth(1)
            .expect("usage: simconnect-cli <tcp-port> (named pipe transport is Windows-only)")
            .parse()
            .expect("port must be a number");
        match simconnect::SimConnect::open_tcp(app_name, "127.0.0.1", port).await {
            Ok(sim) => sim,
            Err(e) => {
                eprintln!("failed to connect: {e}");
                std::process::exit(1);
            }
        }
    };

    println!("negotiated protocol: {:?}", sim.protocol());

    #[cfg(feature = "sunrise")]
    verify_enumerate_simobject_and_livery_list(&sim).await;

    for _ in 0..5 {
        match sim.recv().await {
            Ok(packet) => println!(
                "received {} bytes: {:02x?}",
                packet.len(),
                &packet[..packet.len().min(32)]
            ),
            Err(e) => {
                eprintln!("recv error: {e}");
                break;
            }
        }
    }
}

/// Live-verifies `recv::sunrise::parse_enumerate_simobject_and_livery_list`
/// against a real running sim: sends the request, reads packets until the
/// matching reply arrives (skipping anything else that shows up first, e.g.
/// system-state pushes), decodes it, and prints every title/livery pair.
/// `object_type` 1 == SIMCONNECT_SIMOBJECT_TYPE_AIRCRAFT.
#[cfg(feature = "sunrise")]
async fn verify_enumerate_simobject_and_livery_list(sim: &simconnect::SimConnect) {
    use simconnect::proto::{codec::PacketReader, enums::RecvId, recv::sunrise};

    const REQUEST_ID: u32 = 4242;
    const OBJECT_TYPE_AIRCRAFT: u32 = 1;

    match sim
        .enumerate_sim_objects_and_liveries(REQUEST_ID, OBJECT_TYPE_AIRCRAFT)
        .await
    {
        Ok(send_id) => println!("sent enumerate_sim_objects_and_liveries (send_id={send_id})"),
        Err(e) => {
            eprintln!("failed to send enumerate_sim_objects_and_liveries: {e}");
            return;
        }
    }

    let mut total_entries = 0usize;
    for attempt in 0..200 {
        let packet = match sim.recv().await {
            Ok(packet) => packet,
            Err(e) => {
                eprintln!("recv error while waiting for enumerate reply: {e}");
                return;
            }
        };
        let mut r = PacketReader::new(&packet);
        let header = match r.header() {
            Ok(h) => h,
            Err(_) => continue,
        };
        if RecvId::from_u32(header.id) != Some(RecvId::EnumerateSimobjectAndLiveryList) {
            continue;
        }
        match sunrise::parse_enumerate_simobject_and_livery_list(&mut r) {
            Ok(list) => {
                println!(
                    "decoded page: request_id={} entry_number={} out_of={} array_size={}",
                    list.list.request_id,
                    list.list.entry_number,
                    list.list.out_of,
                    list.list.array_size
                );
                for entry in &list.entries {
                    println!(
                        "  title={:?} livery={:?}",
                        entry.aircraft_title, entry.livery_name
                    );
                }
                total_entries += list.entries.len();
                if list.list.entry_number + 1 >= list.list.out_of {
                    println!("done: {total_entries} total entries decoded");
                    return;
                }
            }
            Err(e) => {
                eprintln!("decode failed on attempt {attempt}: {e:?}");
                return;
            }
        }
    }
    eprintln!("gave up waiting for the full enumerate reply after 50 packets");
}
