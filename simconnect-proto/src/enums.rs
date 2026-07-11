//! Enums shared by the send/recv packet layer.
//!
//! `RecvId` 0-26 covers the FSX-era message set, each value confirmed
//! against a live sim capture at some point (`Open`/`Exception`/`Event`/
//! `SimObjectData` explicitly, via the header-framing fix — see CLAUDE.md's
//! wire-format gotcha #0). 27 is intentionally absent from the C API's own
//! numbering (there's no `SIMCONNECT_RECV_ID` value 27 in the official
//! enum, so this isn't a mistake here).
//!
//! 28+ (`Pick` onward) are MSFS2020 additions, gated behind the
//! `kittyhawk` feature. **Their discriminant values were originally
//! guessed by declaration order and turned out to be wrong** — a live
//! MSFS2024 capture found `enumerate_controllers`'s reply arriving as id
//! `32` and `enumerate_input_events`'s as id `34` (not the guessed 34/36),
//! and `request_facility_data`'s reply as id `28`/`29` (not the guessed
//! 30/31), with no simple uniform offset relating old to new. `FacilityData`,
//! `FacilityDataEnd`, `ControllersList`, and `EnumerateInputEvents` below
//! carry the corrected, live-confirmed values; `Pick`, `EventEx1`,
//! `FacilityMinimalList`, and `JetwayData` still carry unconfirmed
//! placeholder values (chosen only to avoid colliding with the confirmed
//! ones) and are excluded from [`RecvId::from_u32`] for that reason — see
//! GAPS.md. The official SDK's `SIMCONNECT_RECV_ID` has grown to 39+
//! members as of MSFS 2020/2024; `GetInputEvent`, `SubscribeInputEvent`,
//! `EnumerateInputEventParams`, and MSFS 2024's `FlowEvent` remain
//! unimplemented.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum RecvId {
    Null = 0,
    Exception,
    Open,
    Quit,
    Event,
    EventObjectAddRemove,
    EventFileName,
    EventFrame,
    SimObjectData,
    SimObjectDataByType,
    WeatherObservation,
    CloudState,
    AssignedObjectId,
    ReservedKey,
    CustomAction,
    SystemState,
    ClientData,
    EventWeatherMode,
    AirportList,
    VorList,
    NdbList,
    WaypointList,
    EventMultiplayerServerStarted,
    EventMultiplayerClientStarted,
    EventMultiplayerSessionEnded,
    EventRaceEnd,
    EventRaceLap,
    /// Live-confirmed at wire value 28 (`request_facility_data`'s reply).
    #[cfg(feature = "kittyhawk")]
    FacilityData = 28,
    /// Live-confirmed at wire value 29.
    #[cfg(feature = "kittyhawk")]
    FacilityDataEnd = 29,
    /// Unconfirmed placeholder — see this module's doc comment.
    #[cfg(feature = "kittyhawk")]
    Pick = 30,
    /// Unconfirmed placeholder — see this module's doc comment.
    #[cfg(feature = "kittyhawk")]
    EventEx1 = 31,
    /// Live-confirmed at wire value 32 (`enumerate_controllers`'s reply).
    #[cfg(feature = "kittyhawk")]
    ControllersList = 32,
    /// Unconfirmed placeholder — see this module's doc comment.
    #[cfg(feature = "kittyhawk")]
    JetwayData = 33,
    /// Live-confirmed at wire value 34 (`enumerate_input_events`'s reply).
    #[cfg(feature = "kittyhawk")]
    EnumerateInputEvents = 34,
    /// Unconfirmed placeholder — see this module's doc comment.
    #[cfg(feature = "kittyhawk")]
    ActionCallback = 35,
    /// Unconfirmed placeholder — see this module's doc comment.
    #[cfg(feature = "kittyhawk")]
    FacilityMinimalList = 36,
}

impl RecvId {
    /// Only recognizes discriminants this crate has actually confirmed
    /// against a live sim capture (0-26 unconditionally; with `kittyhawk`,
    /// also 28/29/32/34 — see this module's doc comment). Deliberately
    /// does *not* recognize `Pick`/`EventEx1`/`JetwayData`/
    /// `FacilityMinimalList`'s placeholder values: those haven't been
    /// confirmed, so treating their guessed numbers as recognized would
    /// let an uncorrelated wire value silently dispatch to the wrong
    /// decoder instead of falling through as unrecognized.
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0..=26 => Some(unsafe { core::mem::transmute::<u32, Self>(value) }),
            #[cfg(feature = "kittyhawk")]
            28 => Some(Self::FacilityData),
            #[cfg(feature = "kittyhawk")]
            29 => Some(Self::FacilityDataEnd),
            #[cfg(feature = "kittyhawk")]
            32 => Some(Self::ControllersList),
            #[cfg(feature = "kittyhawk")]
            34 => Some(Self::EnumerateInputEvents),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DataType {
    Invalid = 0,
    Int32,
    Int64,
    Float32,
    Float64,
    String8,
    String32,
    String64,
    String128,
    String256,
    String260,
    StringV,
    InitPosition,
    MarkerState,
    Waypoint,
    LatLonAlt,
    Xyz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SimConnectException {
    None = 0,
    Error,
    SizeMismatch,
    UnrecognizedId,
    Unopened,
    VersionMismatch,
    TooManyGroups,
    NameUnrecognized,
    TooManyEventNames,
    EventIdDuplicate,
    TooManyMaps,
    TooManyObjects,
    TooManyRequests,
    WeatherInvalidPort,
    WeatherInvalidMetar,
    WeatherUnableToGetObservation,
    WeatherUnableToCreateStation,
    WeatherUnableToRemoveStation,
    InvalidDataType,
    InvalidDataSize,
    DataError,
    InvalidArray,
    CreateObjectFailed,
    LoadFlightPlanFailed,
    OperationInvalidForObjectType,
    IllegalOperation,
    AlreadySubscribed,
    InvalidEnum,
    DefinitionError,
    DuplicateId,
    DatumId,
    OutOfBounds,
    AlreadyCreated,
    ObjectOutsideRealityBubble,
    ObjectContainer,
    ObjectAi,
    ObjectAtc,
    ObjectSchedule,
}

impl SimConnectException {
    pub fn from_u32(value: u32) -> Option<Self> {
        if value <= Self::ObjectSchedule as u32 {
            // SAFETY: `SimConnectException` is `#[repr(u32)]` and contiguous from 0.
            Some(unsafe { core::mem::transmute::<u32, Self>(value) })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SimObjectType {
    User = 0,
    All,
    Aircraft,
    Helicopter,
    Boat,
    Ground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum State {
    Off = 0,
    On,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Period {
    Never = 0,
    Once,
    VisualFrame,
    SimFrame,
    Second,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FacilityListType {
    Airport = 0,
    Waypoint,
    Ndb,
    Vor,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EventFlags: u32 {
        const DEFAULT = 0;
        const FAST_REPEAT_TIMER = 1;
        const SLOW_REPEAT_TIMER = 2;
        const GROUP_ID_IS_PRIORITY = 16;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DataRequestFlags: u32 {
        const DEFAULT = 0;
        const CHANGED = 1;
        const TAGGED = 2;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DataSetFlags: u32 {
        const DEFAULT = 0;
        const TAGGED = 1;
    }
}

#[cfg(feature = "kittyhawk")]
bitflags::bitflags! {
    /// Decodes the `dwData` payload of a `"Pause_EX1"` system event
    /// (`events::system::PAUSE_EX1`) — a bitmask replacing the older
    /// `"Pause"` event's plain 0/1, added because that couldn't
    /// distinguish e.g. the Esc-menu pause from other pause states.
    /// `SIM` and `ACTIVE` are each confirmed against a live MSFS2024
    /// capture: `Esc` (both at the ready-to-fly screen and in-flight)
    /// produced `dwData == 8`; toggling the in-sim play/pause icon
    /// produced `dwData == 4`; both alongside the legacy `"Pause"`
    /// event's `dwData == 1` firing at the same time. `LEGACY`/`FULL` are
    /// the documented values for the SDK's remaining `PAUSE_STATE_FLAG_*`
    /// constants but haven't individually been triggered and captured
    /// live — see GAPS.md.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PauseStateEx1: u32 {
        const OFF = 0;
        const LEGACY = 1;
        const FULL = 2;
        const ACTIVE = 4;
        const SIM = 8;
    }
}

/// `SIMCONNECT_GROUP_PRIORITY_*` constants.
pub mod group_priority {
    pub const HIGHEST: u32 = 1;
    pub const HIGHEST_MASKABLE: u32 = 10_000_000;
    pub const STANDARD: u32 = 1_900_000_000;
    pub const DEFAULT: u32 = 2_000_000_000;
    pub const LOWEST: u32 = 4_000_000_000;
}

#[cfg(all(test, feature = "kittyhawk"))]
mod tests {
    use super::PauseStateEx1;

    /// `dwData` values captured live from a running MSFS2024 instance —
    /// see `PauseStateEx1`'s doc comment.
    #[test]
    fn pause_state_ex1_decodes_live_captured_values() {
        assert_eq!(PauseStateEx1::from_bits(0), Some(PauseStateEx1::OFF));
        assert_eq!(PauseStateEx1::from_bits(8), Some(PauseStateEx1::SIM));
        assert_eq!(PauseStateEx1::from_bits(4), Some(PauseStateEx1::ACTIVE));
    }
}
