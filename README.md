# SimConnect (Rust)

A pure-Rust, from-scratch client for Microsoft Flight Simulator's SimConnect
wire protocol — no `SimConnect.dll` dependency, so it can be statically
linked into any Rust binary (Windows, or remote/cross-platform over TCP).

This branch is a Rust port of an earlier C# implementation (see
`main`/`dev`), targeting FSX's protocol range as the core scope for now.
MSFS 2020 ("KittyHawk") and MSFS 2024 ("SunRise") protocol entries and
opcodes are also included, behind the `kittyhawk`/`sunrise` feature flags
described below.

## Crates

Published on crates.io under a `flybywireless-` prefix (the plain
`simconnect` name is already taken), but every crate's actual import name
stays short — add an alias so `use simconnect::...` etc. keep working:

```toml
[dependencies]
simconnect = { package = "flybywireless-simconnect", version = "0.1" }
```

| Published name | Import name | Purpose |
|---|---|---|
| `flybywireless-simconnect-proto` | `simconnect_proto` | Wire encoding/decoding, enums, protocol-version table. No I/O. |
| `flybywireless-simconnect-derive` | `simconnect_derive` | `#[derive(DataDefinition)]` proc macro — see below. Re-exported from `simconnect`, not usually depended on directly. |
| `flybywireless-simconnect` | `simconnect` | The client: transport (named pipe / TCP), `SimConnect.cfg` discovery, connection/negotiation, public API. |
| `flybywireless-simconnect-cli` | — (binary: `simconnect-cli`) | Manual smoke-test binary — connect to a running sim and print what comes back. |

## Feature flags

`simconnect` is `async fn` end to end and requires tokio unconditionally —
there is no sync API and no `async` feature flag. One correct, genuinely
non-blocking core (`tokio::net::TcpStream`, or tokio's own IOCP-backed
`NamedPipeClient` on Windows) beat maintaining a blocking core plus a
hand-duplicated async wrapper.

| Flag | Default | Gates |
|---|---|---|
| `kittyhawk` | on | MSFS 2020 protocol entry and any MSFS2020-only opcodes/enums added later |
| `sunrise` | on (implies `kittyhawk`) | MSFS 2024 protocol entry and any MSFS2024-only opcodes/enums added later |

Disable `kittyhawk`/`sunrise` if you specifically want to cap negotiation at
FSX and avoid pulling in newer-version code paths — the protocol is
forward-compatible, so this mainly matters if you're deliberately limiting
feature surface, not for basic connectivity.

```sh
# default: full 2024-capable negotiation
cargo build

# capped at FSX/MSFS2020 (no SunRise entry)
cargo build --no-default-features --features kittyhawk
```

## Quick start

The recommended way to read/write sim data is `#[derive(DataDefinition)]`
(`simconnect-derive`), ported from the prior C# client's reflection-based
`[DataDefinition]` attribute as compile-time codegen — Rust has no runtime
reflection, so this generates the field-by-field `AddToDataDefinition`
registration and `RECV_SIMOBJECT_DATA` decode/encode at compile time
instead:

```rust
use simconnect::DataDefinition;
use simconnect::proto::enums::{DataRequestFlags, Period};

#[derive(DataDefinition, Debug)]
struct Radios {
    // `__1__mhz`: an index segment and a unit segment, classified by
    // content (all-digits vs. a recognized unit alias) rather than
    // position — `__mhz__1` would work identically. This derives the
    // datum name as `"COM ACTIVE FREQUENCY:1"` (colon, matching the real
    // simvar) and infers `units = "MHz"`, both from the field name alone.
    com_active_frequency__1__mhz: f64,
    com_standby_frequency__1__mhz: f64,

    // Or skip the naming convention entirely and just say what you mean —
    // shorter Rust-side names, explicit name/units, same result:
    #[simconnect(name = "COM ACTIVE FREQUENCY:2", units = "MHz")]
    com2_active: f64,
    #[simconnect(name = "COM STANDBY FREQUENCY:2", units = "MHz")]
    com2_standby: f64,
}

let sim = simconnect::SimConnect::open_local("my-addon").await?; // Windows named pipe
println!("negotiated: {:?}", sim.protocol());

let radios = sim.define_data::<Radios>(1).await?; // one AddToDataDefinition per field
sim.request_data_on_sim_object(1, radios.define_id(), 0, Period::Second, DataRequestFlags::empty(), 0, 0, 0).await?;

loop {
    // `recv_ref` reuses the connection's own buffer instead of allocating
    // a fresh `Vec<u8>` per packet — worth it in a polling loop like this
    // one; `recv()` (returns an owned `Vec<u8>`) is still there if you'd
    // rather not deal with the guard's lifetime.
    let packet = sim.recv_ref().await?;
    // ...dispatch on the header's RecvId to RecvId::SimObjectData, then:
    let radios: Radios = radios.decode(sim_object_data.data)?;
    println!("COM1 active: {} MHz", radios.com_active_frequency__1__mhz);
}
// `radios` drops here -> ClearDataDefinition is sent automatically
// (fire-and-forget onto the current tokio runtime).
```

Full runnable version: [`simconnect/examples/data_definition.rs`](simconnect/examples/data_definition.rs)
(`cargo run --example data_definition`). Supported field types: `bool`,
`i32`, `i64`, `f32`, `f64`, the fixed-width `String8`..`String260` types,
`Waypoint`/`LatLonAlt`/`Xyz`/`MarkerState`, and `Bco16`/`FrequencyBcd16`
(each requiring `#[simconnect(units = "BCO16")]`/`"Frequency BCD16"`
explicitly — see the next section for why those two are special).

**Known hiccup**: rustc's `non_snake_case` lint doesn't know about the
`__unit` convention and flags the double underscore on every such field.
Harmless (just a warning), but add `#[allow(non_snake_case)]` on the
struct if it bothers you — see the example above.

For anything not covered by the derive macro, the lower-level API is still
there:

```rust
use simconnect::SimConnect;

let sim = SimConnect::open_local("my-addon").await?;
let send_id = sim.map_client_event_to_sim_event(1, "TOGGLE_MASTER_BATTERY").await?;
let packet = sim.recv().await?; // dispatch on the header's RecvId to decode further
```

**On error propagation**: SimConnect never acknowledges success — every
method above returns `Ok(send_id)` as soon as the packet is written, not
once the sim has processed it. The *only* signal that comes back is a NACK,
`RECV_EXCEPTION`, and it arrives later, out of band, on whatever the next
`recv()`/`recv_ref()` happens to return — there's no way to make a single
call's `Result` carry it without either guessing a timeout or routing
every packet through a background dispatcher (which would undo the
zero-copy read path above). So exception handling stays a
plain check in your own receive loop, matching the `send_id` you're
watching against `simconnect_proto::recv::RecvException::matches`:

```rust
if header.id == simconnect::proto::enums::RecvId::Exception as u32 {
    let exception = simconnect::proto::recv::parse_exception(&mut r)?;
    if exception.matches(send_id) {
        eprintln!("that call failed: {exception:?}");
    }
}
```

## AI-controlled objects

The full FSX-era AI object API is implemented: `ai_create_parked_atc_aircraft`,
`ai_create_enroute_atc_aircraft`, `ai_create_non_atc_aircraft`,
`ai_create_simulated_object`, `ai_release_control`, `ai_remove_object`,
`ai_set_aircraft_flight_plan`. The server-assigned object id for a
just-created object arrives later, out of band, as a
`RECV_ASSIGNED_OBJECT_ID` carrying the `request_id` you passed in — dispatch
on `RecvId::AssignedObjectId` and decode with
`simconnect_proto::recv::parse_assigned_object_id`, the same pattern as
exception correlation above.

```rust
use simconnect::proto::data::InitPosition;

let request_id = sim
    .ai_create_non_atc_aircraft(
        "Boeing 747-8i Asobo",
        "N747BA",
        &InitPosition {
            latitude: 47.44,
            longitude: -122.30,
            altitude: 433.0,
            pitch: 0.0,
            bank: 0.0,
            heading: 270.0,
            on_ground: true,
            airspeed: 0,
        },
        1,
    )
    .await?;

let packet = sim.recv_ref().await?;
let mut r = simconnect::proto::codec::PacketReader::new(&packet);
let header = r.header()?;
if header.id == simconnect::proto::enums::RecvId::AssignedObjectId as u32 {
    let assigned = simconnect::proto::recv::parse_assigned_object_id(&mut r)?;
    if assigned.request_id == request_id {
        println!("created object id {}", assigned.object_id);
    }
}
```

To move a created object afterward — e.g. driving its position from
externally-tracked traffic data — call `ai_release_control` first (so the
built-in AI logic isn't fighting your writes), then push position updates
via the same data-definition mechanism used for reading/writing the user
aircraft, targeting the assigned object id instead of `0`. Confirmed
against a live MSFS2024 instance end to end — full runnable version:
[`simconnect/examples/ai_taxi.rs`](simconnect/examples/ai_taxi.rs)
(`cargo run --example ai_taxi`), which also demonstrates letting the
sim's own physics smoothly integrate motion from a written velocity
(`VELOCITY BODY Z`) instead of teleporting position every tick:

```rust
use simconnect::DataDefinition;

#[derive(DataDefinition)]
struct Position {
    #[simconnect(name = "PLANE LATITUDE", units = "degrees")]
    latitude: f64,
    #[simconnect(name = "PLANE LONGITUDE", units = "degrees")]
    longitude: f64,
    #[simconnect(name = "PLANE ALTITUDE", units = "feet")]
    altitude: f64,
    #[simconnect(name = "PLANE HEADING DEGREES TRUE", units = "degrees")]
    heading: f64,
}

let movement = sim.define_data::<Position>(2).await?; // separate define_id from any other definition

// ...once per incoming traffic update — `DataDefinitionGuard::set_data_on_sim_object`
// encodes `value` and fills in `movement`'s own define_id/flags/unit size for you:
movement
    .set_data_on_sim_object(
        assigned.object_id,
        &Position {
            latitude: 47.4502,
            longitude: -122.3088,
            altitude: 480.0,
            heading: 275.0,
        },
    )
    .await?;
```

Freezing a created object in place (rather than driving it) is a named
client event too, same mechanism as any other — `FREEZE_LATITUDE_LONGITUDE_SET`/
`FREEZE_ALTITUDE_SET`/`FREEZE_ATTITUDE_SET`, each targeting the assigned
object id with `data0 = 1`. Combined with writing the generic light
simvars (`LIGHT NAV`/`LIGHT BEACON`/`LIGHT STROBE`/`LIGHT LANDING`/
`LIGHT TAXI`, all `bool`) via the same data-definition mechanism as
`Position` above, this is enough to hover a static AI aircraft somewhere
and cycle its lights — confirmed live end to end, full runnable version:
[`simconnect/examples/drone_light_show.rs`](simconnect/examples/drone_light_show.rs)
(`cargo run --example drone_light_show`).

MSFS2024 added `_EX1` variants of `AICreateSimulatedObject`/
`AICreateEnrouteATCAircraft` (adding a livery parameter for modular
SimObjects) — both are implemented (`SimConnect::ai_create_simulated_object_ex1`/
`ai_create_enroute_atc_aircraft_ex1`). `ai_create_simulated_object_ex1` is
live-confirmed against a real sim; `ai_create_enroute_atc_aircraft_ex1`'s
opcode is cross-confirmed against an independent reimplementation but not
yet live-tested (see `GAPS.md`).

## System events

`subscribe_to_system_event`/`unsubscribe_to_system_event` take any system
event name as a plain string; `simconnect_proto::events::system` has
constants for the commonly-needed ones (`SIM`, `PAUSE`, and — behind
`kittyhawk` — `PAUSE_EX1`), each confirmed against a live MSFS2024
capture with the exact `dwData` values observed for real pause/menu
states (see their doc comments). `PAUSE_EX1`'s bitmask decodes with
`simconnect_proto::enums::PauseStateEx1`.

```rust
use simconnect::proto::events::system;

sim.subscribe_to_system_event(1, system::PAUSE_EX1).await?;
```

## Key events (client events)

`transmit_client_event`/`map_client_event_to_sim_event` take any key event
name as a plain string, same as system events above — just a different
opcode and a much larger vocabulary (aircraft systems, not sim lifecycle).
`simconnect_proto::events::client` has **2100+** of these, transcribed
directly from the official SimConnect key-event docs and organized into one
submodule per doc category (`client::autopilot`, `client::engine`,
`client::radio_navigation`, `client::view_camera`, etc. — 13 in total; see
that module's doc comment for the full list).

```rust
use simconnect::proto::enums::EventFlags;
use simconnect::proto::events::client::engine;

sim.map_client_event_to_sim_event(1, engine::MAGNETO_START).await?;
sim.transmit_client_event(0, 1, 1, 0, EventFlags::DEFAULT).await?;
```

Each constant is cross-referenced against both the MSFS2020 and MSFS2024 SDK
docs: names present in both are ungated, names new to the 2024 docs are
behind `#[cfg(feature = "sunrise")]`, and names the vendor docs mark
deprecated carry a real `#[deprecated]` attribute. None of these are
wire-verified against a live sim the way `COM_RADIO_SET`/`system::SIM`/
`system::PAUSE` are — treat the spelling as "per the vendor docs."

Regenerated by `scripts/gen_events.py` (run `python scripts/gen_events.py
--fetch` to pick up a new SDK docs revision) — see that script and
`events::client`'s module doc for why it parses raw HTML tables directly
rather than going through a summarizing fetch, which previously mangled a
large fraction of these names.

## Gotchas: BCD16, octal squawk codes, and other common hiccups

- **`Bco16` and `FrequencyBcd16` are 4 bytes on the wire, not 2**, even
  though they only wrap a `u16` in Rust. The prior C# client's originals
  both declared their `Data` field as `readonly int` (4 bytes) and mapped
  to `DataType.Int32` in their data-definition registration, even though
  only the low 16 bits are ever meaningful — this crate's `write_le`/
  `read_le` on both types (and the derive macro's `DataType::Int32`
  mapping for them) match that, but it's an easy thing to get wrong if
  you're encoding one by hand instead of through this crate.
- **`Bco16` is octal, not decimal BCD**, despite the similar name to
  `Bcd16`/`FrequencyBcd16`. Transponder squawk codes are four *octal*
  digits (0-7 each, e.g. `1200`, `7700`) packed one per nibble — `Bco16::
  from_octal_digits(7700)` panics if you pass a digit outside 0-7 (a
  decimal code like `1289` is not a valid squawk and can't round-trip
  through this format). "BCO" ("binary coded octal") vs. "BCD" ("binary
  coded decimal") is the whole naming distinction; easy to misread at a
  glance.
- **`FrequencyBcd16` only handles 25 kHz-spaced (and coarser) COM
  frequencies** — it structurally cannot represent 8.33 kHz-only channels
  (several of them collide with a 25 kHz-spaced value's encoding; see the
  type's module docs in `simconnect-proto/src/bcd.rs` for the derivation).
  For 8.33 kHz-capable radios, request the frequency as a plain
  `f64`/`u32` with `Units = "Hz"`/`"MHz"` instead — `add_com_frequency_definition`/
  `set_com_frequency` already do this automatically; there's no
  "`Frequency8_33Khz`" type here on purpose (two earlier attempts at one
  didn't hold up — nibble overloading is ambiguous between certain 25 kHz/
  8.33 kHz values, and equal-thirds channel math doesn't match the real
  ICAO 8.33 kHz channel plan's round displayed values; both are preserved
  in git history as worked examples of why they don't fit).
- **`#[simconnect(units = ...)]` is required, and exact, for `Bco16`/
  `FrequencyBcd16` fields** (`"BCO16"`/`"Frequency BCD16"`) — the derive
  macro rejects the field at compile time if the units string is missing
  or doesn't match exactly, rather than guessing.
- **Fixed-width string/struct fields (`String8`..`String260`, `Waypoint`,
  `LatLonAlt`, `Xyz`, `MarkerState`) don't take a `units` override at
  all** — they're structural data types, not unit-converted scalars, so
  the derive macro rejects `#[simconnect(units = ...)]` on those fields
  too (matching the same restriction the C# original had).

## Comparison with the official SDK and other ports

| Capability | Official SDK (2024) | Prior C# client (`main`/`dev`) | This crate |
|---|---|---|---|
| Transport | Named pipe, TCP | Named pipe only, hardcoded path | Named pipe + TCP + `SimConnect.cfg` discovery |
| Protocol table | RTM…SunRise (12.2/282174.999) | RTM…FSX SE beta (10/63003) | Same 5-entry table, reconnect-per-attempt negotiation |
| `RECV_ID` coverage | 39+ (incl. `FlowEvent`) | 27 (FSX set) | 36 (FSX set + MSFS2020 facility/jetway/controllers/input-event enumeration; `FlowEvent` and a few input-event opcodes still open) |
| Facility APIs | Modern (`FACILITY_DATA`, `JETWAY_DATA`, ...) + legacy lists | Legacy lists only | Modern facility/jetway data only — legacy `AIRPORT_LIST`/`VOR_LIST`/`NDB_LIST`/`WAYPOINT_LIST` decode isn't implemented (no confirmed opcode/struct source) |
| COM frequency (8.33 kHz) | `Units="Hz"` datum + `_HZ` client events | BCD16 only (25 kHz) | Both — `set_com_frequency_hz`/`set_com_frequency_bcd16` convenience methods |
| Concurrency | Callback-driven | `async`/`Task` | `async fn` end to end (tokio) |
| Link model | `SimConnect.dll` | Managed assembly | Statically-linked Rust crate |
