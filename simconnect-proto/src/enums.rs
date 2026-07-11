//! Enums shared by the send/recv packet layer.
//!
//! `RecvId` 0-26 covers the FSX-era message set, each value confirmed
//! against a live sim capture at some point (`Open`/`Exception`/`Event`/
//! `SimObjectData` explicitly, via the header-framing fix — see CLAUDE.md's
//! wire-format gotcha #0).
//!
//! 27+ (`EventEx1` onward) are MSFS2020/2024 additions, gated behind the
//! `kittyhawk`/`sunrise` features. **Their discriminant values were
//! originally guessed by declaration order and turned out to be wrong** — a
//! live MSFS2024 capture found `enumerate_controllers`'s reply arriving as
//! id `32` and `enumerate_input_events`'s as id `34` (not the guessed
//! 34/36), and `request_facility_data`'s reply as id `28`/`29` (not the
//! guessed 30/31), with no simple uniform offset relating old to new.
//! Every discriminant from `EventEx1` (27) through `CameraWorldLocker` (44)
//! is now ground-truthed rather than guessed or cross-referenced — it
//! exactly matches every value this crate had already live-confirmed
//! (`FacilityData`=28, `FacilityDataEnd`=29, `ControllersList`=32,
//! `EnumerateInputEvents`=34) and confirms there is no `Pick` variant at
//! all, contrary to this crate's original declaration-order guess.

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
    /// Supersedes this crate's own earlier declaration-order guess, which
    /// had assumed a `Pick` variant at 30; no such variant exists.
    #[cfg(feature = "kittyhawk")]
    EventEx1 = 27,
    /// Separately live-confirmed (`request_facility_data`'s reply).
    #[cfg(feature = "kittyhawk")]
    FacilityData = 28,
    /// Separately live-confirmed.
    #[cfg(feature = "kittyhawk")]
    FacilityDataEnd = 29,
    #[cfg(feature = "kittyhawk")]
    FacilityMinimalList = 30,
    #[cfg(feature = "kittyhawk")]
    JetwayData = 31,
    /// Separately live-confirmed (`enumerate_controllers`'s reply).
    #[cfg(feature = "kittyhawk")]
    ControllersList = 32,
    #[cfg(feature = "kittyhawk")]
    ActionCallback = 33,
    /// Separately live-confirmed (`enumerate_input_events`'s reply).
    #[cfg(feature = "kittyhawk")]
    EnumerateInputEvents = 34,
    /// No send builder/decode support yet — see GAPS.md.
    #[cfg(feature = "kittyhawk")]
    GetInputEvent = 35,
    #[cfg(feature = "kittyhawk")]
    SubscribeInputEvent = 36,
    /// No send builder/decode support yet — see GAPS.md.
    #[cfg(feature = "kittyhawk")]
    EnumerateInputEventParams = 37,
    /// MSFS2024 addition, ground-truthed alongside the rest of this range.
    /// No decode support yet.
    #[cfg(feature = "sunrise")]
    EnumerateSimobjectAndLiveryList = 38,
    /// `subscribe_to_flow_event`'s reply — no decode support yet, and as of
    /// this writing the send side itself is only documented in Microsoft's
    /// unreleased `/flighting/` (beta) doc tree, not the stable docs.
    #[cfg(feature = "sunrise")]
    FlowEvent = 39,
    /// Camera API. No send builder/decode support yet — see GAPS.md.
    #[cfg(feature = "sunrise")]
    CameraData = 40,
    #[cfg(feature = "sunrise")]
    CameraStatus = 41,
    #[cfg(feature = "sunrise")]
    CameraDefinitionList = 42,
    /// CommBus — WASM/JS-gauge ↔ SimConnect-client messaging. No send
    /// builder/decode support yet.
    #[cfg(feature = "sunrise")]
    CommBus = 43,
    #[cfg(feature = "sunrise")]
    CameraWorldLocker = 44,
}

impl RecvId {
    /// Recognizes every discriminant this crate declares: 0-26
    /// unconditionally (live-confirmed), and with `kittyhawk`/`sunrise`,
    /// 27-44 — all ground-truthed, not a guess (see this module's doc
    /// comment), so there's no risk of misdispatching an uncorrelated wire
    /// value the way there was when 27+ were only declaration-order
    /// guesses.
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0..=26 => Some(unsafe { core::mem::transmute::<u32, Self>(value) }),
            #[cfg(feature = "kittyhawk")]
            27 => Some(Self::EventEx1),
            #[cfg(feature = "kittyhawk")]
            28 => Some(Self::FacilityData),
            #[cfg(feature = "kittyhawk")]
            29 => Some(Self::FacilityDataEnd),
            #[cfg(feature = "kittyhawk")]
            30 => Some(Self::FacilityMinimalList),
            #[cfg(feature = "kittyhawk")]
            31 => Some(Self::JetwayData),
            #[cfg(feature = "kittyhawk")]
            32 => Some(Self::ControllersList),
            #[cfg(feature = "kittyhawk")]
            33 => Some(Self::ActionCallback),
            #[cfg(feature = "kittyhawk")]
            34 => Some(Self::EnumerateInputEvents),
            #[cfg(feature = "kittyhawk")]
            35 => Some(Self::GetInputEvent),
            #[cfg(feature = "kittyhawk")]
            36 => Some(Self::SubscribeInputEvent),
            #[cfg(feature = "kittyhawk")]
            37 => Some(Self::EnumerateInputEventParams),
            #[cfg(feature = "sunrise")]
            38 => Some(Self::EnumerateSimobjectAndLiveryList),
            #[cfg(feature = "sunrise")]
            39 => Some(Self::FlowEvent),
            #[cfg(feature = "sunrise")]
            40 => Some(Self::CameraData),
            #[cfg(feature = "sunrise")]
            41 => Some(Self::CameraStatus),
            #[cfg(feature = "sunrise")]
            42 => Some(Self::CameraDefinitionList),
            #[cfg(feature = "sunrise")]
            43 => Some(Self::CommBus),
            #[cfg(feature = "sunrise")]
            44 => Some(Self::CameraWorldLocker),
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

impl DataType {
    /// Fixed wire byte width of a field of this type — `None` for
    /// `Invalid`/`StringV`, which have no fixed width (`StringV` is
    /// variable-length and null-terminated, so it can't be placed at a
    /// static byte offset the way a ClientData definition requires).
    pub const fn byte_width(self) -> Option<u32> {
        match self {
            Self::Invalid | Self::StringV => None,
            Self::Int32 | Self::Float32 => Some(4),
            Self::Int64 | Self::Float64 => Some(8),
            Self::String8 => Some(8),
            Self::String32 => Some(32),
            Self::String64 => Some(64),
            Self::String128 => Some(128),
            Self::String256 => Some(256),
            Self::String260 => Some(260),
            // 6 f64 (lat/lon/alt/pitch/bank/heading) + 2 u32 (on_ground/airspeed).
            Self::InitPosition => Some(6 * 8 + 2 * 4),
            // 64-byte fixed name + u32 state.
            Self::MarkerState => Some(64 + 4),
            // lat, lon, alt (f64) + flags (u32) + speed, throttle (f64) —
            // see `crate::data::Waypoint`.
            Self::Waypoint => Some(3 * 8 + 4 + 2 * 8),
            Self::LatLonAlt => Some(3 * 8),
            Self::Xyz => Some(3 * 8),
        }
    }
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
    /// Ground-truthed live: this is what a real MSFS2024 instance actually
    /// returns for a malformed/rejected `request_jetway_data` call — not
    /// the generic `SizeMismatch` this crate's code once assumed.
    JetwayData,
    ActionNotFound,
    NotAnAction,
    IncorrectActionParams,
    GetInputEventFailed,
    SetInputEventFailed,
    EventNameReserved,
    Internal,
    CameraApi,
}

impl SimConnectException {
    pub fn from_u32(value: u32) -> Option<Self> {
        if value <= Self::CameraApi as u32 {
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
