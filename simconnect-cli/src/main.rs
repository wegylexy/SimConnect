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
