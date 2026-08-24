# Gaps: this crate vs. the official SimConnect SDK

Tracks what's *not* implemented yet, and why. Anything not listed here is
done. Sourcing policy: official SDK docs (`docs.flightsimulator.com`) and
independently-observed wire-protocol facts (opcode numbers, build
numbers, field layouts) inform this crate; this project is MIT and
doesn't cite or attribute any LGPL-licensed source, even for facts.

## `RECV_ID` / opcode coverage

| Item | Added | Why not yet |
|---|---|---|
| `GetInputEvent`/`EnumerateInputEventParams` reply decode (`RECV_GET_INPUT_EVENT`/`RECV_ENUMERATE_INPUT_EVENT_PARAMS`) | MSFS2020 | `get_input_event`'s reply struct is now ground-truthed and decoded (`recv::kittyhawk::RecvGetInputEvent`, `DOUBLE` case only — `STRING` value's wire shape unconfirmed); `EnumerateInputEventParams`'s reply still has no typed decode |
| `FlowEvent` reply decode (`RECV_FLOW_EVENT`) | MSFS2024 (SDK 1.4.0) | send side implemented (`send::subscribe_to_flow_event`, opcode ground-truthed), but no reply decode yet |
| Camera API beyond `CameraAcquire`/`CameraSetRelative6DOF`/`CameraGetStatus` (`CameraRelease`/`CameraGet`/`CameraSet`/`CameraEnableFlag`/`CameraDisableFlag`/`EnumerateCameraDefinitions`/etc., plus `CameraFlag`/`CameraDataMask` enum discriminants) | post-FSX | `camera_acquire`/`camera_set_relative_6dof` live-confirmed, `RECV_CAMERA_STATUS` decode ground-truthed; the rest of this large subsystem not investigated |
| CommBus beyond `SubscribeToCommBusEvent`/`CallCommBusEvent` (`UnsubscribeToCommBusEvent`, `RECV_COMM_BUS` reply decode) | post-FSX | `subscribe_to_comm_bus_event`/`call_comm_bus_event` implemented (opcodes/layouts ground-truthed); the rest not investigated |
| `HEvent` in the Event API | post-FSX | not investigated |
| `RecvFacilityDataEnd`'s field list beyond the two request ids | MSFS2020 | unconfirmed |
| `SIMCONNECT_FACILITY_DATA_TYPE`/`SIMCONNECT_INPUT_EVENT_TYPE` enum discriminants | MSFS2020 | field layouts confirmed, but exposed as raw `u32` (`RecvFacilityData::data_type`, `InputEventDescriptor::event_type`) rather than variants — `FACILITY_DATA_TYPE`'s member order is documented, `INPUT_EVENT_TYPE`'s isn't confirmed from any source, so both were left as raw integers to avoid mixing a confirmed enum with a guessed one |
| `AICreateEnrouteATCAircraft_EX1` decode support | MSFS2024 | send builder exists, opcode cross-confirmed against an independent reimplementation, but not yet live-tested (its sibling `ai_create_simulated_object_ex1` is live-confirmed) |
| `ControllersList`'s entry struct (`ControllerItem`/`parse_controller_item`) | MSFS2020 | **known broken.** Two live captures (5 devices, then 6) gave inconsistent per-entry widths, meaning entries are likely variable-length (probably a per-device axis/button array) rather than the crate's current fixed-width guess. Needs systematic offset analysis across both captures, not a fixed-width struct |
| `EnumerateSimobjectAndLiveryList` reply decode (`RECV_ENUMERATE_SIMOBJECT_AND_LIVERY_LIST`) | MSFS2024 | send side implemented and live-confirmed (`enumerate_sim_objects_and_liveries`, opcode `0xF000005B`), but no reply decode — `RecvId::EnumerateSimobjectAndLiveryList` (38) exists with an explicit "No decode support yet" doc comment. **Real, currently-blocked downstream need** (not just a coverage gap): a consumer (POSCON's `tauri-launcher`, `src-tauri/simconnect/simconnect-rs`) needs this to inject AI traffic on MSFS2024 — every 2020-era hardcoded container title (`"Generic Airliner Twin Engines Asobo 00"`, `"Airbus A320 Neo Asobo"`, etc.) now fails `AICreateNonATCAircraft`/`ai_create_simulated_object_ex1` with `CreateObjectFailed`, because MSFS2024 replaced that whole AI-traffic content set with a new, differently-named `passiveaircraft` package family (`fs24-asobo-passiveaircraft-a320family`, `-b737family`, `-generic-glider`, etc., found by inspecting a real installed MSFS2024's `InstalledPackagesPath`) whose real container titles aren't published anywhere and can't be read off disk (MSFS2024's streamed-content installs pack model data into opaque `.fsarchive` blobs, not plain-text `aircraft.cfg`). Enumerating them at runtime via this reply is the only currently-known way to get real, working titles. **Wire layout, from the MSFS2024 SDK's `SimConnect.h`** (`SIMCONNECT_RECV_ENUMERATE_SIMOBJECT_AND_LIVERY_LIST : SIMCONNECT_RECV_LIST_TEMPLATE` + `SIMCONNECT_ENUMERATE_SIMOBJECT_LIVERY`): the generic `ListTemplate` header (`request_id`/`array_size`/`entry_number`/`out_of`, all `u32`, same as `parse_controllers_list`/`parse_jetway_data`'s own `parse_list_template` call), then `array_size` entries of `{ aircraft_title: fixed_str(256), livery_name: fixed_str(256) }` (Latin-1, null-terminated, matching `ControllerItem::device_name`'s own `r.fixed_str(256)` pattern in `parse_controller_item`). Implementation should mirror `parse_controllers_list`/`ControllerItem` exactly (a `RecvEnumerateSimobjectAndLiveryList { list: ListTemplate, entries: Vec<EnumerateSimobjectLivery> }` + `parse_enumerate_simobject_and_livery_list`, gated `#[cfg(feature = "sunrise")]`), with round-trip tests per this crate's existing `recv.rs` test conventions (build a fake inbound packet with `PacketWriter::new_inbound`/`finish_inbound` per CLAUDE.md's wire-format gotcha #0, not `new`/`finish`). The response can be paginated across multiple packets for a large install (`entry_number`/`out_of`) — a caller needs to accumulate across all pages before treating the list as complete, not just consume the first packet. Needs live-sim verification (`simconnect-cli` or `tauri-launcher`'s own consumer, which currently has a temporary hand-rolled decode of this exact struct directly in `src-tauri/src/simconnect.rs` reading raw `PacketReader` bytes — once this crate has a real typed decoder + a published version bump, that hand-rolled decode should be deleted in favor of depending on this crate's own `recv::parse_enumerate_simobject_and_livery_list`). |

## Zero-copy pass — read side only

`Connection::recv` reuses one buffer across calls (no per-packet
allocation once it's grown to fit the largest packet seen); `send::*`
builders in `simconnect_proto::send` still allocate a fresh `Vec<u8>` per
call via `PacketWriter::new`. Pooling the write side would mean threading
a reusable buffer through all ~30 builder signatures for a much
lower-frequency code path — not done.

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
- Verifying an actual ARM64 build (no `#[repr(packed)]` pointer casts, no
  `usize`-width-dependent wire encoding, explicit `u32`/`u64` throughout —
  the intent holds by construction, but this hasn't actually been built
  for an ARM64 target).

## Verification status

Every opcode/struct in this crate needs its exact byte layout checked
against a real sim — there's no public canonical wire-format spec, only a
C header. Live MSFS2024 ("SunRise") passes have confirmed, end to end:

- Protocol negotiation; `add_to_data_definition`/`request_data_on_sim_object`
  round-tripping `TITLE`/`PLANE ALTITUDE`; `RECV_SIMOBJECT_DATA` decoding.
- `subscribe_to_system_event`/`RECV_EVENT` for `"Sim"`/`"Pause"`/`"Pause_EX1"`
  (`PauseStateEx1::LEGACY`/`FULL` remain SDK-documented but not
  independently captured — only `SIM`/`ACTIVE` are confirmed).
- `RECV_EXCEPTION` decoding for `NameUnrecognized` and `CreateObjectFailed`.
- Full AI-object lifecycle: `ai_create_non_atc_aircraft` →
  `RECV_ASSIGNED_OBJECT_ID` → `ai_release_control` →
  `set_data_on_sim_object` (repeated position writes) → `ai_remove_object`.
- `request_jetway_data` (from a real gate, RCTP/Taipei Taoyuan — 38 real
  entries decoded correctly), `request_facilities_list`,
  `request_facilities_list_ex1` vs. `request_all_facilities` (confirmed
  `_EX1` is scoped to nearby facilities, not a synonym for "all"),
  `request_system_state` (`"AircraftLoaded"`), `unsubscribe_to_facilities`,
  `camera_acquire`/`RECV_CAMERA_STATUS`,
  `enumerate_sim_objects_and_liveries`, `get_input_event`.
- COM frequency read/write (`set_com_frequency_bcd16`/`set_com_frequency_hz`)
  — found transmitting with a nonexistent notification group id `0`,
  **fixed** by using `EventFlags::GROUP_ID_IS_PRIORITY` +
  `group_priority::HIGHEST` instead; now live-confirmed both directions,
  plus raw (no-wrapper) `COM_STBY_RADIO_SET_HZ`/`COM_STBY_RADIO_SWAP` with
  the same fix.

Still outstanding against a real sim: facility data (send side beyond the
OPEN/CLOSE AIRPORT bracketing requirement — confirmed required, not yet
enforced at compile time), controllers list (confirmed broken, see
above), input-event enumeration (decodes correctly, but individual
subscription/triggering via `subscribe_input_event`/`set_input_event` is
untested), `ai_set_aircraft_flight_plan`, and everything else implemented
so far that hasn't specifically been exercised live yet.

`SimConnect_Close` — this crate doesn't send an explicit close packet;
connections are torn down by dropping the transport (pipe/socket EOF).
Whether the real client sends a wire message on `Close` (vs. just closing
the socket) is unconfirmed.

## Detecting menu vs. in-flight state — no clean signal yet

There's no confirmed one-shot way in this crate to tell "sitting at the
main menu / ready-to-fly screen, no flight actually in progress" apart
from "actually flying, or paused mid-flight":
- `AircraftLoaded` (`request_system_state`) reports a valid aircraft path
  even at a pure menu screen (MSFS preloads a default aircraft for the
  menu background).
- `events::system::SIM` reads `1` (running) at both the ready-to-fly menu
  screen and in-flight.
- `events::system::PAUSE`/`PAUSE_EX1` read `0`/`OFF` both while genuinely
  flying unpaused *and* at a freshly-loaded "Ready to fly" gate screen.

Some combination of these (or a simvar like `SIM ON GROUND` alongside a
camera-state check) might work, but hasn't been tried.

## Remaining `SimConnect_*` functions with no equivalent yet

The Weather API (`WeatherCreateStation`/`SetObservation`/
`RequestCloudState`/etc.) and `SimConnect_Text` (on-screen text) remain
entire FSX-era subsystems with zero coverage — not investigated at all.
The Menu API's first member (`menu_add_item`) is implemented
(ground-truthed); `MenuAddSubItem`/`MenuDeleteItem`/`MenuDeleteSubItem`
aren't. Also missing: `SimConnect_UnsubscribeToFacilities_EX1`,
`SimConnect_UnsubscribeToFlowEvent`, `SimConnect_UnsubscribeToCommBusEvent`,
`SimConnect_UnsubscribeInputEvent`, camera subsystem functions listed
above, `SimConnect_AICreateParkedATCAircraft_EX1`,
`SimConnect_AICreateNonATCAircraft_EX1`,
`SimConnect_AddFacilityDataDefinitionFilter`/
`ClearAllFacilityDataDefinitionFilters`, `SimConnect_FlightLoad`/
`FlightSave`/`FlightPlanLoad`, `SimConnect_ExecuteAction`,
`SimConnect_ExecuteMissionAction`/`CompleteCustomMissionAction`,
`SimConnect_RequestResponseTimes`, camera-world-locker functions
(`RequestCameraWorldLocker`/`DeleteCameraWorldLocker`/
`Subscribe(Un)ToCameraWorldLockerStatusUpdate`), and
`SubscribeToCameraStatusUpdate`/`UnsubscribeToCameraStatusUpdate`.

`SimConnect_CallDispatch`/`GetNextDispatch` have no equivalent by design —
this crate's `async` `recv()`/`recv_ref()` model doesn't need a
dispatch/callback loop. `SimConnect_InsertString`/`RetrieveString`/
`GetLastSentPacketID` are local marshalling helpers with no wire
component, not applicable to a from-scratch reimplementation.
