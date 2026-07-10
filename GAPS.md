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
C header. A `simconnect-cli` smoke-test run against a live MSFS 2020 or
2024 instance is still outstanding for everything implemented so far.
