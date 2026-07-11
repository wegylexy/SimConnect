//! Spawns an AI aircraft on the ground just ahead of the user aircraft and
//! taxis it slowly forward for a while, then removes it. Demonstrates the
//! full AI-object lifecycle end to end: `ai_create_non_atc_aircraft` ->
//! `RECV_ASSIGNED_OBJECT_ID` -> `ai_release_control` ->
//! repeated `DataDefinitionGuard::set_data_on_sim_object` -> `ai_remove_object`
//! — confirmed against a live MSFS2024 instance (see GAPS.md).
//!
//! Spawns using the user's own aircraft title (`TITLE`), since an
//! arbitrary title isn't guaranteed to be installed — a bad title gets a
//! `RECV_EXCEPTION::CreateObjectFailed`, not a wire-format bug.
//!
//! The forward-offset/taxi math is a flat-plane approximation (fine at the
//! tens-of-meters scale this demo moves at; do not reuse it for anything
//! that needs to stay accurate over longer distances or near the poles).
//!
//! ```sh
//! cargo run --example ai_taxi            # Windows named pipe
//! cargo run --example ai_taxi -- 500      # TCP, port 500
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

/// Forward body-frame velocity — writing this and letting the sim's own
/// per-frame physics integrate position from it reads as genuinely smooth
/// motion, unlike teleporting `PLANE LATITUDE`/`PLANE LONGITUDE` (no
/// interpolation between writes, however frequent).
#[derive(DataDefinition, Debug, Clone)]
struct AiVelocity {
    #[simconnect(units = "feet per second")]
    velocity_body_z: f64,
}

/// Sends a named client event (`event_name`, e.g. `"SPOILERS_OFF"`) at
/// `object_id` — the same `map_client_event_to_sim_event` +
/// `transmit_client_event` pair `SimConnect::set_com_frequency_hz` uses for
/// the user aircraft, just targeting an arbitrary object id instead of the
/// implicit `0` (user). `event_id` must be unique per distinct event name
/// used in a program, same as any other client event mapping.
async fn send_client_event(
    sim: &SimConnect,
    object_id: u32,
    event_id: u32,
    event_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    sim.map_client_event_to_sim_event(event_id, event_name)
        .await?;
    sim.transmit_client_event(object_id, event_id, 0, 0, EventFlags::empty())
        .await?;
    Ok(())
}

/// Meters-to-degrees conversion at `latitude_deg`, flat-plane approximate.
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
    let app_name = "simconnect-ai-taxi-example";

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
            .expect("usage: ai_taxi <tcp-port> (named pipe transport is Windows-only)")
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

    // Spawn ~70m directly ahead, on the ground — `on_ground: true` makes
    // the sim snap it to the actual surface elevation, so the altitude
    // given here doesn't need to be exact.
    let (dlat, dlon) = offset_degrees(user.plane_latitude, user.plane_heading_degrees_true, 70.0);
    let init_position = InitPosition {
        latitude: user.plane_latitude + dlat,
        longitude: user.plane_longitude + dlon,
        altitude: user.plane_altitude,
        pitch: 0.0,
        bank: 0.0,
        heading: user.plane_heading_degrees_true,
        on_ground: true,
        airspeed: 0,
    };
    let create_request_id = 2;
    sim.ai_create_non_atc_aircraft(&user.title.0, "TAXI1", &init_position, create_request_id)
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
    println!("AI object {object_id} created");

    sim.ai_release_control(object_id, 3).await?;

    // Newly created aircraft default to whatever ground state their own
    // flight model considers normal for "on the ground, idle throttle" —
    // for many airliners (this one included) that's ground spoilers
    // extended, since real certified ground-spoiler logic auto-deploys on
    // weight-on-wheels + idle/no thrust, which is exactly this AI copy's
    // state. Not a bug in this example; explicitly configure it for
    // taxiing instead: spoilers stowed, strobes off (conventionally only
    // used entering/crossing a runway, not general taxi), taxi light on.
    //
    // Quirk confirmed live: `STROBES_OFF` visibly works, but `SPOILERS_OFF`
    // transmits successfully (no exception) yet the default Asobo A320/
    // A321's spoilers stay visibly deployed. That aircraft's speedbrake
    // animation is very likely driven by its own custom Airbus FBW systems
    // (Lvars, via WASM) rather than listening to the classic FSX
    // `SPOILERS_*` events at all, which only touch the generic
    // `SPOILERS HANDLE POSITION` simvar — a known limitation of that
    // aircraft model, not a bug here. Reaching its real Lvars would need
    // SimConnect's `ClientData` API, which this crate doesn't implement
    // yet (`RecvId::ClientData` is recognized but has no send/decode
    // support — see GAPS.md).
    send_client_event(&sim, object_id, 10, "SPOILERS_OFF").await?;
    send_client_event(&sim, object_id, 11, "STROBES_OFF").await?;
    send_client_event(&sim, object_id, 12, "TAXI_LIGHTS_ON").await?;
    send_client_event(&sim, object_id, 13, "LANDING_LIGHTS_OFF").await?;

    let ai_velocity = sim.define_data::<AiVelocity>(2).await?;

    // Taxi forward at ~10 knots (~16.9 ft/s) for 20 seconds. Reinforced
    // every 2s (not written continuously) since ground friction/rolling
    // resistance will otherwise decay it between writes — the sim's own
    // physics does the frame-by-frame integration and interpolation in
    // between, which is the point: no manual position teleporting at all.
    const TAXI_SPEED_FPS: f64 = 16.9;
    const REINFORCE_EVERY_SECS: u64 = 2;
    const DURATION_SECS: u64 = 20;
    for elapsed in (0..DURATION_SECS).step_by(REINFORCE_EVERY_SECS as usize) {
        ai_velocity
            .set_data_on_sim_object(
                object_id,
                &AiVelocity {
                    velocity_body_z: TAXI_SPEED_FPS,
                },
            )
            .await?;
        println!("t={elapsed}s: velocity_body_z={TAXI_SPEED_FPS} ft/s");
        tokio::time::sleep(std::time::Duration::from_secs(REINFORCE_EVERY_SECS)).await;
    }

    println!("removing AI object {object_id}");
    sim.ai_remove_object(object_id, 4).await?;

    // `user_state`/`ai_position` drop here -> ClearDataDefinition sent
    // automatically (fire-and-forget onto this runtime).
    Ok(())
}
