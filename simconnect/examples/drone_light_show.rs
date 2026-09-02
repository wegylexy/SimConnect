//! Spawns an AI aircraft hovering ~50m in front of the user aircraft,
//! freezes it in place (`FREEZE_*_SET` client events — otherwise an
//! aircraft-categorized object falls under its own flight model), then
//! runs a chasing light-cycle pattern through its nav/beacon/strobe/
//! landing/taxi lights for a while, and removes it. Mirrors `ai_taxi.rs`'s
//! create -> release-control -> drive -> remove lifecycle; see that
//! example and GAPS.md for the parts of this already live-confirmed.
//!
//! Spawns using the user's own aircraft title (`TITLE`), same as
//! `ai_taxi.rs` — an arbitrary installed-simobject title (e.g. one of the
//! "Quad" quadcopter-drone titles surfaced by
//! `enumerate_sim_objects_and_liveries`, tried in an earlier version of
//! this example) isn't guaranteed to render at a size/altitude a human
//! will actually notice, whereas the user's own title is both guaranteed
//! installed and guaranteed recognizable.
//!
//! The forward-offset math is a flat-plane approximation (fine at the
//! tens-of-meters scale this demo spawns at — see `ai_taxi.rs`). 50m was
//! chosen because it's measured from the aircraft's CG reference datum,
//! not its nose — an A320-sized airframe is itself ~10-12m nose-to-CG, so
//! anything closer risks spawning inside/barely clear of the user's own
//! fuselage, invisible for that reason rather than any API failure.
//!
//! ```sh
//! cargo run --example drone_light_show            # Windows named pipe
//! cargo run --example drone_light_show -- 500      # TCP, port 500
//! ```

use std::env;

use simconnect::proto::codec::PacketReader;
use simconnect::proto::data::InitPosition;
use simconnect::proto::enums::{DataRequestFlags, EventFlags, Period, RecvId};
use simconnect::proto::recv;
use simconnect::proto::strings::String256;
use simconnect::{DataDefinition, SimConnect};

#[derive(DataDefinition, Debug, Clone)]
struct UserState {
    title: String256,
    #[simconnect(units = "degrees")]
    plane_latitude: f64,
    #[simconnect(units = "degrees")]
    plane_longitude: f64,
    #[simconnect(units = "feet")]
    plane_altitude: f64,
    #[simconnect(units = "degrees")]
    plane_heading_degrees_true: f64,
}

#[derive(DataDefinition, Debug, Clone, Default)]
struct Lights {
    light_nav: bool,
    light_beacon: bool,
    light_strobe: bool,
    light_landing: bool,
    light_taxi: bool,
}

async fn send_client_event(
    sim: &SimConnect,
    object_id: u32,
    event_id: u32,
    event_name: &str,
    data: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    sim.map_client_event_to_sim_event(event_id, event_name)
        .await?;
    sim.transmit_client_event(object_id, event_id, data, 0, EventFlags::empty())
        .await?;
    Ok(())
}

/// Meters-to-degrees conversion at `latitude_deg`, flat-plane approximate
/// (fine at the tens-of-meters scale used here — see `ai_taxi.rs`).
fn offset_degrees(latitude_deg: f64, heading_deg: f64, meters: f64) -> (f64, f64) {
    const METERS_PER_DEGREE_LAT: f64 = 111_320.0;
    let heading_rad = heading_deg.to_radians();
    let dlat = meters * heading_rad.cos() / METERS_PER_DEGREE_LAT;
    let dlon = meters * heading_rad.sin()
        / (METERS_PER_DEGREE_LAT * latitude_deg.to_radians().cos().abs().max(0.01));
    (dlat, dlon)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = "simconnect-drone-light-show-example";

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
            .expect("usage: drone_light_show <tcp-port> (named pipe transport is Windows-only)")
            .parse()?;
        simconnect::SimConnect::open_tcp(app_name, "127.0.0.1", port).await?
    };

    println!("negotiated protocol: {:?}", sim.protocol());

    let user_state = sim.define_data::<UserState>(1).await?;
    let user_request_id = 1;
    sim.request_data_on_sim_object(
        user_request_id,
        user_state.define_id(),
        0, // user aircraft
        Period::Once,
        DataRequestFlags::empty(),
        0,
        0,
        0,
    )
    .await?;

    let user: UserState = loop {
        let packet = sim.recv_ref().await?;
        let mut r = PacketReader::new(&packet);
        let header = r.header()?;
        if header.id == RecvId::SimObjectData as u32 {
            let data = recv::parse_sim_object_data(&mut r)?;
            if data.request_id == user_request_id {
                break user_state.decode(data.data)?;
            }
        }
    };
    println!("user state: {user:?}");

    // Hover ~50m directly ahead, ~10ft above the user's own altitude.
    // Earlier attempts at 20m and 8m were measured from the aircraft's CG
    // reference datum, not its nose — an A320 airframe is itself ~10-12m
    // nose-to-CG, so both prior spawns likely landed inside/barely clear of
    // the user's own fuselage, invisible for that reason rather than any
    // API failure. 50m clears that with a lot of margin.
    let (dlat, dlon) = offset_degrees(user.plane_latitude, user.plane_heading_degrees_true, 50.0);
    let init_position = InitPosition {
        latitude: user.plane_latitude + dlat,
        longitude: user.plane_longitude + dlon,
        altitude: user.plane_altitude + 10.0,
        pitch: 0.0,
        bank: 0.0,
        heading: user.plane_heading_degrees_true,
        on_ground: false,
        airspeed: 0,
    };

    // Spawn using the user's own aircraft title rather than the
    // "Quad"-family drone titles from the previous attempt — an arbitrary
    // title isn't guaranteed installed/visible, but the user's own title
    // definitely is, and this is the exact approach `ai_taxi.rs` already
    // confirmed live end to end (see GAPS.md).
    let create_request_id = 2;
    sim.ai_create_non_atc_aircraft(&user.title.0, "PSCN1", &init_position, create_request_id)
        .await?;

    let object_id = loop {
        let packet = sim.recv_ref().await?;
        let mut r = PacketReader::new(&packet);
        let header = r.header()?;
        if header.id == RecvId::Exception as u32 {
            let exception = recv::parse_exception(&mut r)?;
            if exception.matches(create_request_id) {
                return Err(format!("ai_create_non_atc_aircraft failed: {exception:?}").into());
            }
        } else if header.id == RecvId::AssignedObjectId as u32 {
            let assigned = recv::parse_assigned_object_id(&mut r)?;
            if assigned.request_id == create_request_id {
                break assigned.object_id;
            }
        }
    };
    println!(
        "drone object {object_id} created ~50m ahead, target lat/lon/alt: {}/{}/{}",
        init_position.latitude, init_position.longitude, init_position.altitude
    );

    sim.ai_release_control(object_id, 3).await?;

    // Lock it in place — an aircraft-categorized AI object otherwise falls
    // under its own flight model the instant control is released.
    send_client_event(&sim, object_id, 10, "FREEZE_LATITUDE_LONGITUDE_SET", 1).await?;
    send_client_event(&sim, object_id, 11, "FREEZE_ALTITUDE_SET", 1).await?;
    send_client_event(&sim, object_id, 12, "FREEZE_ATTITUDE_SET", 1).await?;

    // Read the object's own position back to confirm it actually ended up
    // where we asked, and that it's a real aircraft-model object (this
    // request would throw an exception otherwise).
    let ai_state = sim.define_data::<UserState>(5).await?;
    let ai_request_id = 6;
    sim.request_data_on_sim_object(
        ai_request_id,
        ai_state.define_id(),
        object_id,
        Period::Once,
        DataRequestFlags::empty(),
        0,
        0,
        0,
    )
    .await?;
    let confirmed: Result<UserState, _> = loop {
        let packet = sim.recv_ref().await?;
        let mut r = PacketReader::new(&packet);
        let header = r.header()?;
        if header.id == RecvId::Exception as u32 {
            let exception = recv::parse_exception(&mut r)?;
            if exception.matches(ai_request_id) {
                break Err(format!("readback failed: {exception:?}"));
            }
        } else if header.id == RecvId::SimObjectData as u32 {
            let data = recv::parse_sim_object_data(&mut r)?;
            if data.request_id == ai_request_id {
                break ai_state.decode(data.data).map_err(|e| format!("{e:?}"));
            }
        }
    };
    match confirmed {
        Ok(state) => println!("drone confirmed at: {state:?}"),
        Err(e) => eprintln!("could not confirm drone position/category: {e}"),
    }

    let lights = sim.define_data::<Lights>(2).await?;

    // Chase pattern: exactly one light on at a time, cycling nav -> beacon
    // -> strobe -> landing -> taxi -> repeat, then finish with all off.
    const STEP_MS: u64 = 250;
    const CYCLES: usize = 40;
    let sequence = [
        Lights {
            light_nav: true,
            ..Default::default()
        },
        Lights {
            light_beacon: true,
            ..Default::default()
        },
        Lights {
            light_strobe: true,
            ..Default::default()
        },
        Lights {
            light_landing: true,
            ..Default::default()
        },
        Lights {
            light_taxi: true,
            ..Default::default()
        },
    ];
    for cycle in 0..CYCLES {
        for (i, state) in sequence.iter().enumerate() {
            let set_send_id = lights.set_data_on_sim_object(object_id, state).await?;
            println!("cycle {cycle} step {i}: {state:?} (send_id={set_send_id})");
            // SetDataOnSimObject is fire-and-forget like every other send —
            // give any NACK a chance to arrive so a silently-rejected write
            // (e.g. this object category not implementing that simvar)
            // actually surfaces instead of looking identical to success.
            if let Ok(packet) =
                tokio::time::timeout(std::time::Duration::from_millis(20), sim.recv_ref()).await
            {
                let packet = packet?;
                let mut r = PacketReader::new(&packet);
                if let Ok(header) = r.header() {
                    if header.id == RecvId::Exception as u32 {
                        if let Ok(exception) = recv::parse_exception(&mut r) {
                            if exception.matches(set_send_id) {
                                eprintln!("  set_data_on_sim_object rejected: {exception:?}");
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(STEP_MS)).await;
        }
    }
    lights
        .set_data_on_sim_object(object_id, &Lights::default())
        .await?;

    println!("removing drone object {object_id}");
    sim.ai_remove_object(object_id, 4).await?;

    // `user_state`/`lights` drop here -> ClearDataDefinition sent
    // automatically (fire-and-forget onto this runtime).
    Ok(())
}
