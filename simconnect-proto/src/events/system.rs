//! Known SimConnect *system* event names — plain strings passed to
//! `send::subscribe_to_system_event`/`unsubscribe_from_system_event`, not new
//! opcodes or wire structures (unlike the client events, these fire
//! unprompted whenever the sim's own state changes, not in response to a
//! `transmit_client_event` call). The `RecvEvent` a subscription produces
//! has `group_id == u32::MAX` (`SIMCONNECT_UNUSED`, confirmed against a
//! live MSFS2024 capture) and `event_id` equal to whatever id was passed to
//! `subscribe_to_system_event`, not tied to the event name string itself —
//! `event_id` is how a caller with multiple subscriptions tells them apart
//! in its `recv()` loop.

/// Fires whenever the simulation loop starts/stops running —
/// `dwData == 1`/`0`. Confirmed against a live MSFS2024 capture to
/// already read `1` at the ready-to-fly menu screen and unchanged
/// during an active flight — it tracks the sim engine's own run state,
/// not "in a flight" vs. "at a menu" as such; don't use it to
/// distinguish those. `dwData` is otherwise undocumented as anything
/// other than a plain 0/1, so this crate doesn't offer a decode helper
/// (unlike [`PAUSE_EX1`]).
pub const SIM: &str = "Sim";

/// Legacy pause notification — `dwData == 1` while paused (of any
/// kind), `0` otherwise. Confirmed against a live MSFS2024 capture:
/// `dwData == 1` both for an Esc-menu pause and for toggling the
/// in-sim play/pause icon (Active Pause), at the ready-to-fly screen
/// and in-flight alike. Can't distinguish *which* kind of pause (menu,
/// active pause, etc.) — see [`PAUSE_EX1`], which replaces this for
/// that.
///
/// Also confirmed to read `1` (with [`PAUSE_EX1`] `== PauseStateEx1::SIM`)
/// at the main menu *after* having flown and returned to it, vs. `0`/
/// `PauseStateEx1::OFF` at a genuinely fresh app launch before any
/// flight has started — MSFS keeps that flight/world session loaded
/// behind the main menu rather than tearing it down, and counts that
/// as the same kind of pause as pressing Esc mid-flight, even though
/// [`SIM`] reads `1` (running) in both menu states. The `0`/`OFF` case
/// extends to the "Ready to fly" gate screen for a freshly-loaded new
/// flight too (also separately confirmed live) — it's not just literal
/// app launch that reads unpaused, it's specifically "no flight has
/// been through its play/pause lifecycle yet".
pub const PAUSE: &str = "Pause";

/// Bitmask pause notification (see
/// [`crate::enums::PauseStateEx1`] for the bit layout and what's
/// confirmed vs. documented-but-unverified) — added because [`PAUSE`]
/// alone can't tell an Esc-menu pause apart from other pause states.
/// Gated behind `kittyhawk` alongside every other post-FSX addition
/// this crate tracks, even though (unlike most of those) this is a
/// plain string passed to the same FSX-era
/// `subscribe_to_system_event` opcode as [`SIM`]/[`PAUSE`] — no new
/// wire opcode or struct, just a magic string this crate hasn't
/// independently confirmed predates MSFS2020.
#[cfg(feature = "kittyhawk")]
pub const PAUSE_EX1: &str = "Pause_EX1";

/// Fires once per visual frame. FSX-era; carried through unchanged in
/// MSFS2020/2024's SDK docs. Not yet confirmed against a live capture —
/// spelling/casing is as documented, unlike [`SIM`]/[`PAUSE`]/
/// [`PAUSE_EX1`] above which this crate has independently verified.
pub const FRAME: &str = "Frame";
/// Fires once per second of *sim* time (pauses when the sim is paused).
pub const ONE_SEC: &str = "1sec";
/// Fires every 4 seconds of sim time.
pub const FOUR_SEC: &str = "4sec";
/// Fires 6 times per second of sim time.
pub const SIX_HZ: &str = "6Hz";
/// Fires when the user aircraft's position changes. Documented as
/// deprecated in favor of polling position via a data definition.
pub const POSITION_CHANGED: &str = "PositionChanged";
/// Fires when a sound event (e.g. a sim alert chime) occurs.
pub const SOUND: &str = "Sound";
/// Fires when the user selects "Pause" from the menu — narrower than
/// [`PAUSE`], which also fires for Active Pause.
pub const PAUSED: &str = "Paused";
/// Counterpart to [`PAUSED`].
pub const UNPAUSED: &str = "Unpaused";
/// Fires once per simulated frame while paused (unlike [`FRAME`], which
/// stops firing when paused).
pub const PAUSE_FRAME: &str = "PauseFrame";
/// Fires when the user aircraft crashes.
pub const CRASHED: &str = "Crashed";
/// Fires when the crash-reset dialog is actioned.
pub const CRASH_RESET: &str = "CrashReset";
/// Fires when a flight (`.FLT`) file finishes loading.
pub const FLIGHT_LOADED: &str = "FlightLoaded";
/// Fires when a flight is saved.
pub const FLIGHT_SAVED: &str = "FlightSaved";
/// Fires when a flight plan (`.PLN`) is activated.
pub const FLIGHT_PLAN_ACTIVATED: &str = "FlightPlanActivated";
/// Fires when the active flight plan is deactivated.
pub const FLIGHT_PLAN_DEACTIVATED: &str = "FlightPlanDeactivated";
/// Fires when a new aircraft is loaded.
pub const AIRCRAFT_LOADED: &str = "AircraftLoaded";
/// Fires when the flight moves from "not running" (e.g. loading) to
/// running.
pub const SIM_START: &str = "SimStart";
/// Counterpart to [`SIM_START`].
pub const SIM_STOP: &str = "SimStop";
/// Fires when the user's view changes (e.g. cockpit to external).
pub const VIEW: &str = "View";
/// Fires when the weather mode changes (theme/live/custom).
pub const WEATHER_MODE_CHANGED: &str = "WeatherModeChanged";
/// Fires when an AI or user object is added to the simulation.
pub const OBJECT_ADDED: &str = "ObjectAdded";
/// Counterpart to [`OBJECT_ADDED`].
pub const OBJECT_REMOVED: &str = "ObjectRemoved";
/// Fires when a custom mission action is triggered.
pub const CUSTOM_MISSION_ACTION_EXECUTED: &str = "CustomMissionActionExecuted";
/// Fires when a multiplayer race ends.
pub const RACE_END: &str = "RaceEnd";
/// Fires when a multiplayer race lap completes.
pub const RACE_LAP: &str = "RaceLap";
