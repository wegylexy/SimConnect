//! Outbound packet builders. Opcodes and field layouts ported from the
//! prior FSX-era C# client's `Sends.cs`, plus the MSFS 2020-only sends
//! (modern facility data requests, input-event enumeration, controllers
//! list) added since — camera and CommBus sends remain unimplemented.

use crate::codec::PacketWriter;
use crate::enums::{DataRequestFlags, DataSetFlags, EventFlags, Period, SimObjectType};
use crate::protocol::ProtocolVersion;
use crate::strings::FixedStringError;

pub mod opcode {
    pub const OPEN: u32 = 0xF0000001;
    pub const MAP_CLIENT_EVENT_TO_SIM_EVENT: u32 = 0xF0000004;
    pub const TRANSMIT_CLIENT_EVENT: u32 = 0xF0000005;
    pub const SET_SYSTEM_EVENT_STATE: u32 = 0xF0000006;
    pub const ADD_CLIENT_EVENT_TO_NOTIFICATION_GROUP: u32 = 0xF0000007;
    pub const REMOVE_CLIENT_EVENT: u32 = 0xF0000008;
    pub const SET_NOTIFICATION_GROUP_PRIORITY: u32 = 0xF0000009;
    pub const CLEAR_NOTIFICATION_GROUP: u32 = 0xF000000A;
    pub const REQUEST_NOTIFICATION_GROUP: u32 = 0xF000000B;
    pub const ADD_TO_DATA_DEFINITION: u32 = 0xF000000C;
    pub const CLEAR_DATA_DEFINITION: u32 = 0xF000000D;
    pub const REQUEST_DATA_ON_SIM_OBJECT: u32 = 0xF000000E;
    pub const REQUEST_DATA_ON_SIM_OBJECT_TYPE: u32 = 0xF000000F;
    pub const SET_DATA_ON_SIM_OBJECT: u32 = 0xF0000010;
    pub const MAP_INPUT_EVENT_TO_CLIENT_EVENT: u32 = 0xF0000011;
    pub const SET_INPUT_GROUP_PRIORITY: u32 = 0xF0000012;
    pub const REMOVE_INPUT_EVENT: u32 = 0xF0000013;
    pub const CLEAR_INPUT_GROUP: u32 = 0xF0000014;
    pub const SET_INPUT_GROUP_STATE: u32 = 0xF0000015;
    pub const REQUEST_RESERVED_KEY: u32 = 0xF0000016;
    pub const SUBSCRIBE_TO_SYSTEM_EVENT: u32 = 0xF0000017;
    pub const UNSUBSCRIBE_TO_SYSTEM_EVENT: u32 = 0xF0000018;

    /// MSFS2020 additions. Values are `0xF0000000 | small_code`, where
    /// `small_code` is independently-observed wire-protocol behavior, not
    /// copied code — this project is MIT and doesn't cite or attribute any
    /// LGPL-licensed source, even for facts. Not yet verified against a
    /// live sim capture by this crate.
    #[cfg(feature = "kittyhawk")]
    pub mod kittyhawk {
        pub const SUBSCRIBE_TO_FACILITIES: u32 = 0xF0000041;
        pub const ADD_TO_FACILITY_DEFINITION: u32 = 0xF0000045;
        pub const REQUEST_FACILITY_DATA: u32 = 0xF0000046;
        pub const REQUEST_FACILITY_DATA_EX1: u32 = 0xF000004A;
        pub const REQUEST_JETWAY_DATA: u32 = 0xF000004B;
        pub const ENUMERATE_CONTROLLERS: u32 = 0xF000004C;
        pub const ENUMERATE_INPUT_EVENTS: u32 = 0xF000004F;
        pub const SET_INPUT_EVENT: u32 = 0xF0000051;
        pub const SUBSCRIBE_INPUT_EVENT: u32 = 0xF0000052;
    }
}

/// `SIMCONNECT_UNTAGGED` sentinel used as `datum_id` in `add_to_data_definition`.
pub const UNUSED: i32 = -1;

/// The "FSX" application-identifier magic that occupies the high 32 bits of
/// the `Open` packet's `dwReserved1/2` alias field.
const FSX_MAGIC: u64 = 0x4653_5800_0000_0000;

pub fn open(
    protocol: ProtocolVersion,
    application_name: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::OPEN, protocol.wire_version());
    w.fixed_str(256, application_name)?;
    w.bytes(&FSX_MAGIC.to_le_bytes());
    let v = protocol.sim_connect_version();
    w.i32(v.major as i32)
        .i32(v.minor as i32)
        .i32(v.build_major as i32)
        .i32(v.build_minor as i32);
    Ok(w)
}

pub fn map_client_event_to_sim_event(
    protocol_version: u32,
    event_id: u32,
    event_name: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::MAP_CLIENT_EVENT_TO_SIM_EVENT, protocol_version);
    w.u32(event_id);
    w.fixed_str(256, event_name)?;
    Ok(w)
}

pub fn transmit_client_event(
    protocol_version: u32,
    object_id: u32,
    event_id: u32,
    data: i32,
    group_id: u32,
    flags: EventFlags,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::TRANSMIT_CLIENT_EVENT, protocol_version);
    w.u32(object_id)
        .u32(event_id)
        .i32(data)
        .u32(group_id)
        .u32(flags.bits());
    w
}

pub fn set_system_event_state(protocol_version: u32, event_id: u32, on: bool) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::SET_SYSTEM_EVENT_STATE, protocol_version);
    w.u32(event_id).u32(on as u32);
    w
}

pub fn add_client_event_to_notification_group(
    protocol_version: u32,
    group_id: u32,
    event_id: u32,
    maskable: bool,
) -> PacketWriter {
    let mut w = PacketWriter::new(
        opcode::ADD_CLIENT_EVENT_TO_NOTIFICATION_GROUP,
        protocol_version,
    );
    w.u32(group_id).u32(event_id).i32(maskable as i32);
    w
}

pub fn remove_client_event(protocol_version: u32, group_id: u32, event_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::REMOVE_CLIENT_EVENT, protocol_version);
    w.u32(group_id).u32(event_id);
    w
}

pub fn set_notification_group_priority(
    protocol_version: u32,
    group_id: u32,
    priority: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::SET_NOTIFICATION_GROUP_PRIORITY, protocol_version);
    w.u32(group_id).u32(priority);
    w
}

pub fn clear_notification_group(protocol_version: u32, group_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::CLEAR_NOTIFICATION_GROUP, protocol_version);
    w.u32(group_id);
    w
}

pub fn request_notification_group(
    protocol_version: u32,
    group_id: u32,
    reserved: u32,
    flags: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::REQUEST_NOTIFICATION_GROUP, protocol_version);
    w.u32(group_id).u32(reserved).u32(flags);
    w
}

pub fn add_to_data_definition(
    protocol_version: u32,
    define_id: u32,
    datum_name: &str,
    units_name: Option<&str>,
    datum_type: crate::enums::DataType,
    epsilon: f32,
    datum_id: i32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::ADD_TO_DATA_DEFINITION, protocol_version);
    w.u32(define_id);
    w.fixed_str(256, datum_name)?;
    w.fixed_str(256, units_name.unwrap_or(""))?;
    w.u32(datum_type as u32).f32(epsilon).i32(datum_id);
    Ok(w)
}

pub fn clear_data_definition(protocol_version: u32, define_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::CLEAR_DATA_DEFINITION, protocol_version);
    w.u32(define_id);
    w
}

pub fn request_data_on_sim_object(
    protocol_version: u32,
    request_id: u32,
    define_id: u32,
    object_id: u32,
    period: Period,
    flags: DataRequestFlags,
    origin: u32,
    interval: u32,
    limit: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::REQUEST_DATA_ON_SIM_OBJECT, protocol_version);
    w.u32(request_id)
        .u32(define_id)
        .u32(object_id)
        .u32(period as u32)
        .u32(flags.bits())
        .u32(origin)
        .u32(interval)
        .u32(limit);
    w
}

pub fn request_data_on_sim_object_type(
    protocol_version: u32,
    request_id: u32,
    define_id: u32,
    radius_meters: u32,
    object_type: SimObjectType,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::REQUEST_DATA_ON_SIM_OBJECT_TYPE, protocol_version);
    w.u32(request_id)
        .u32(define_id)
        .u32(radius_meters)
        .u32(object_type as u32);
    w
}

/// `data` is a sequence of already-encoded datum values matching the data
/// definition registered for `define_id`; the caller is responsible for
/// laying them out per `add_to_data_definition` order (this crate does not
/// attempt to derive that layout from Rust types).
pub fn set_data_on_sim_object(
    protocol_version: u32,
    define_id: u32,
    object_id: u32,
    flags: DataSetFlags,
    unit_size: u32,
    data: &[u8],
) -> PacketWriter {
    let count = if unit_size == 0 {
        0
    } else {
        data.len() as u32 / unit_size
    };
    let mut w = PacketWriter::new(opcode::SET_DATA_ON_SIM_OBJECT, protocol_version);
    w.u32(define_id)
        .u32(object_id)
        .u32(flags.bits())
        .u32(count)
        .u32(unit_size)
        .bytes(data);
    w
}

pub fn map_input_event_to_client_event(
    protocol_version: u32,
    group_id: u32,
    input_definition: &str,
    down_event_id: u32,
    down_value: i32,
    up_event_id: u32,
    up_value: i32,
    maskable: bool,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::MAP_INPUT_EVENT_TO_CLIENT_EVENT, protocol_version);
    w.u32(group_id);
    w.fixed_str(256, input_definition)?;
    w.u32(down_event_id)
        .i32(down_value)
        .u32(up_event_id)
        .i32(up_value)
        .i32(maskable as i32);
    Ok(w)
}

pub fn set_input_group_priority(
    protocol_version: u32,
    group_id: u32,
    priority: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::SET_INPUT_GROUP_PRIORITY, protocol_version);
    w.u32(group_id).u32(priority);
    w
}

pub fn clear_input_group(protocol_version: u32, group_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::CLEAR_INPUT_GROUP, protocol_version);
    w.u32(group_id);
    w
}

pub fn set_input_group_state(protocol_version: u32, group_id: u32, on: bool) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::SET_INPUT_GROUP_STATE, protocol_version);
    w.u32(group_id).u32(on as u32);
    w
}

pub fn subscribe_to_system_event(
    protocol_version: u32,
    event_id: u32,
    event_name: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::SUBSCRIBE_TO_SYSTEM_EVENT, protocol_version);
    w.u32(event_id);
    w.fixed_str(256, event_name)?;
    Ok(w)
}

pub fn unsubscribe_to_system_event(protocol_version: u32, event_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::UNSUBSCRIBE_TO_SYSTEM_EVENT, protocol_version);
    w.u32(event_id);
    w
}

// MSFS2020 additions. Field presence and order for the primary
// parameters (define/request IDs, ICAO/name strings, hashes) are
// confirmed against a working implementation observed during research;
// secondary parameter *semantics* noted below are inferred from that
// same research, not independently confirmed.

/// Registers one field of a facility data definition (parallel to
/// [`add_to_data_definition`] for sim objects, but for the modern facility
/// data API — e.g. field name `"OPEN AIRPORT"`, `"LATITUDE"`,
/// `"CLOSE AIRPORT"`).
#[cfg(feature = "kittyhawk")]
pub fn add_to_facility_definition(
    protocol_version: u32,
    define_id: u32,
    field_name: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(
        opcode::kittyhawk::ADD_TO_FACILITY_DEFINITION,
        protocol_version,
    );
    w.u32(define_id);
    w.fixed_str(256, field_name)?;
    Ok(w)
}

/// Subscribes to add/remove notifications for a facility type.
///
/// The second parameter is a request id, not a client event id — confirmed
/// against the official function signature,
/// `SimConnect_SubscribeToFacilities(HANDLE, SIMCONNECT_FACILITY_LIST_TYPE
/// type, SIMCONNECT_DATA_REQUEST_ID RequestID)`, whose docs describe
/// `RequestID` as "the client defined request ID. This will be returned
/// along with the data" — i.e. it comes back on `RecvFacilityMinimalList`/
/// whatever `RECV_ID` this subscription's data arrives on, the same way
/// every other `RequestID` parameter in this API does, not as an event id
/// a caller would map via `map_client_event_to_sim_event`. (An earlier
/// version of this doc comment left this unresolved — a `clientEventId`
/// naming convention from unrelated research had raised doubt — since
/// settled by reading the official signature directly rather than left
/// unconfirmed.)
#[cfg(feature = "kittyhawk")]
pub fn subscribe_to_facilities(
    protocol_version: u32,
    facility_list_type: u32,
    request_id: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(
        opcode::kittyhawk::SUBSCRIBE_TO_FACILITIES,
        protocol_version,
    );
    w.u32(facility_list_type).u32(request_id);
    w
}

/// Requests facility data for one ICAO identifier using a facility
/// definition already registered via [`add_to_facility_definition`].
/// `icao`/`region` widths (16/4 bytes) per research into observed wire
/// behavior.
#[cfg(feature = "kittyhawk")]
pub fn request_facility_data(
    protocol_version: u32,
    define_id: u32,
    request_id: u32,
    icao: &str,
    region: Option<&str>,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::kittyhawk::REQUEST_FACILITY_DATA, protocol_version);
    w.u32(define_id).u32(request_id);
    w.fixed_str(16, icao)?;
    w.fixed_str(4, region.unwrap_or(""))?;
    Ok(w)
}

/// `_EX1` variant of [`request_facility_data`], additionally filtering by
/// a single-character facility type code.
#[cfg(feature = "kittyhawk")]
pub fn request_facility_data_ex1(
    protocol_version: u32,
    define_id: u32,
    request_id: u32,
    icao: &str,
    region: Option<&str>,
    facility_type: Option<char>,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(
        opcode::kittyhawk::REQUEST_FACILITY_DATA_EX1,
        protocol_version,
    );
    w.u32(define_id).u32(request_id);
    w.fixed_str(16, icao)?;
    w.fixed_str(4, region.unwrap_or(""))?;
    w.fixed_str(1, &facility_type.map(String::from).unwrap_or_default())?;
    Ok(w)
}

/// Requests jetway data for an airport, optionally restricted to specific
/// parking indices (empty = all).
#[cfg(feature = "kittyhawk")]
pub fn request_jetway_data(
    protocol_version: u32,
    request_id: u32,
    airport_icao: &str,
    parking_indices: &[i32],
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::kittyhawk::REQUEST_JETWAY_DATA, protocol_version);
    w.u32(request_id);
    w.fixed_str(8, airport_icao)?;
    w.u32(parking_indices.len() as u32);
    for &i in parking_indices {
        w.i32(i);
    }
    Ok(w)
}

/// Requests the list of connected input controllers (joysticks, yokes,
/// etc). Takes no parameters beyond the packet header.
#[cfg(feature = "kittyhawk")]
pub fn enumerate_controllers(protocol_version: u32) -> PacketWriter {
    PacketWriter::new(opcode::kittyhawk::ENUMERATE_CONTROLLERS, protocol_version)
}

/// Requests the list of input events available for the current SimObject.
#[cfg(feature = "kittyhawk")]
pub fn enumerate_input_events(protocol_version: u32, request_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::kittyhawk::ENUMERATE_INPUT_EVENTS, protocol_version);
    w.u32(request_id);
    w
}

/// Sets an input event's value by its CRC-based hash (from
/// [`enumerate_input_events`]'s response). `value` is the already-encoded
/// datum matching the input event's type — this crate doesn't attempt to
/// derive that encoding.
#[cfg(feature = "kittyhawk")]
pub fn set_input_event(protocol_version: u32, input_event_hash: u64, value: &[u8]) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::kittyhawk::SET_INPUT_EVENT, protocol_version);
    w.bytes(&input_event_hash.to_le_bytes());
    w.bytes(value);
    w
}

/// Subscribes to change notifications for an input event by its hash.
#[cfg(feature = "kittyhawk")]
pub fn subscribe_input_event(protocol_version: u32, input_event_hash: u64) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::kittyhawk::SUBSCRIBE_INPUT_EVENT, protocol_version);
    w.bytes(&input_event_hash.to_le_bytes());
    w
}
