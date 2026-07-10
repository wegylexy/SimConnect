//! Runnable demo of `#[derive(DataDefinition)]` — connects to a running
//! sim, registers a COM radio data definition, requests it once a second,
//! and prints decoded values as they arrive.
//!
//! ```sh
//! cargo run --example data_definition            # Windows named pipe
//! cargo run --example data_definition -- 500      # TCP, port 500
//! ```

use std::env;

use simconnect::proto::enums::{DataRequestFlags, Period};
use simconnect::proto::recv::{self};
use simconnect::DataDefinition;

// `__1__mhz`: a numeric index segment and a unit-alias segment, peeled
// off the field name and classified by content (not position — `__mhz__1`
// works the same). Together they derive the datum name
// `"COM ACTIVE FREQUENCY:1"` (colon, matching the real simvar — Rust
// identifiers can't contain `:` at all, so `__` is the delimiter) and
// infer `units = "MHz"`, with no attribute needed.
//
// Rustc's `non_snake_case` lint doesn't know about this convention and
// flags the double underscore — harmless, but silence it here rather
// than let every user of this pattern see a warning on every field.
#[allow(non_snake_case)]
#[derive(DataDefinition, Debug)]
struct Radios {
    com_active_frequency__1__mhz: f64,
    com_standby_frequency__1__mhz: f64,
    com_active_frequency__2__mhz: f64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = "simconnect-data-definition-example";

    #[cfg(windows)]
    let sim = match env::args().nth(1) {
        Some(port) => simconnect::SimConnect::open_tcp(app_name, "127.0.0.1", port.parse()?).await?,
        None => simconnect::SimConnect::open_local(app_name).await?,
    };
    #[cfg(not(windows))]
    let sim = {
        let port: u16 = env::args()
            .nth(1)
            .expect("usage: data_definition <tcp-port> (named pipe transport is Windows-only)")
            .parse()?;
        simconnect::SimConnect::open_tcp(app_name, "127.0.0.1", port).await?
    };

    println!("negotiated protocol: {:?}", sim.protocol());

    let radios = sim.define_data::<Radios>(1).await?; // one AddToDataDefinition per field
    let request_send_id = sim
        .request_data_on_sim_object(
            1,
            radios.define_id(),
            0, // user aircraft
            Period::Second,
            DataRequestFlags::empty(),
            0,
            0,
            0,
        )
        .await?;

    for _ in 0..10 {
        // `recv_ref` reuses the connection's own buffer instead of
        // allocating a fresh `Vec<u8>` per packet — worth it here since
        // this loop polls once a second.
        let packet = sim.recv_ref().await?;
        let mut r = simconnect::proto::codec::PacketReader::new(&packet);
        let header = r.header()?;
        if header.id == simconnect::proto::enums::RecvId::Exception as u32 {
            // SimConnect never ACKs success, only NACKs asynchronously
            // like this — there's no per-call `Result` to check, so a
            // caller who cares which earlier `send_id` failed compares it
            // here against whichever call's return value they're
            // watching (`request_send_id`, above).
            let exception = recv::parse_exception(&mut r)?;
            if exception.matches(request_send_id) {
                eprintln!("request_data_on_sim_object failed: {exception:?}");
            }
        } else if header.id == simconnect::proto::enums::RecvId::SimObjectData as u32 {
            let data = recv::parse_sim_object_data(&mut r)?;
            let radios: Radios = radios.decode(data.data)?;
            println!("{radios:?}");
        }
    }

    // `radios` drops here -> ClearDataDefinition is sent automatically
    // (fire-and-forget onto this runtime).
    Ok(())
}
