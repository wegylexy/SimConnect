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
    pub const UNSUBSCRIBE_FROM_SYSTEM_EVENT: u32 = 0xF0000018;
    /// Ground-truthed — not a guess or cross-reference. See
    /// `request_facilities_list`'s doc comment. Every other opcode in this
    /// file with a similar doc comment was confirmed the same way.
    pub const REQUEST_FACILITIES_LIST: u32 = 0xF0000043;
    pub const REQUEST_SYSTEM_STATE: u32 = 0xF0000035;
    pub const SET_SYSTEM_STATE: u32 = 0xF0000036;
    pub const MENU_ADD_ITEM: u32 = 0xF0000031;
    pub const CAMERA_SET_RELATIVE_6DOF: u32 = 0xF0000030;

    /// AI object creation/control (FSX-era). Cross-confirmed two
    /// independent ways: these values fall exactly in the `0x1C`-`0x2F`
    /// gap the prior C# client's `Sends.cs` left unimplemented between its
    /// last weather opcode (`0x1B`) and its first camera opcode (`0x30`),
    /// and match another from-scratch reimplementation's independently
    /// observed values for the same functions (consulted for the fact,
    /// not cited or copied — this project stays MIT).
    pub const AI_CREATE_PARKED_ATC_AIRCRAFT: u32 = 0xF0000027;
    pub const AI_CREATE_ENROUTE_ATC_AIRCRAFT: u32 = 0xF0000028;
    pub const AI_CREATE_NON_ATC_AIRCRAFT: u32 = 0xF0000029;
    pub const AI_CREATE_SIMULATED_OBJECT: u32 = 0xF000002A;
    pub const AI_RELEASE_CONTROL: u32 = 0xF000002B;
    pub const AI_REMOVE_OBJECT: u32 = 0xF000002C;
    pub const AI_SET_AIRCRAFT_FLIGHT_PLAN: u32 = 0xF000002D;

    /// MSFS2024 `_EX1` variants (add a `livery` parameter for modular
    /// SimObjects). Values cross-confirmed against an independent
    /// reimplementation, not copied code — this project is MIT and doesn't
    /// cite or attribute any LGPL-licensed source, even for facts.
    /// `AI_CREATE_SIMULATED_OBJECT_EX1` is live-confirmed against a real
    /// MSFS2024 instance: sending it with the user's own aircraft title and
    /// an empty livery string got back a genuine `RECV_ID::AssignedObjectId`
    /// (the request id echoed correctly), and the resulting object was
    /// successfully cleaned up with `ai_remove_object`.
    /// `AI_CREATE_ENROUTE_ATC_AIRCRAFT_EX1` is not yet live-tested.
    #[cfg(feature = "sunrise")]
    pub const AI_CREATE_ENROUTE_ATC_AIRCRAFT_EX1: u32 = 0xF0000058;
    #[cfg(feature = "sunrise")]
    pub const AI_CREATE_SIMULATED_OBJECT_EX1: u32 = 0xF000005A;

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
        pub const UNSUBSCRIBE_TO_FACILITIES: u32 = 0xF0000042;
        pub const REQUEST_FACILITIES_LIST_EX1: u32 = 0xF0000049;
        pub const REQUEST_ALL_FACILITIES: u32 = 0xF000005E;
        pub const MAP_INPUT_EVENT_TO_CLIENT_EVENT_EX1: u32 = 0xF000004D;
        pub const TRANSMIT_CLIENT_EVENT_EX1: u32 = 0xF0000044;
        pub const GET_INPUT_EVENT: u32 = 0xF0000050;
        pub const ENUMERATE_INPUT_EVENT_PARAMS: u32 = 0xF0000054;
    }

    /// MSFS2024 additions. Values ground-truthed, not guessed.
    #[cfg(feature = "sunrise")]
    pub mod sunrise {
        pub const SUBSCRIBE_TO_FLOW_EVENT: u32 = 0xF000005C;
        pub const CAMERA_ACQUIRE: u32 = 0xF000005F;
        pub const SUBSCRIBE_TO_COMM_BUS_EVENT: u32 = 0xF000006A;
        pub const CALL_COMM_BUS_EVENT: u32 = 0xF000006C;
        pub const ENUMERATE_SIMOBJECTS_AND_LIVERIES: u32 = 0xF000005B;
    }

    /// ClientData API (FSX-era — not gated behind `kittyhawk`). Values
    /// cross-confirmed by two independent reimplementations agreeing
    /// exactly, not copied code — this project is MIT and doesn't cite or
    /// attribute any LGPL-licensed source, even for facts. Not yet verified
    /// against a live sim capture by this crate.
    pub const MAP_CLIENT_DATA_NAME_TO_ID: u32 = 0xF0000037;
    pub const CREATE_CLIENT_DATA: u32 = 0xF0000038;
    pub const ADD_TO_CLIENT_DATA_DEFINITION: u32 = 0xF0000039;
    pub const CLEAR_CLIENT_DATA_DEFINITION: u32 = 0xF000003A;
    pub const REQUEST_CLIENT_DATA: u32 = 0xF000003B;
    pub const SET_CLIENT_DATA: u32 = 0xF000003C;
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

/// Maps a client-chosen name to a `client_data_id` the sim will recognize in
/// later `ClientData` calls. Layout cross-confirmed by two independent
/// reimplementations, and live-confirmed against a real MSFS2024 instance:
/// this send, paired with `create_client_data`/`add_to_client_data_definition`/
/// `request_client_data`, gets back a genuine `RECV_ID::ClientData` reply
/// (not an exception).
pub fn map_client_data_name_to_id(
    protocol_version: u32,
    client_data_name: &str,
    client_data_id: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::MAP_CLIENT_DATA_NAME_TO_ID, protocol_version);
    w.fixed_str(256, client_data_name)?;
    w.u32(client_data_id);
    Ok(w)
}

/// Creates (or re-opens) a ClientData area of `size` bytes. `read_only`
/// mirrors `SIMCONNECT_CREATE_CLIENT_DATA_FLAG_READ_ONLY`.
pub fn create_client_data(
    protocol_version: u32,
    client_data_id: u32,
    size: u32,
    read_only: bool,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::CREATE_CLIENT_DATA, protocol_version);
    w.u32(client_data_id).u32(size).u32(read_only as u32);
    w
}

/// Registers one field of a ClientData definition. `size_or_type` mirrors
/// the official `SimConnect_AddToClientDataDefinition`'s overloaded
/// `dwSizeOrType` parameter (either a byte count or a `SIMCONNECT_DATATYPE`
/// discriminant).
pub fn add_to_client_data_definition(
    protocol_version: u32,
    define_id: u32,
    offset: u32,
    size_or_type: u32,
    epsilon: f32,
    datum_id: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::ADD_TO_CLIENT_DATA_DEFINITION, protocol_version);
    w.u32(define_id)
        .u32(offset)
        .u32(size_or_type)
        .f32(epsilon)
        .u32(datum_id);
    w
}

pub fn clear_client_data_definition(protocol_version: u32, define_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::CLEAR_CLIENT_DATA_DEFINITION, protocol_version);
    w.u32(define_id);
    w
}

pub fn request_client_data(
    protocol_version: u32,
    client_data_id: u32,
    request_id: u32,
    define_id: u32,
    period: Period,
    flags: DataRequestFlags,
    origin: u32,
    interval: u32,
    limit: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::REQUEST_CLIENT_DATA, protocol_version);
    w.u32(client_data_id)
        .u32(request_id)
        .u32(define_id)
        .u32(period as u32)
        .u32(flags.bits())
        .u32(origin)
        .u32(interval)
        .u32(limit);
    w
}

/// `data` is already-encoded per the field layout registered with
/// `add_to_client_data_definition`, mirroring `set_data_on_sim_object`'s
/// `array_count`/`unit_size` convention (see its doc comment).
pub fn set_client_data(
    protocol_version: u32,
    client_data_id: u32,
    define_id: u32,
    array_count: u32,
    unit_size: u32,
    data: &[u8],
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::SET_CLIENT_DATA, protocol_version);
    w.u32(client_data_id)
        .u32(define_id)
        .u32(0) // reserved
        .u32(array_count)
        .u32(unit_size)
        .bytes(data);
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
///
/// `array_count`/`unit_size` mirror the official
/// `SimConnect_SetDataOnSimObject(..., DWORD ArrayCount, DWORD
/// cbUnitSize, ...)` signature directly — confirmed against a live
/// MSFS2024 capture, where getting this wrong (an earlier version of this
/// function computed a "count" from `data.len() / unit_size` and sent
/// `(count, unit_size)`, which cannot express the common single-value
/// case at all) made every write fail with `RECV_EXCEPTION::InvalidDataSize`.
/// For the common case of writing one non-array value/struct (the whole
/// registered data definition, once): `array_count = 0`, `unit_size =
/// data.len() as u32` (the *total* byte size of that one write, not a
/// per-field size). For an actual array write: `array_count` = number of
/// elements, `unit_size` = bytes per element, and `data.len() ==
/// array_count * unit_size`.
pub fn set_data_on_sim_object(
    protocol_version: u32,
    define_id: u32,
    object_id: u32,
    flags: DataSetFlags,
    array_count: u32,
    unit_size: u32,
    data: &[u8],
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::SET_DATA_ON_SIM_OBJECT, protocol_version);
    w.u32(define_id)
        .u32(object_id)
        .u32(flags.bits())
        .u32(array_count)
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

pub fn unsubscribe_from_system_event(protocol_version: u32, event_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::UNSUBSCRIBE_FROM_SYSTEM_EVENT, protocol_version);
    w.u32(event_id);
    w
}

/// Creates an AI-controlled aircraft that's currently parked with no
/// flight plan (`szContainerTitle`/`szTailNumber`/`szAirportID` widths —
/// 256/12/5 — per research into observed wire behavior). Follow up with
/// [`ai_set_aircraft_flight_plan`] to set it in motion.
pub fn ai_create_parked_atc_aircraft(
    protocol_version: u32,
    container_title: &str,
    tail_number: &str,
    airport_id: &str,
    request_id: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::AI_CREATE_PARKED_ATC_AIRCRAFT, protocol_version);
    w.fixed_str(256, container_title)?;
    w.fixed_str(12, tail_number)?;
    w.fixed_str(5, airport_id)?;
    w.u32(request_id);
    Ok(w)
}

/// Creates an AI-controlled aircraft already underway on a flight plan
/// (on the ground or airborne), typically flying under IFR in constant
/// radio contact with ATC.
pub fn ai_create_enroute_atc_aircraft(
    protocol_version: u32,
    container_title: &str,
    tail_number: &str,
    flight_number: i32,
    flight_plan_path: &str,
    flight_plan_position: f64,
    touch_and_go: bool,
    request_id: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::AI_CREATE_ENROUTE_ATC_AIRCRAFT, protocol_version);
    w.fixed_str(256, container_title)?;
    w.fixed_str(12, tail_number)?;
    w.i32(flight_number);
    w.fixed_str(260, flight_plan_path)?;
    w.f64(flight_plan_position);
    w.u32(touch_and_go as u32);
    w.u32(request_id);
    Ok(w)
}

/// Creates an aircraft not under ATC control (typically flying VFR) —
/// there's no internal AI pilot for helicopters, gliders, or balloons, so
/// this is also the entry point for those (use
/// [`ai_create_simulated_object`] instead for non-aircraft objects).
pub fn ai_create_non_atc_aircraft(
    protocol_version: u32,
    container_title: &str,
    tail_number: &str,
    init_position: &crate::data::InitPosition,
    request_id: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::AI_CREATE_NON_ATC_AIRCRAFT, protocol_version);
    w.fixed_str(256, container_title)?;
    w.fixed_str(12, tail_number)?;
    write_init_position(&mut w, init_position);
    w.u32(request_id);
    Ok(w)
}

/// Creates an AI-controlled object other than an aircraft (ground
/// vehicles, boats, and other simulation objects defined by a `sim.cfg`
/// container title) — can also create a stationary, unflyable aircraft.
pub fn ai_create_simulated_object(
    protocol_version: u32,
    container_title: &str,
    init_position: &crate::data::InitPosition,
    request_id: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::AI_CREATE_SIMULATED_OBJECT, protocol_version);
    w.fixed_str(256, container_title)?;
    write_init_position(&mut w, init_position);
    w.u32(request_id);
    Ok(w)
}

/// MSFS2024 `_EX1` variant of [`ai_create_simulated_object`], adding a
/// `livery` parameter (a modular SimObject's livery folder name). Layout
/// cross-confirmed against an independent reimplementation, and
/// live-confirmed against a real MSFS2024 instance (see
/// `opcode::AI_CREATE_SIMULATED_OBJECT_EX1`'s doc comment).
#[cfg(feature = "sunrise")]
pub fn ai_create_simulated_object_ex1(
    protocol_version: u32,
    container_title: &str,
    livery: &str,
    init_position: &crate::data::InitPosition,
    request_id: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::AI_CREATE_SIMULATED_OBJECT_EX1, protocol_version);
    w.fixed_str(256, container_title)?;
    w.fixed_str(256, livery)?;
    write_init_position(&mut w, init_position);
    w.u32(request_id);
    Ok(w)
}

/// MSFS2024 `_EX1` variant of [`ai_create_enroute_atc_aircraft`], adding a
/// `livery` parameter. Layout cross-confirmed against an independent
/// reimplementation; not yet verified against a live sim capture by this
/// crate.
#[cfg(feature = "sunrise")]
pub fn ai_create_enroute_atc_aircraft_ex1(
    protocol_version: u32,
    container_title: &str,
    livery: &str,
    tail_number: &str,
    flight_number: i32,
    flight_plan_path: &str,
    flight_plan_position: f64,
    touch_and_go: bool,
    request_id: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::AI_CREATE_ENROUTE_ATC_AIRCRAFT_EX1, protocol_version);
    w.fixed_str(256, container_title)?;
    w.fixed_str(256, livery)?;
    w.fixed_str(12, tail_number)?;
    w.i32(flight_number);
    w.fixed_str(260, flight_plan_path)?;
    w.f64(flight_plan_position);
    w.u32(touch_and_go as u32);
    w.u32(request_id);
    Ok(w)
}

fn write_init_position(w: &mut PacketWriter, pos: &crate::data::InitPosition) {
    w.f64(pos.latitude);
    w.f64(pos.longitude);
    w.f64(pos.altitude);
    w.f64(pos.pitch);
    w.f64(pos.bank);
    w.f64(pos.heading);
    w.u32(pos.on_ground as u32);
    w.u32(pos.airspeed);
}

/// Transfers control of an AI-created object (typically an aircraft) to
/// this SimConnect client — without this, the AI system and the client
/// may fight over control with unpredictable results. `object_id` is the
/// server-assigned id from the `RecvAssignedObjectId` reply to whichever
/// `ai_create_*` call created it.
pub fn ai_release_control(protocol_version: u32, object_id: u32, request_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::AI_RELEASE_CONTROL, protocol_version);
    w.u32(object_id).u32(request_id);
    w
}

/// Removes an AI-created object. A client can only remove objects it
/// created, not ones created by another client or by the sim itself.
pub fn ai_remove_object(protocol_version: u32, object_id: u32, request_id: u32) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::AI_REMOVE_OBJECT, protocol_version);
    w.u32(object_id).u32(request_id);
    w
}

/// Sets or changes an AI-controlled aircraft's flight plan — typically
/// called some time after [`ai_create_parked_atc_aircraft`] to set it in
/// motion. `flight_plan_path` is a `.pln` file path (extension optional;
/// a bare filename resolves against the default Flight Simulator Files
/// directory).
pub fn ai_set_aircraft_flight_plan(
    protocol_version: u32,
    object_id: u32,
    flight_plan_path: &str,
    request_id: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::AI_SET_AIRCRAFT_FLIGHT_PLAN, protocol_version);
    w.u32(object_id);
    w.fixed_str(260, flight_plan_path)?;
    w.u32(request_id);
    Ok(w)
}

// MSFS2020 additions. Field presence and order for the primary
// parameters (define/request IDs, ICAO/name strings, hashes) are
// confirmed against a working implementation observed during research;
// secondary parameter *semantics* noted below are inferred from that
// same research, not independently confirmed.

/// Registers one field of a facility data definition (parallel to
/// [`add_to_data_definition`] for sim objects, but for the modern facility
/// data API). **The `"OPEN <TYPE>"`/`"CLOSE <TYPE>"` bracketing (e.g.
/// `"OPEN AIRPORT"` ... `"LATITUDE"`/`"LONGITUDE"`/`"ALTITUDE"` ...
/// `"CLOSE AIRPORT"`) is required, not illustrative** — confirmed against
/// a live MSFS2024 capture: calling this with `"LATITUDE"` etc. directly,
/// without opening/closing an `"OPEN AIRPORT"`/`"CLOSE AIRPORT"` pair
/// around them, makes the later `request_facility_data` call fail with
/// `RECV_EXCEPTION::DataError`; bracketed, it correctly returns the
/// requested fields.
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
    let mut w = PacketWriter::new(opcode::kittyhawk::SUBSCRIBE_TO_FACILITIES, protocol_version);
    w.u32(facility_list_type).u32(request_id);
    w
}

/// Legacy FSX-era facility list request: gets back
/// `RecvId::AirportList`/`VorList`/`NdbList`/`WaypointList` depending on
/// `facility_list_type` (see `recv::parse_airport_list` etc.). Opcode and
/// layout ground-truthed: the payload is exactly
/// `facility_list_type`(`u32`) + `request_id`(`u32`), nothing else,
/// matching the official function's declared parameter order exactly. Not
/// gated behind `kittyhawk` — this is an FSX-era call (unlike its `_EX1`
/// sibling and `SubscribeToFacilities`, both MSFS2020 additions).
/// Live-confirmed against a real MSFS2024 instance: a call with
/// `facility_list_type = 0` (airports) returned a genuine `AirportList`
/// reply with plausible real airport data, decoded correctly.
pub fn request_facilities_list(
    protocol_version: u32,
    facility_list_type: u32,
    request_id: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::REQUEST_FACILITIES_LIST, protocol_version);
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
///
/// This crate's own six blind-guessed variants and a seventh re-derived
/// from an independent reimplementation all failed live against real
/// MSFS2024 with `RECV_EXCEPTION::SizeMismatch` — see GAPS.md. The real
/// layout was finally pinned down by ground-truth verification:
/// **20-byte icao** (every previous guess used 8/5/16/256), **no
/// `request_id` field at all**, then `ArrayCount`(`u32`) and that many
/// `i32` indices. Live-confirmed fully working against a real MSFS2024
/// instance from an actual parked gate: returned a genuine
/// `RecvId::JetwayData` reply (real entries, `airport_icao` echoed back
/// exactly), decoded correctly by this module's existing
/// `parse_jetway_data`.
#[cfg(feature = "kittyhawk")]
pub fn request_jetway_data(
    protocol_version: u32,
    airport_icao: &str,
    parking_indices: &[i32],
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::kittyhawk::REQUEST_JETWAY_DATA, protocol_version);
    w.fixed_str(20, airport_icao)?;
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

/// Ground-truthed: no payload at all beyond the packet header.
#[cfg(feature = "sunrise")]
pub fn subscribe_to_flow_event(protocol_version: u32) -> PacketWriter {
    PacketWriter::new(opcode::sunrise::SUBSCRIBE_TO_FLOW_EVENT, protocol_version)
}

/// Ground-truthed: `RequestID`(`u32`)
/// then `Hash`(`u64`), matching `SimConnect_GetInputEvent(HANDLE,
/// SIMCONNECT_DATA_REQUEST_ID RequestID, UINT64 Hash)`'s parameter order.
#[cfg(feature = "kittyhawk")]
pub fn get_input_event(protocol_version: u32, request_id: u32, hash: u64) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::kittyhawk::GET_INPUT_EVENT, protocol_version);
    w.u32(request_id).u64(hash);
    w
}

/// Ground-truthed: just `Hash`(`u64`), no
/// request id — matching `SimConnect_EnumerateInputEventParams(HANDLE,
/// UINT64 Hash)` exactly.
#[cfg(feature = "kittyhawk")]
pub fn enumerate_input_event_params(protocol_version: u32, hash: u64) -> PacketWriter {
    let mut w = PacketWriter::new(
        opcode::kittyhawk::ENUMERATE_INPUT_EVENT_PARAMS,
        protocol_version,
    );
    w.u64(hash);
    w
}

/// Ground-truthed: `RequestID`(`u32`)
/// then a 256-byte fixed string for `szState`.
pub fn request_system_state(
    protocol_version: u32,
    request_id: u32,
    state: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::REQUEST_SYSTEM_STATE, protocol_version);
    w.u32(request_id);
    w.fixed_str(256, state)?;
    Ok(w)
}

/// Ground-truthed: 256-byte `szState`,
/// then `dwInteger`(`u32`), `fFloat`(`f32`), then a 260-byte `szString` —
/// matching `SimConnect_SetSystemState`'s parameter order exactly, with the
/// two fixed string widths determined by total packet length (524-byte
/// payload only divides evenly as 256+4+4+260, not the more common 256
/// twice).
pub fn set_system_state(
    protocol_version: u32,
    state: &str,
    integer: u32,
    float: f32,
    string: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::SET_SYSTEM_STATE, protocol_version);
    w.fixed_str(256, state)?;
    w.u32(integer).f32(float);
    w.fixed_str(260, string)?;
    Ok(w)
}

/// Ground-truthed: just
/// `facility_list_type`(`u32`), the counterpart to `subscribe_to_facilities`.
#[cfg(feature = "kittyhawk")]
pub fn unsubscribe_to_facilities(protocol_version: u32, facility_list_type: u32) -> PacketWriter {
    let mut w = PacketWriter::new(
        opcode::kittyhawk::UNSUBSCRIBE_TO_FACILITIES,
        protocol_version,
    );
    w.u32(facility_list_type);
    w
}

/// Ground-truthed: `EventID`(`u32`) then
/// three 30-byte fixed strings for the key choices — a narrower width than
/// every other fixed string in this crate, confirmed exactly by total
/// packet length (94-byte payload = 4 + 3×30) and by the three key-choice
/// strings landing at offsets 30 bytes apart in the captured bytes.
pub fn request_reserved_key(
    protocol_version: u32,
    event_id: u32,
    key_choice_1: &str,
    key_choice_2: &str,
    key_choice_3: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::REQUEST_RESERVED_KEY, protocol_version);
    w.u32(event_id);
    w.fixed_str(30, key_choice_1)?;
    w.fixed_str(30, key_choice_2)?;
    w.fixed_str(30, key_choice_3)?;
    Ok(w)
}

/// Ground-truthed: `GroupID`(`u32`) then
/// a 256-byte fixed string for `szInputDefinition`.
pub fn remove_input_event(
    protocol_version: u32,
    group_id: u32,
    input_definition: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::REMOVE_INPUT_EVENT, protocol_version);
    w.u32(group_id);
    w.fixed_str(256, input_definition)?;
    Ok(w)
}

/// Ground-truthed: a single, unusually
/// wide 2048-byte fixed string for `ClientId` — confirmed by total packet
/// length (2048-byte payload for one string parameter).
#[cfg(feature = "sunrise")]
pub fn camera_acquire(
    protocol_version: u32,
    client_id: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::sunrise::CAMERA_ACQUIRE, protocol_version);
    w.fixed_str(2048, client_id)?;
    Ok(w)
}

/// Ground-truthed: six `f32`s in
/// declaration order (`fDeltaX/Y/Z`, `fPitchDeg`, `fBankDeg`,
/// `fHeadingDeg`) — every value in the captured packet matched the exact
/// floats this verification pass passed in.
pub fn camera_set_relative_6dof(
    protocol_version: u32,
    delta_x: f32,
    delta_y: f32,
    delta_z: f32,
    pitch_deg: f32,
    bank_deg: f32,
    heading_deg: f32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::CAMERA_SET_RELATIVE_6DOF, protocol_version);
    w.f32(delta_x)
        .f32(delta_y)
        .f32(delta_z)
        .f32(pitch_deg)
        .f32(bank_deg)
        .f32(heading_deg);
    w
}

/// Ground-truthed: `EventID`(`u32`) then
/// a 256-byte fixed string for `EventName`.
#[cfg(feature = "sunrise")]
pub fn subscribe_to_comm_bus_event(
    protocol_version: u32,
    event_id: u32,
    event_name: &str,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::sunrise::SUBSCRIBE_TO_COMM_BUS_EVENT, protocol_version);
    w.u32(event_id);
    w.fixed_str(256, event_name)?;
    Ok(w)
}

/// Ground-truthed: a 256-byte fixed
/// string for `EventName` (no length prefix, unlike every other string
/// this crate sends — this one just isn't length-tagged since it's a
/// fixed field), then `BroadcastTo`(`u32`), `BufferSize`(`u32`), then
/// `data` as a raw byte buffer whose length is exactly `BufferSize` — not
/// null-terminated or padded, confirmed by the captured packet ending
/// exactly at `data.len()` bytes past the `BufferSize` field.
#[cfg(feature = "sunrise")]
pub fn call_comm_bus_event(
    protocol_version: u32,
    event_name: &str,
    broadcast_to: u32,
    data: &[u8],
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::sunrise::CALL_COMM_BUS_EVENT, protocol_version);
    w.fixed_str(256, event_name)?;
    w.u32(broadcast_to).u32(data.len() as u32);
    w.bytes(data);
    Ok(w)
}

/// Ground-truthed: a 256-byte fixed
/// string for `szMenuItem`, then `MenuEventID`(`u32`) and `dwData`(`u32`).
pub fn menu_add_item(
    protocol_version: u32,
    menu_item: &str,
    menu_event_id: u32,
    data: u32,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(opcode::MENU_ADD_ITEM, protocol_version);
    w.fixed_str(256, menu_item)?;
    w.u32(menu_event_id).u32(data);
    Ok(w)
}

/// `_EX1` variant of [`request_facilities_list`]. Ground-truthed: identical
/// payload shape (`facility_list_type`(`u32`) + `request_id`(`u32`)) to the
/// non-`_EX1` call, just a different opcode.
#[cfg(feature = "kittyhawk")]
pub fn request_facilities_list_ex1(
    protocol_version: u32,
    facility_list_type: u32,
    request_id: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(
        opcode::kittyhawk::REQUEST_FACILITIES_LIST_EX1,
        protocol_version,
    );
    w.u32(facility_list_type).u32(request_id);
    w
}

/// Ground-truthed: same payload shape as
/// [`request_facilities_list`]/[`request_facilities_list_ex1`]
/// (`facility_list_type`(`u32`) + `request_id`(`u32`)), a different opcode
/// again.
#[cfg(feature = "kittyhawk")]
pub fn request_all_facilities(
    protocol_version: u32,
    facility_list_type: u32,
    request_id: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(opcode::kittyhawk::REQUEST_ALL_FACILITIES, protocol_version);
    w.u32(facility_list_type).u32(request_id);
    w
}

/// `_EX1` variant of [`map_input_event_to_client_event`]. Ground-truthed:
/// `GroupID`(`u32`), 256-byte `szInputDefinition`, then `DownEventID`/
/// `DownValue`/`UpEventID`/`UpValue`/`bMaskable` as five more `u32`s in
/// declaration order.
#[cfg(feature = "kittyhawk")]
pub fn map_input_event_to_client_event_ex1(
    protocol_version: u32,
    group_id: u32,
    input_definition: &str,
    down_event_id: u32,
    down_value: u32,
    up_event_id: u32,
    up_value: u32,
    maskable: bool,
) -> Result<PacketWriter, FixedStringError> {
    let mut w = PacketWriter::new(
        opcode::kittyhawk::MAP_INPUT_EVENT_TO_CLIENT_EVENT_EX1,
        protocol_version,
    );
    w.u32(group_id);
    w.fixed_str(256, input_definition)?;
    w.u32(down_event_id)
        .u32(down_value)
        .u32(up_event_id)
        .u32(up_value)
        .u32(maskable as u32);
    Ok(w)
}

/// `_EX1` variant of [`transmit_client_event`]. Ground-truthed: nine
/// `u32`s in declaration order (`ObjectID`, `EventID`, `GroupID`, `Flags`,
/// `dwData0`..`dwData4`).
#[cfg(feature = "kittyhawk")]
#[allow(clippy::too_many_arguments)]
pub fn transmit_client_event_ex1(
    protocol_version: u32,
    object_id: u32,
    event_id: u32,
    group_id: u32,
    flags: u32,
    data0: u32,
    data1: u32,
    data2: u32,
    data3: u32,
    data4: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(
        opcode::kittyhawk::TRANSMIT_CLIENT_EVENT_EX1,
        protocol_version,
    );
    w.u32(object_id)
        .u32(event_id)
        .u32(group_id)
        .u32(flags)
        .u32(data0)
        .u32(data1)
        .u32(data2)
        .u32(data3)
        .u32(data4);
    w
}

/// Ground-truthed: `RequestID`(`u32`)
/// then `Type`(`u32`) — note `RequestID` comes first here, the opposite
/// order from [`request_facilities_list`]'s `(type, request_id)`, matching
/// `SimConnect_EnumerateSimObjectsAndLiveries(HANDLE,
/// SIMCONNECT_DATA_REQUEST_ID RequestID, SIMCONNECT_SIMOBJECT_TYPE Type)`'s
/// declared parameter order exactly.
#[cfg(feature = "sunrise")]
pub fn enumerate_sim_objects_and_liveries(
    protocol_version: u32,
    request_id: u32,
    object_type: u32,
) -> PacketWriter {
    let mut w = PacketWriter::new(
        opcode::sunrise::ENUMERATE_SIMOBJECTS_AND_LIVERIES,
        protocol_version,
    );
    w.u32(request_id).u32(object_type);
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::PacketReader;
    use crate::data::InitPosition;

    fn header_id(packet: &[u8]) -> u32 {
        PacketReader::new(packet).outbound_header().unwrap().0
    }

    #[test]
    fn ai_create_parked_atc_aircraft_has_expected_opcode_and_layout() {
        let packet = ai_create_parked_atc_aircraft(4, "Boeing 747-8i Asobo", "N747BA", "KSEA", 42)
            .unwrap()
            .finish(1);
        assert_eq!(header_id(&packet), opcode::AI_CREATE_PARKED_ATC_AIRCRAFT);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.fixed_str(256).unwrap(), "Boeing 747-8i Asobo");
        assert_eq!(r.fixed_str(12).unwrap(), "N747BA");
        assert_eq!(r.fixed_str(5).unwrap(), "KSEA");
        assert_eq!(r.u32().unwrap(), 42);
    }

    #[test]
    fn ai_create_enroute_atc_aircraft_has_expected_opcode_and_layout() {
        let packet = ai_create_enroute_atc_aircraft(
            4,
            "Boeing 747-8i Asobo",
            "N747BA",
            123,
            "myflightplan",
            0.5,
            true,
            42,
        )
        .unwrap()
        .finish(1);
        assert_eq!(header_id(&packet), opcode::AI_CREATE_ENROUTE_ATC_AIRCRAFT);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.fixed_str(256).unwrap(), "Boeing 747-8i Asobo");
        assert_eq!(r.fixed_str(12).unwrap(), "N747BA");
        assert_eq!(r.i32().unwrap(), 123);
        assert_eq!(r.fixed_str(260).unwrap(), "myflightplan");
        assert_eq!(r.f64().unwrap(), 0.5);
        assert_eq!(r.u32().unwrap(), 1); // touch_and_go
        assert_eq!(r.u32().unwrap(), 42);
    }

    fn sample_init_position() -> InitPosition {
        InitPosition {
            latitude: 47.44,
            longitude: -122.30,
            altitude: 433.0,
            pitch: 0.0,
            bank: 0.0,
            heading: 270.0,
            on_ground: true,
            airspeed: 0,
        }
    }

    #[test]
    fn ai_create_non_atc_aircraft_has_expected_opcode_and_layout() {
        let pos = sample_init_position();
        let packet = ai_create_non_atc_aircraft(4, "Boeing 747-8i Asobo", "N747BA", &pos, 42)
            .unwrap()
            .finish(1);
        assert_eq!(header_id(&packet), opcode::AI_CREATE_NON_ATC_AIRCRAFT);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.fixed_str(256).unwrap(), "Boeing 747-8i Asobo");
        assert_eq!(r.fixed_str(12).unwrap(), "N747BA");
        assert_eq!(InitPosition::read_le(&mut r).unwrap(), pos);
        assert_eq!(r.u32().unwrap(), 42);
    }

    #[test]
    fn ai_create_simulated_object_has_expected_opcode_and_layout() {
        let pos = sample_init_position();
        let packet = ai_create_simulated_object(4, "Boeing 747-8i Asobo", &pos, 42)
            .unwrap()
            .finish(1);
        assert_eq!(header_id(&packet), opcode::AI_CREATE_SIMULATED_OBJECT);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.fixed_str(256).unwrap(), "Boeing 747-8i Asobo");
        assert_eq!(InitPosition::read_le(&mut r).unwrap(), pos);
        assert_eq!(r.u32().unwrap(), 42);
    }

    #[test]
    fn ai_release_control_has_expected_opcode_and_layout() {
        let packet = ai_release_control(4, 100, 42).finish(1);
        assert_eq!(header_id(&packet), opcode::AI_RELEASE_CONTROL);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 100);
        assert_eq!(r.u32().unwrap(), 42);
    }

    #[test]
    fn subscribe_to_flow_event_matches_ground_truth() {
        let packet = subscribe_to_flow_event(6).finish(2);
        assert_eq!(header_id(&packet), opcode::sunrise::SUBSCRIBE_TO_FLOW_EVENT);
        assert_eq!(packet.len(), 16); // header only, no payload
    }

    #[test]
    fn get_input_event_matches_ground_truth() {
        let packet = get_input_event(6, 1, 0x1234_5678_9abc_def0).finish(2);
        assert_eq!(header_id(&packet), opcode::kittyhawk::GET_INPUT_EVENT);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 1);
        assert_eq!(r.u64().unwrap(), 0x1234_5678_9abc_def0);
    }

    #[test]
    fn enumerate_input_event_params_matches_ground_truth() {
        let packet = enumerate_input_event_params(6, 0x1234_5678_9abc_def0).finish(2);
        assert_eq!(
            header_id(&packet),
            opcode::kittyhawk::ENUMERATE_INPUT_EVENT_PARAMS
        );
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u64().unwrap(), 0x1234_5678_9abc_def0);
        assert_eq!(packet.len(), 24); // header + one u64, no request id
    }

    #[test]
    fn request_system_state_matches_ground_truth() {
        let packet = request_system_state(6, 1, "AircraftLoaded").unwrap().finish(2);
        assert_eq!(header_id(&packet), opcode::REQUEST_SYSTEM_STATE);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 1);
        assert_eq!(r.fixed_str(256).unwrap(), "AircraftLoaded");
    }

    #[test]
    fn set_system_state_matches_ground_truth() {
        let packet = set_system_state(6, "Sound", 0x1111_2222, 3.5, "SysStateStr")
            .unwrap()
            .finish(2);
        assert_eq!(header_id(&packet), opcode::SET_SYSTEM_STATE);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.fixed_str(256).unwrap(), "Sound");
        assert_eq!(r.u32().unwrap(), 0x1111_2222);
        assert_eq!(r.f32().unwrap(), 3.5);
        assert_eq!(r.fixed_str(260).unwrap(), "SysStateStr");
    }

    #[test]
    fn unsubscribe_to_facilities_matches_ground_truth() {
        let packet = unsubscribe_to_facilities(6, 0).finish(2);
        assert_eq!(
            header_id(&packet),
            opcode::kittyhawk::UNSUBSCRIBE_TO_FACILITIES
        );
        assert_eq!(packet.len(), 20); // header + one u32
    }

    #[test]
    fn request_reserved_key_matches_ground_truth() {
        let packet = request_reserved_key(6, 1, "choiceA", "choiceBB", "choiceCCC")
            .unwrap()
            .finish(2);
        assert_eq!(header_id(&packet), opcode::REQUEST_RESERVED_KEY);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 1);
        assert_eq!(r.fixed_str(30).unwrap(), "choiceA");
        assert_eq!(r.fixed_str(30).unwrap(), "choiceBB");
        assert_eq!(r.fixed_str(30).unwrap(), "choiceCCC");
    }

    #[test]
    fn remove_input_event_matches_ground_truth() {
        let packet = remove_input_event(6, 1, "joystick:0:button:0")
            .unwrap()
            .finish(2);
        assert_eq!(header_id(&packet), opcode::REMOVE_INPUT_EVENT);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 1);
        assert_eq!(r.fixed_str(256).unwrap(), "joystick:0:button:0");
    }

    #[test]
    fn camera_acquire_matches_ground_truth() {
        let packet = camera_acquire(6, "probe").unwrap().finish(2);
        assert_eq!(header_id(&packet), opcode::sunrise::CAMERA_ACQUIRE);
        assert_eq!(packet.len(), 16 + 2048);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.fixed_str(2048).unwrap(), "probe");
    }

    #[test]
    fn camera_set_relative_6dof_matches_ground_truth() {
        let packet = camera_set_relative_6dof(6, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0).finish(2);
        assert_eq!(header_id(&packet), opcode::CAMERA_SET_RELATIVE_6DOF);
        assert_eq!(
            packet,
            vec![
                0x28, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0xf0, 0x02,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00,
                0x40, 0x40, 0x00, 0x00, 0x80, 0x40, 0x00, 0x00, 0xa0, 0x40, 0x00, 0x00, 0xc0,
                0x40,
            ]
        );
    }

    #[test]
    fn subscribe_to_comm_bus_event_matches_ground_truth() {
        let packet = subscribe_to_comm_bus_event(6, 1, "MyEvent").unwrap().finish(2);
        assert_eq!(
            header_id(&packet),
            opcode::sunrise::SUBSCRIBE_TO_COMM_BUS_EVENT
        );
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 1);
        assert_eq!(r.fixed_str(256).unwrap(), "MyEvent");
    }

    #[test]
    fn call_comm_bus_event_matches_ground_truth() {
        let packet = call_comm_bus_event(6, "MyBusEventName", 0x99, b"PayloadData")
            .unwrap()
            .finish(2);
        assert_eq!(header_id(&packet), opcode::sunrise::CALL_COMM_BUS_EVENT);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.fixed_str(256).unwrap(), "MyBusEventName");
        assert_eq!(r.u32().unwrap(), 0x99);
        assert_eq!(r.u32().unwrap(), 11);
        assert_eq!(r.rest(), b"PayloadData");
    }

    #[test]
    fn menu_add_item_matches_ground_truth() {
        let packet = menu_add_item(6, "MyMenuItem", 1, 0).unwrap().finish(2);
        assert_eq!(header_id(&packet), opcode::MENU_ADD_ITEM);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.fixed_str(256).unwrap(), "MyMenuItem");
        assert_eq!(r.u32().unwrap(), 1);
        assert_eq!(r.u32().unwrap(), 0);
    }

    #[test]
    fn request_facilities_list_ex1_matches_ground_truth() {
        let packet = request_facilities_list_ex1(6, 0, 1).finish(2);
        assert_eq!(
            header_id(&packet),
            opcode::kittyhawk::REQUEST_FACILITIES_LIST_EX1
        );
        assert_eq!(packet.len(), 24);
    }

    #[test]
    fn request_all_facilities_matches_ground_truth() {
        let packet = request_all_facilities(6, 0, 1).finish(2);
        assert_eq!(header_id(&packet), opcode::kittyhawk::REQUEST_ALL_FACILITIES);
        assert_eq!(packet.len(), 24);
    }

    #[test]
    fn map_input_event_to_client_event_ex1_matches_ground_truth() {
        let packet =
            map_input_event_to_client_event_ex1(6, 1, "joystick:0:button:0", 2, 0, 3, 0, false)
                .unwrap()
                .finish(2);
        assert_eq!(
            header_id(&packet),
            opcode::kittyhawk::MAP_INPUT_EVENT_TO_CLIENT_EVENT_EX1
        );
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 1);
        assert_eq!(r.fixed_str(256).unwrap(), "joystick:0:button:0");
        assert_eq!(r.u32().unwrap(), 2);
        assert_eq!(r.u32().unwrap(), 0);
        assert_eq!(r.u32().unwrap(), 3);
        assert_eq!(r.u32().unwrap(), 0);
        assert_eq!(r.u32().unwrap(), 0);
    }

    #[test]
    fn transmit_client_event_ex1_matches_ground_truth() {
        let packet = transmit_client_event_ex1(6, 0, 1, 2, 0, 10, 20, 30, 40, 50).finish(2);
        assert_eq!(
            header_id(&packet),
            opcode::kittyhawk::TRANSMIT_CLIENT_EVENT_EX1
        );
        assert_eq!(
            packet,
            vec![
                0x34, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x44, 0x00, 0x00, 0xf0, 0x02,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00,
                0x00, 0x1e, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn enumerate_sim_objects_and_liveries_matches_ground_truth() {
        let packet = enumerate_sim_objects_and_liveries(6, 1, 0).finish(2);
        assert_eq!(
            header_id(&packet),
            opcode::sunrise::ENUMERATE_SIMOBJECTS_AND_LIVERIES
        );
        assert_eq!(
            packet,
            vec![
                0x18, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x5b, 0x00, 0x00, 0xf0, 0x02,
                0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn request_jetway_data_matches_ground_truth() {
        // Exact ground-truthed bytes.
        let packet = request_jetway_data(6, "KSEA", &[]).unwrap().finish(2);
        assert_eq!(
            packet,
            vec![
                0x28, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x4b, 0x00, 0x00, 0xf0, 0x02,
                0x00, 0x00, 0x00, 0x4b, 0x53, 0x45, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00,
            ]
        );
    }

    #[test]
    fn request_facilities_list_matches_ground_truth() {
        // Exact ground-truthed bytes.
        let packet = request_facilities_list(6, 0, 1).finish(2);
        assert_eq!(
            packet,
            vec![
                0x18, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x43, 0x00, 0x00, 0xf0, 0x02,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn ai_remove_object_has_expected_opcode_and_layout() {
        let packet = ai_remove_object(4, 100, 42).finish(1);
        assert_eq!(header_id(&packet), opcode::AI_REMOVE_OBJECT);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 100);
        assert_eq!(r.u32().unwrap(), 42);
    }

    #[test]
    fn ai_set_aircraft_flight_plan_has_expected_opcode_and_layout() {
        let packet = ai_set_aircraft_flight_plan(4, 100, "myflightplan", 42)
            .unwrap()
            .finish(1);
        assert_eq!(header_id(&packet), opcode::AI_SET_AIRCRAFT_FLIGHT_PLAN);
        let mut r = PacketReader::new(&packet);
        r.outbound_header().unwrap();
        assert_eq!(r.u32().unwrap(), 100);
        assert_eq!(r.fixed_str(260).unwrap(), "myflightplan");
        assert_eq!(r.u32().unwrap(), 42);
    }
}
