# Gaps: this crate vs. the official SimConnect SDK

Tracks what's *not* implemented yet, and why. Anything not listed here is
done. Sourcing policy: official SDK docs (`docs.flightsimulator.com`) and
independently-observed wire-protocol facts (opcode numbers, build
numbers, field layouts) inform this crate; this project is MIT and
doesn't cite or attribute any LGPL-licensed source, even for facts.

## `RECV_ID` / opcode coverage

| Item | Added | Why not yet |
|---|---|---|
| `GetInputEvent` | MSFS2020 | no confirmed opcode/struct sourced |
| `EnumerateInputEventParams` | MSFS2020 | no confirmed opcode/struct sourced |
| `FlowEvent` | MSFS2024 (SDK 1.4.0) | exists via `SimConnect_SubscribeToFlowEvent` (confirmed purpose: fires on things like "Back On Track"), but no opcode/struct sourced |
| Camera API (`CameraFlag`/`CameraDataMask`) | post-FSX | not investigated |
| CommBus | post-FSX | not investigated |
| `HEvent` in the Event API | post-FSX | not investigated |
| Legacy `RequestFacilitiesList` + `AIRPORT_LIST`/`VOR_LIST`/`NDB_LIST`/`WAYPOINT_LIST` per-entry structs | FSX | official docs only publish the shared `SIMCONNECT_RECV_FACILITIES_LIST` wrapper header (`dwRequestID`/`dwArraySize`/`dwEntryNumber`/`dwOutOf`), not the send opcode or the per-type entry struct layouts; no other source found. `AirportList`/`VorList`/`NdbList`/`WaypointList` `RecvId` values are recognized but have no decode support |
| `RecvFacilityDataEnd`'s field list beyond the two request ids | MSFS2020 | unconfirmed |
| `SIMCONNECT_FACILITY_DATA_TYPE`/`SIMCONNECT_INPUT_EVENT_TYPE` enum discriminants | MSFS2020 | field layouts confirmed, but exposed as raw `u32` (`RecvFacilityData::data_type`, `InputEventDescriptor::event_type`) rather than variants — `FACILITY_DATA_TYPE`'s ~26 member names/declaration order are documented, but `INPUT_EVENT_TYPE`'s order couldn't be confirmed from any source, and mixing a confirmed enum with a guessed one in the same pass risked a quietly-wrong variant, so both were left as raw integers |
| `AICreateSimulatedObject_EX1` / `AICreateEnrouteATCAircraft_EX1` | MSFS2024 | official docs confirm these exist (`_EX1` adds a `szLivery` parameter for modular SimObjects) but no wire opcode sourced — the FSX-era non-`_EX1` functions (`AICreate{Parked,Enroute}ATCAircraft`, `AICreateNonATCAircraft`, `AICreateSimulatedObject`, `AIReleaseControl`, `AIRemoveObject`, `AISetAircraftFlightPlan`) are implemented and work unchanged on MSFS2024 |
| `ClientData` send/decode (`SimConnect_MapClientDataNameToID`/`CreateClientData`/`AddToClientDataDefinition`/`RequestClientData`/`SetClientData`) | FSX | `RecvId::ClientData` is recognized but has no send builder or decode support. Would matter for reading/writing an aircraft's custom Lvars (e.g. the default Asobo A320/A321's speedbrake animation doesn't respond to the classic `SPOILERS_*` client events at all — confirmed live in `simconnect/examples/ai_taxi.rs` — because that aircraft's FBW systems are Lvar-driven, and Lvars aren't reachable through any other part of the SimConnect API) |

## `RecvId` 28+ (`kittyhawk`): partially mis-numbered, found via live capture

A live MSFS2024 capture found the guessed discriminants for the MSFS2020
`RecvId` range (28-36, previously assigned by simple declaration order)
were wrong, and not by a uniform offset: `enumerate_controllers`'s reply
actually arrives as id `32` (guessed as `FacilityMinimalList`, not
`ControllersList`), `enumerate_input_events`'s as id `34` (guessed as
`ControllersList`, not `EnumerateInputEvents`), and
`request_facility_data`'s reply as id `28`/`29` (guessed as `Pick`/
`EventEx1`, not `FacilityData`/`FacilityDataEnd`). `enums.rs` now carries
the four corrected, live-confirmed values (`FacilityData=28`,
`FacilityDataEnd=29`, `ControllersList=32`, `EnumerateInputEvents=34`);
`Pick`, `EventEx1`, `JetwayData`, and `FacilityMinimalList` still carry
unconfirmed placeholder values (30/31/33/36, chosen only to avoid
colliding with the confirmed four) and `RecvId::from_u32` deliberately
does not recognize those four placeholders, so a real wire packet at
those ids falls through as unrecognized instead of being silently
mis-dispatched. Re-deriving their real values needs whatever triggers
`Pick`/`EventEx1`/`ActionCallback` (unclear what user action fires
those) plus fixing the `request_jetway_data` `SizeMismatch` below so its
reply can actually be captured.

Once `enumerate_controllers`'s reply was correctly dispatched (after the
id fix), its entry struct (`ControllerItem`/`parse_controller_item`) was
also found broken. A live capture with 5 real devices (Mouse, Keyboard,
"Pro Flight Cessna Yoke", "Pro Flight Cessna Rudder Pedals", "Saitek
Extreme 3D Pro Stick") gave a precise, consistent entry width — every
device name is exactly **276 bytes** from the next one — vs. the current
struct's assumed `fixed_str(256) + device_id + product_id + composite_id
+ hardware_version(4×i32)` = 284 bytes, 8 bytes too many. It's not simply
a shorter name field either: non-zero, partially-printable bytes appear
at consistent relative offsets (+24 and +120 from each entry's start),
i.e. *inside* what a naive 248-byte name field would cover — meaning the
real struct interleaves at least one GUID-shaped field with the name
rather than putting the name first and numeric fields after. Not
re-derived yet — needs decoding those specific byte ranges (likely a
16-byte GUID, common for DirectInput device enumeration) rather than
guessing further from name-offset deltas alone.

`add_to_facility_definition`'s doc comment already showed `"OPEN
AIRPORT"`/`"CLOSE AIRPORT"` as example field names — live capture
confirmed this bracketing is *required*, not illustrative: registering
`LATITUDE`/`LONGITUDE`/`ALTITUDE` directly (without opening/closing an
`"OPEN AIRPORT"`/`"CLOSE AIRPORT"` pair around them) makes
`request_facility_data` fail with `RECV_EXCEPTION::DataError`; bracketed,
it correctly returns KSEA's lat/long/alt as three `f64`s. Not yet
reflected as an enforced API shape (a caller can still forget the
bracket and get a runtime `DataError` instead of a compile-time
guarantee) — just documented for now.

`request_jetway_data`'s wire encoding (`send::request_jetway_data`) is
confirmed broken: a live call with `parking_indices = []` gets back
`RECV_EXCEPTION::SizeMismatch` at index 0, meaning the sim recognized the
opcode but rejected the total packet size. Six wire-layout variants were
tried live and all six failed the same way (`SizeMismatch`, except the
`fixed_str(256)` icao variant, which instead hit a `RECV_EXCEPTION` code
(38) outside this crate's currently-modeled `SimConnectException` range):
`request_id`+`fixed_str(8)` icao+`count`+indices (the current
implementation); `count` before a `fixed_str(8)` icao; no `request_id` at
all; `fixed_str(5)` icao instead of 8; `fixed_str(256)` icao; and
`request_id`+`fixed_str(8)` icao+one parking index (`[0]`) instead of
none. None of these guesses were right, and there's no more principled
variant to try without an actual source for this opcode's layout — left
broken; re-deriving it needs either a documented layout or a lot more
blind trial-and-error against a live sim.

## Zero-copy pass — read side only

`Connection::recv` reuses one buffer across calls (no per-packet
allocation once it's grown to fit the largest packet seen); `send::*`
builders in `simconnect_proto::send` still allocate a fresh `Vec<u8>` per
call via `PacketWriter::new`. Pooling the write side would mean threading
a reusable buffer through all ~30 builder signatures for a much
lower-frequency code path (one send per user action/subscription setup,
vs. `recv` polled every frame/second in a control loop) — not done.

## Exception correlation — left manual, by design

SimConnect only ever NACKs (`RECV_EXCEPTION`), never ACKs success, and the
NACK arrives later, out of band, on whatever packet `recv()`/`recv_ref()`
next returns. Routing it back to the specific call's own `Result` would
need either a guessed timeout or a background dispatcher task that takes
over all reads — the latter would force `recv_ref()` back to allocating a
`Vec<u8>` per packet for the channel hand-off, undoing the read-side
zero-copy pass. Decided against, in favor of keeping zero-copy:
`simconnect_proto::recv::RecvException::matches(send_id)` is the extent of
the help this crate gives; callers check it in their own receive loop.

## Not attempted

- Cross-platform named-pipe emulation for non-Windows local connections.
  TCP already covers remote connections from any platform; a non-Windows
  *local* transport isn't meaningful since the sim itself is Windows-only.
- Verifying an actual ARM64 build (Rust has no AnyCPU-equivalent
  single-binary-multi-arch concept, and the intent — no
  architecture-specific assumptions — holds by construction: no
  `#[repr(packed)]` pointer casts, no `usize`-width-dependent wire
  encoding, explicit `u32`/`u64` throughout — but this hasn't actually
  been built for an ARM64 target).

## Verification caveat

Every opcode/struct in this crate needs its exact byte layout checked
against a real sim — there's no public canonical wire-format spec, only a
C header.

A live MSFS2024 ("SunRise") smoke test confirmed: protocol negotiation;
`add_to_data_definition`/`request_data_on_sim_object` round-tripping
`TITLE`/`PLANE ALTITUDE` correctly; `RECV_SIMOBJECT_DATA` decoding
correctly; `subscribe_to_system_event`/`RECV_EVENT` decoding correctly for
`"Sim"`/`"Pause"`/`"Pause_EX1"` (`events::system::{SIM,PAUSE,PAUSE_EX1}`,
`enums::PauseStateEx1` — see their doc comments for the exact captured
values); `RECV_EXCEPTION` decoding correctly for both `NameUnrecognized`
(bogus datum name) and `CreateObjectFailed` (invalid
`ai_create_non_atc_aircraft` title). This run is also what caught the
header-framing bug fixed in `codec.rs` — see CLAUDE.md's wire-format
gotcha #0.

`PauseStateEx1::LEGACY`/`FULL` remain SDK-documented but not
independently captured live — `SIM` (Esc menu) and `ACTIVE` (in-sim
play/pause icon) are confirmed, `LEGACY`/`FULL` would need whatever
triggers the keyboard `Pause`/`Break`-key-style full pause, not yet
tested.

A second live pass confirmed the full AI-object lifecycle: `ai_create_non_atc_aircraft`
(using the user's own aircraft title, since an arbitrary title like
`"Boeing 747-8i Asobo"` isn't necessarily installed — that specific title
got `RECV_EXCEPTION::CreateObjectFailed`, not a wire bug) →
`RECV_ASSIGNED_OBJECT_ID` decoding correctly (`object_id` usable
immediately) → `ai_release_control` → `set_data_on_sim_object` writing a
fresh `PLANE LATITUDE`/`PLANE LONGITUDE`/`PLANE ALTITUDE` position 5
times in a row (moving the AI aircraft) → `ai_remove_object`, with zero
exceptions end to end. This is also how `set_data_on_sim_object` was
found broken and fixed: `simconnect_proto::send::set_data_on_sim_object`
used to derive a "count" from `data.len() / unit_size` and send
`(count, unit_size)`, which cannot express `SimConnect_SetDataOnSimObject`'s
real `(ArrayCount, cbUnitSize)` pair for the ordinary single-value
case (`ArrayCount = 0`, `cbUnitSize = total bytes`) — every write failed
with `RECV_EXCEPTION::InvalidDataSize` until the function was changed to
take `array_count`/`unit_size` directly from the caller instead of
deriving them. This also fixes `DataDefinitionGuard::set_data_on_sim_object`
(the derive-macro convenience path), which had the same latent bug,
untested until now.

Still outstanding against a real sim: facility data (send side beyond
the OPEN/CLOSE AIRPORT bracketing note above), jetway data (confirmed
broken, see above), controllers list (confirmed broken, see above),
input-event enumeration (decodes correctly per the `enumerate_controllers`/
`enumerate_input_events` investigation above, but individual input event
subscription/triggering — `subscribe_input_event`/`set_input_event` —
untested), `ai_set_aircraft_flight_plan`, COM frequency read/write, and
everything else implemented so far that hasn't specifically been
exercised live yet.
