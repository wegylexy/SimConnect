//! Runnable demo of `#[derive(DataDefinition)]` used for a ClientData area
//! instead of a SimObject data definition — maps a name to a
//! `client_data_id`, creates the area, registers the struct's fields by
//! byte offset (`SimConnect::define_client_data`, not `define_data`),
//! writes a value, then requests and prints it back.
//!
//! ClientData is how an add-on shares its own private data (not simvars)
//! between processes — e.g. a WASM gauge and this client talking to each
//! other, or two SimConnect clients coordinating. This example just talks
//! to itself (writes, then reads back its own write) to demonstrate the
//! API end to end without needing a second process.
//!
//! ```sh
//! cargo run --example client_data            # Windows named pipe
//! cargo run --example client_data -- 500      # TCP, port 500
//! ```

use std::env;

use simconnect::proto::enums::{DataRequestFlags, Period};
use simconnect::proto::recv::{self};
use simconnect::DataDefinition;

#[derive(DataDefinition, Debug, Clone, PartialEq)]
struct SharedState {
    counter: i32,
    throttle_percent: f32,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = "simconnect-client-data-example";

    #[cfg(windows)]
    let sim = match env::args().nth(1) {
        Some(port) => {
            simconnect::SimConnect::open_tcp(app_name, "127.0.0.1", port.parse()?).await?
        }
        None => simconnect::SimConnect::open_local(app_name).await?,
    };
    #[cfg(not(windows))]
    let sim = {
        let port: u16 = env::args()
            .nth(1)
            .expect("usage: client_data <tcp-port> (named pipe transport is Windows-only)")
            .parse()?;
        simconnect::SimConnect::open_tcp(app_name, "127.0.0.1", port).await?
    };

    println!("negotiated protocol: {:?}", sim.protocol());

    let client_data_id = 1;
    let define_id = 1;
    sim.map_client_data_name_to_id("SimConnectRsExample.SharedState", client_data_id)
        .await?;
    sim.create_client_data(client_data_id, SharedState::client_data_byte_size(), false)
        .await?;
    let shared = sim
        .define_client_data::<SharedState>(client_data_id, define_id)
        .await?;

    let value = SharedState {
        counter: 1,
        throttle_percent: 75.0,
    };
    shared.set_client_data(&value).await?;
    println!("wrote {value:?}");

    let request_send_id = sim
        .request_client_data(
            client_data_id,
            1, // request_id
            define_id,
            Period::Once,
            DataRequestFlags::empty(),
            0,
            0,
            0,
        )
        .await?;

    loop {
        let packet = sim.recv_ref().await?;
        let mut r = simconnect::proto::codec::PacketReader::new(&packet);
        let header = r.header()?;
        if header.id == simconnect::proto::enums::RecvId::Exception as u32 {
            let exception = recv::parse_exception(&mut r)?;
            if exception.matches(request_send_id) {
                eprintln!("request_client_data failed: {exception:?}");
            }
        } else if header.id == simconnect::proto::enums::RecvId::ClientData as u32 {
            let data = recv::parse_client_data(&mut r)?;
            let read_back: SharedState = shared.decode(data.data)?;
            println!("read back {read_back:?}");
            break;
        }
    }

    Ok(())
}
