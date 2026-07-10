# SimConnect (Rust)

A pure-Rust, from-scratch client for Microsoft Flight Simulator's SimConnect
wire protocol — no `SimConnect.dll` dependency, so it can be statically
linked into any Rust binary (Windows, or remote/cross-platform over TCP).

This is a Rust port of an earlier C# implementation, targeting FSX's
protocol range as the core scope for now. MSFS 2020 ("KittyHawk") and
MSFS 2024 ("SunRise") protocol entries and opcodes are also included,
behind the `kittyhawk`/`sunrise` feature flags described below.

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

Full runnable version: [`examples/data_definition.rs`](https://github.com/wegylexy/SimConnect/blob/rust/simconnect/examples/data_definition.rs)
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
  type's module docs in `simconnect-proto`'s
  [`bcd.rs`](https://github.com/wegylexy/SimConnect/blob/rust/simconnect-proto/src/bcd.rs)
  for the derivation).
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

| Capability | Official SDK (2024) | Prior C# implementation | This crate |
|---|---|---|---|
| Transport | Named pipe, TCP | Named pipe only, hardcoded path | Named pipe + TCP + `SimConnect.cfg` discovery |
| Protocol table | RTM…SunRise (12.2/282174.999) | RTM…FSX SE beta (10/63003) | Same 5-entry table, reconnect-per-attempt negotiation |
| `RECV_ID` coverage | 39+ (incl. `FlowEvent`) | 27 (FSX set) | 36 (FSX set + MSFS2020 facility/jetway/controllers/input-event enumeration; `FlowEvent` and a few input-event opcodes still open) |
| Facility APIs | Modern (`FACILITY_DATA`, `JETWAY_DATA`, ...) + legacy lists | Legacy lists only | Modern facility/jetway data only — legacy `AIRPORT_LIST`/`VOR_LIST`/`NDB_LIST`/`WAYPOINT_LIST` decode isn't implemented (no confirmed opcode/struct source) |
| COM frequency (8.33 kHz) | `Units="Hz"` datum + `_HZ` client events | BCD16 only (25 kHz) | Both — `set_com_frequency_hz`/`set_com_frequency_bcd16` convenience methods |
| Concurrency | Callback-driven | `async`/`Task` | `async fn` end to end (tokio) |
| Link model | `SimConnect.dll` | Managed assembly | Statically-linked Rust crate |
