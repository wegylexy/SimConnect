//! Enums shared by the send/recv packet layer.
//!
//! `RecvId` 0-26 covers the FSX-era message set. 28-36 (`Pick` through
//! `EnumerateInputEvents`) are MSFS2020 additions, gated behind the
//! `kittyhawk` feature; `Pick` (28) is included only as a recognizable tag
//! (no decode support yet — no confirmed struct layout). 27 is
//! intentionally absent from the C API's own numbering (there's no
//! `SIMCONNECT_RECV_ID` value 27 in the official enum, so this isn't a
//! mistake here). The official SDK's `SIMCONNECT_RECV_ID` has grown to
//! 39+ members as of MSFS 2020/2024; `GetInputEvent`, `SubscribeInputEvent`,
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
    #[cfg(feature = "kittyhawk")]
    Pick = 28,
    #[cfg(feature = "kittyhawk")]
    EventEx1,
    #[cfg(feature = "kittyhawk")]
    FacilityData,
    #[cfg(feature = "kittyhawk")]
    FacilityDataEnd,
    #[cfg(feature = "kittyhawk")]
    FacilityMinimalList,
    #[cfg(feature = "kittyhawk")]
    JetwayData,
    #[cfg(feature = "kittyhawk")]
    ControllersList,
    #[cfg(feature = "kittyhawk")]
    ActionCallback,
    #[cfg(feature = "kittyhawk")]
    EnumerateInputEvents,
}

impl RecvId {
    pub fn from_u32(value: u32) -> Option<Self> {
        #[cfg(feature = "kittyhawk")]
        const MAX: u32 = RecvId::EnumerateInputEvents as u32;
        #[cfg(not(feature = "kittyhawk"))]
        const MAX: u32 = RecvId::EventRaceLap as u32;

        // Value 27 has no assigned meaning in the official enum either, so
        // reject it explicitly rather than transmuting a gap.
        if value == 27 || value > MAX {
            return None;
        }
        // SAFETY: every value in `0..=EventRaceLap` and, with `kittyhawk`,
        // `28..=EnumerateInputEvents` is checked above to have a matching
        // `RecvId` variant at that exact discriminant.
        Some(unsafe { core::mem::transmute::<u32, Self>(value) })
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

/// `SIMCONNECT_GROUP_PRIORITY_*` constants.
pub mod group_priority {
    pub const HIGHEST: u32 = 1;
    pub const HIGHEST_MASKABLE: u32 = 10_000_000;
    pub const STANDARD: u32 = 1_900_000_000;
    pub const DEFAULT: u32 = 2_000_000_000;
    pub const LOWEST: u32 = 4_000_000_000;
}
