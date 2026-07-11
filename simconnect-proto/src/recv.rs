//! Inbound packet parsing, keyed by `RecvId` (`Recvs.cs` in the prior C#
//! client). Each `parse_*` function expects the reader positioned just past
//! the common `Recv` header (already consumed via `PacketReader::header`).

use crate::codec::{PacketReader, TooShort};
use crate::enums::SimConnectException;
use crate::protocol::SimConnectVersion;

#[derive(Debug, Clone, PartialEq)]
pub struct RecvException {
    pub exception: Option<SimConnectException>,
    pub send_id: u32,
    pub index: u32,
}

impl RecvException {
    /// Whether this exception was raised in response to the packet sent
    /// with the given `send_id` (the value returned by whichever
    /// `SimConnect` method issued it). SimConnect never acknowledges
    /// success explicitly — this is only ever a NACK, arriving later and
    /// out of band from any particular call — so there's no matching
    /// "this send succeeded" signal to check for; this just makes the
    /// caller's own `header.id == RecvId::Exception` + `send_id` match in
    /// their `recv`/`recv_ref` loop a single call instead of two.
    pub fn matches(&self, send_id: u32) -> bool {
        self.send_id == send_id
    }
}

pub fn parse_exception(r: &mut PacketReader) -> Result<RecvException, TooShort> {
    let exception = SimConnectException::from_u32(r.u32()?);
    Ok(RecvException {
        exception,
        send_id: r.u32()?,
        index: r.u32()?,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecvOpen {
    pub application_name: String,
    pub application_version: SimConnectVersion,
    pub sim_connect_version: SimConnectVersion,
}

pub fn parse_open(r: &mut PacketReader) -> Result<RecvOpen, TooShort> {
    let application_name = r.fixed_str(256)?;
    let application_version = SimConnectVersion {
        major: r.i32()? as u32,
        minor: r.i32()? as u32,
        build_major: r.i32()? as u32,
        build_minor: r.i32()? as u32,
    };
    let sim_connect_version = SimConnectVersion {
        major: r.i32()? as u32,
        minor: r.i32()? as u32,
        build_major: r.i32()? as u32,
        build_minor: r.i32()? as u32,
    };
    Ok(RecvOpen {
        application_name,
        application_version,
        sim_connect_version,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RecvEvent {
    pub group_id: u32,
    pub event_id: u32,
    pub data: i32,
}

pub fn parse_event(r: &mut PacketReader) -> Result<RecvEvent, TooShort> {
    Ok(RecvEvent {
        group_id: r.u32()?,
        event_id: r.u32()?,
        data: r.i32()?,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RecvEventFrame {
    pub event: RecvEvent,
    pub frame_rate: f32,
    pub sim_speed: f32,
}

pub fn parse_event_frame(r: &mut PacketReader) -> Result<RecvEventFrame, TooShort> {
    let event = parse_event(r)?;
    Ok(RecvEventFrame {
        event,
        frame_rate: r.f32()?,
        sim_speed: r.f32()?,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecvSimObjectData<'a> {
    pub request_id: u32,
    pub object_id: u32,
    pub define_id: u32,
    pub flags: u32,
    pub entry_number: u32,
    pub out_of: u32,
    pub define_count: u32,
    /// Raw, still-encoded datum values in the order the data definition was
    /// built; the caller decodes each field per its registered `DataType`.
    pub data: &'a [u8],
}

pub fn parse_sim_object_data<'a>(
    r: &mut PacketReader<'a>,
) -> Result<RecvSimObjectData<'a>, TooShort> {
    Ok(RecvSimObjectData {
        request_id: r.u32()?,
        object_id: r.u32()?,
        define_id: r.u32()?,
        flags: r.u32()?,
        entry_number: r.u32()?,
        out_of: r.u32()?,
        define_count: r.u32()?,
        data: r.rest(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RecvAssignedObjectId {
    pub request_id: u32,
    pub object_id: u32,
}

pub fn parse_assigned_object_id(r: &mut PacketReader) -> Result<RecvAssignedObjectId, TooShort> {
    Ok(RecvAssignedObjectId {
        request_id: r.u32()?,
        object_id: r.u32()?,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecvSystemState {
    pub request_id: u32,
    pub integer: u32,
    pub float: f32,
    pub string: String,
}

pub fn parse_system_state(r: &mut PacketReader) -> Result<RecvSystemState, TooShort> {
    Ok(RecvSystemState {
        request_id: r.u32()?,
        integer: r.u32()?,
        float: r.f32()?,
        string: r.fixed_str(256)?,
    })
}

// MSFS2020 additions. Struct field lists are confirmed against
// `flybywiresim/msfs-rs`'s bindgen dump of the real `SimConnect.h` (all
// `#[repr(C, packed(1))]`, matching our byte-cursor approach). Not yet
// verified against a live sim capture by this crate.
#[cfg(feature = "kittyhawk")]
pub mod kittyhawk {
    use super::*;

    /// Common prefix shared by every list-style recv
    /// (`SIMCONNECT_RECV_LIST_TEMPLATE`): request id, total array size,
    /// and this message's entry-number/out-of pagination position.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ListTemplate {
        pub request_id: u32,
        pub array_size: u32,
        pub entry_number: u32,
        pub out_of: u32,
    }

    pub fn parse_list_template(r: &mut PacketReader) -> Result<ListTemplate, TooShort> {
        Ok(ListTemplate {
            request_id: r.u32()?,
            array_size: r.u32()?,
            entry_number: r.u32()?,
            out_of: r.u32()?,
        })
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct RecvEventEx1 {
        pub group_id: u32,
        pub event_id: u32,
        pub data: [u32; 5],
    }

    pub fn parse_event_ex1(r: &mut PacketReader) -> Result<RecvEventEx1, TooShort> {
        Ok(RecvEventEx1 {
            group_id: r.u32()?,
            event_id: r.u32()?,
            data: [r.u32()?, r.u32()?, r.u32()?, r.u32()?, r.u32()?],
        })
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RecvFacilityData<'a> {
        pub user_request_id: u32,
        pub unique_request_id: u32,
        /// Non-zero when this message is about a child of another
        /// facility data request; matches that parent's `unique_request_id`.
        pub parent_unique_request_id: u32,
        /// `SIMCONNECT_FACILITY_DATA_TYPE` — kept as a raw discriminant
        /// rather than a typed enum since this crate hasn't confirmed the
        /// enum's actual values yet (only the struct's field layout).
        pub data_type: u32,
        pub is_list_item: bool,
        pub item_index: u32,
        pub list_size: u32,
        /// Raw bytes; caller decodes per `data_type` and whatever facility
        /// definition was registered via `send::add_to_facility_definition`.
        pub data: &'a [u8],
    }

    pub fn parse_facility_data<'a>(
        r: &mut PacketReader<'a>,
    ) -> Result<RecvFacilityData<'a>, TooShort> {
        Ok(RecvFacilityData {
            user_request_id: r.u32()?,
            unique_request_id: r.u32()?,
            parent_unique_request_id: r.u32()?,
            data_type: r.u32()?,
            is_list_item: r.bool32()?,
            item_index: r.u32()?,
            list_size: r.u32()?,
            data: r.rest(),
        })
    }

    /// Marks the end of a `parse_facility_data` sequence for one request.
    /// Exact field list beyond the two request ids is unconfirmed — this
    /// only reads what's documented, and doesn't assert the packet's exact
    /// total size, so any additional trailing fields are silently ignored
    /// rather than misparsed.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct RecvFacilityDataEnd {
        pub user_request_id: u32,
        pub unique_request_id: u32,
    }

    pub fn parse_facility_data_end(r: &mut PacketReader) -> Result<RecvFacilityDataEnd, TooShort> {
        Ok(RecvFacilityDataEnd {
            user_request_id: r.u32()?,
            unique_request_id: r.u32()?,
        })
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct LatLonAlt {
        pub lat: f64,
        pub lon: f64,
        pub alt: f64,
    }

    fn parse_lat_lon_alt(r: &mut PacketReader) -> Result<LatLonAlt, TooShort> {
        Ok(LatLonAlt {
            lat: r.f64()?,
            lon: r.f64()?,
            alt: r.f64()?,
        })
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pbh {
        pub pitch: f32,
        pub bank: f32,
        pub heading: f32,
    }

    fn parse_pbh(r: &mut PacketReader) -> Result<Pbh, TooShort> {
        Ok(Pbh {
            pitch: r.f32()?,
            bank: r.f32()?,
            heading: r.f32()?,
        })
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Xyz {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    fn parse_xyz(r: &mut PacketReader) -> Result<Xyz, TooShort> {
        Ok(Xyz {
            x: r.f64()?,
            y: r.f64()?,
            z: r.f64()?,
        })
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct JetwayData {
        pub airport_icao: String,
        pub parking_index: i32,
        pub lla: LatLonAlt,
        pub pbh: Pbh,
        pub status: i32,
        pub door: i32,
        pub exit_door_relative_pos: Xyz,
        pub main_handle_pos: Xyz,
        pub secondary_handle: Xyz,
        pub wheel_ground_lock: Xyz,
        pub jetway_object_id: u32,
        pub attached_object_id: u32,
    }

    fn parse_jetway_data_item(r: &mut PacketReader) -> Result<JetwayData, TooShort> {
        Ok(JetwayData {
            airport_icao: r.fixed_str(8)?,
            parking_index: r.i32()?,
            lla: parse_lat_lon_alt(r)?,
            pbh: parse_pbh(r)?,
            status: r.i32()?,
            door: r.i32()?,
            exit_door_relative_pos: parse_xyz(r)?,
            main_handle_pos: parse_xyz(r)?,
            secondary_handle: parse_xyz(r)?,
            wheel_ground_lock: parse_xyz(r)?,
            jetway_object_id: r.u32()?,
            attached_object_id: r.u32()?,
        })
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RecvJetwayData {
        pub list: ListTemplate,
        pub entries: Vec<JetwayData>,
    }

    pub fn parse_jetway_data(r: &mut PacketReader) -> Result<RecvJetwayData, TooShort> {
        let list = parse_list_template(r)?;
        let entries = (0..list.array_size)
            .map(|_| parse_jetway_data_item(r))
            .collect::<Result<_, _>>()?;
        Ok(RecvJetwayData { list, entries })
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ControllerItem {
        pub device_name: String,
        pub device_id: u32,
        pub product_id: u32,
        pub composite_id: u32,
        pub hardware_version: SimConnectVersion,
    }

    fn parse_controller_item(r: &mut PacketReader) -> Result<ControllerItem, TooShort> {
        Ok(ControllerItem {
            device_name: r.fixed_str(256)?,
            device_id: r.u32()?,
            product_id: r.u32()?,
            composite_id: r.u32()?,
            hardware_version: SimConnectVersion {
                major: r.i32()? as u32,
                minor: r.i32()? as u32,
                build_major: r.i32()? as u32,
                build_minor: r.i32()? as u32,
            },
        })
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RecvControllersList {
        pub list: ListTemplate,
        pub entries: Vec<ControllerItem>,
    }

    pub fn parse_controllers_list(r: &mut PacketReader) -> Result<RecvControllersList, TooShort> {
        let list = parse_list_template(r)?;
        let entries = (0..list.array_size)
            .map(|_| parse_controller_item(r))
            .collect::<Result<_, _>>()?;
        Ok(RecvControllersList { list, entries })
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RecvActionCallback {
        pub action_id: String,
        pub request_id: u32,
    }

    pub fn parse_action_callback(r: &mut PacketReader) -> Result<RecvActionCallback, TooShort> {
        Ok(RecvActionCallback {
            action_id: r.fixed_str(260)?,
            request_id: r.u32()?,
        })
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct InputEventDescriptor {
        pub name: String,
        pub hash: u64,
        /// `SIMCONNECT_INPUT_EVENT_TYPE` — raw discriminant; see
        /// `RecvFacilityData::data_type`'s doc for why.
        pub event_type: u32,
    }

    fn parse_input_event_descriptor(
        r: &mut PacketReader,
    ) -> Result<InputEventDescriptor, TooShort> {
        Ok(InputEventDescriptor {
            name: r.fixed_str(64)?,
            hash: r.u64()?,
            event_type: r.u32()?,
        })
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct RecvEnumerateInputEvents {
        pub list: ListTemplate,
        pub entries: Vec<InputEventDescriptor>,
    }

    pub fn parse_enumerate_input_events(
        r: &mut PacketReader,
    ) -> Result<RecvEnumerateInputEvents, TooShort> {
        let list = parse_list_template(r)?;
        let entries = (0..list.array_size)
            .map(|_| parse_input_event_descriptor(r))
            .collect::<Result<_, _>>()?;
        Ok(RecvEnumerateInputEvents { list, entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::PacketWriter;
    use crate::protocol::ProtocolVersion;
    use crate::send;

    #[test]
    fn open_send_packet_has_expected_opcode_and_version() {
        // The Open *send* and Open *recv* packets are NOT layout-compatible
        // (the send includes an 8-byte "FSX" alias field the recv reply
        // doesn't have), so this only checks the send side's header/opcode.
        let packet = send::open(ProtocolVersion::FsxSp2, "test-app")
            .unwrap()
            .finish(7);
        let mut r = PacketReader::new(&packet);
        let header = r.header().unwrap();
        assert_eq!(header.id, send::opcode::OPEN);
        // Outbound-only 4th field (send id); not part of the inbound header
        // `PacketReader::header` reads, so check it directly.
        assert_eq!(u32::from_le_bytes(packet[12..16].try_into().unwrap()), 7);
    }

    #[test]
    fn recv_open_round_trips_through_writer_and_reader() {
        let mut w = PacketWriter::new_inbound(2, 4);
        w.fixed_str(256, "test-app").unwrap();
        w.i32(1).i32(0).i32(62615).i32(0); // application version
        w.i32(10).i32(0).i32(61259).i32(0); // sim connect version
        let packet = w.finish_inbound();
        let mut r = PacketReader::new(&packet);
        r.header().unwrap();
        let parsed = parse_open(&mut r).unwrap();
        assert_eq!(parsed.application_name, "test-app");
        assert_eq!(parsed.sim_connect_version.build_major, 61259);
    }

    #[test]
    fn event_frame_parses_nested_event() {
        let mut w = PacketWriter::new_inbound(9, 4);
        w.u32(1).u32(2).i32(3).f32(60.0).f32(1.0);
        let packet = w.finish_inbound();
        let mut r = PacketReader::new(&packet);
        r.header().unwrap();
        let ev = parse_event_frame(&mut r).unwrap();
        assert_eq!(ev.event.group_id, 1);
        assert_eq!(ev.event.event_id, 2);
        assert_eq!(ev.frame_rate, 60.0);
    }

    #[cfg(feature = "kittyhawk")]
    mod kittyhawk_tests {
        use super::*;
        use crate::recv::kittyhawk::*;

        #[test]
        fn event_ex1_round_trips() {
            let mut w = PacketWriter::new_inbound(29, 4);
            w.u32(1).u32(2).u32(10).u32(20).u32(30).u32(40).u32(50);
            let packet = w.finish_inbound();
            let mut r = PacketReader::new(&packet);
            r.header().unwrap();
            let ev = parse_event_ex1(&mut r).unwrap();
            assert_eq!(ev.group_id, 1);
            assert_eq!(ev.event_id, 2);
            assert_eq!(ev.data, [10, 20, 30, 40, 50]);
        }

        #[test]
        fn facility_data_send_and_recv_round_trip() {
            let packet = send::request_facility_data(4, 1, 2, "KSEA", None)
                .unwrap()
                .finish(9);
            let mut r = PacketReader::new(&packet);
            let header = r.header().unwrap();
            assert_eq!(header.id, send::opcode::kittyhawk::REQUEST_FACILITY_DATA);
            // Outbound-only 4th field (send id); not part of the inbound
            // header `PacketReader::header` reads, so check it directly.
            assert_eq!(u32::from_le_bytes(packet[12..16].try_into().unwrap()), 9);

            // Recv side has a different layout (envelope + type/index
            // fields, not the request's define/request ids as sent) — spot
            // check it parses without asserting cross-compatibility.
            let mut w = PacketWriter::new_inbound(30, 4);
            w.u32(1).u32(2).u32(0).u32(3).u32(1).u32(0).u32(5);
            w.bytes(&[0xAA, 0xBB, 0xCC]);
            let recv_packet = w.finish_inbound();
            let mut rr = PacketReader::new(&recv_packet);
            rr.header().unwrap();
            let fd = parse_facility_data(&mut rr).unwrap();
            assert_eq!(fd.user_request_id, 1);
            assert_eq!(fd.unique_request_id, 2);
            assert_eq!(fd.data_type, 3);
            assert!(fd.is_list_item);
            assert_eq!(fd.data, &[0xAA, 0xBB, 0xCC]);
        }

        #[test]
        fn jetway_data_round_trips_one_entry() {
            let mut w = PacketWriter::new_inbound(33, 4);
            // list template
            w.u32(1).u32(1).u32(0).u32(1);
            // one JETWAY_DATA entry
            w.fixed_str(8, "KSEA").unwrap();
            w.i32(5); // parking_index
            w.f64(47.44).f64(-122.30).f64(433.0); // Lla
            w.f32(1.0).f32(2.0).f32(3.0); // Pbh
            w.i32(1).i32(0); // status, door
            for _ in 0..4 {
                w.f64(0.0).f64(0.0).f64(0.0); // 4x Xyz
            }
            w.u32(111).u32(222); // object ids
            let packet = w.finish_inbound();
            let mut r = PacketReader::new(&packet);
            r.header().unwrap();
            let jd = parse_jetway_data(&mut r).unwrap();
            assert_eq!(jd.list.array_size, 1);
            assert_eq!(jd.entries.len(), 1);
            assert_eq!(jd.entries[0].airport_icao, "KSEA");
            assert_eq!(jd.entries[0].parking_index, 5);
            assert_eq!(jd.entries[0].jetway_object_id, 111);
        }

        #[test]
        fn controllers_list_round_trips_one_entry() {
            let mut w = PacketWriter::new_inbound(34, 4);
            w.u32(1).u32(1).u32(0).u32(1); // list template
            w.fixed_str(256, "Yoke").unwrap();
            w.u32(1).u32(2).u32(3); // device/product/composite id
            w.i32(1).i32(0).i32(100).i32(0); // hardware version
            let packet = w.finish_inbound();
            let mut r = PacketReader::new(&packet);
            r.header().unwrap();
            let cl = parse_controllers_list(&mut r).unwrap();
            assert_eq!(cl.entries.len(), 1);
            assert_eq!(cl.entries[0].device_name, "Yoke");
            assert_eq!(cl.entries[0].hardware_version.build_major, 100);
        }

        #[test]
        fn action_callback_round_trips() {
            let mut w = PacketWriter::new_inbound(35, 4);
            w.fixed_str(260, "MyAction").unwrap();
            w.u32(42);
            let packet = w.finish_inbound();
            let mut r = PacketReader::new(&packet);
            r.header().unwrap();
            let cb = parse_action_callback(&mut r).unwrap();
            assert_eq!(cb.action_id, "MyAction");
            assert_eq!(cb.request_id, 42);
        }

        #[test]
        fn enumerate_input_events_send_and_recv_round_trip() {
            let packet = send::enumerate_input_events(4, 7).finish(1);
            let mut r = PacketReader::new(&packet);
            let header = r.header().unwrap();
            assert_eq!(header.id, send::opcode::kittyhawk::ENUMERATE_INPUT_EVENTS);

            let mut w = PacketWriter::new_inbound(36, 4);
            w.u32(7).u32(1).u32(0).u32(1); // list template
            w.fixed_str(64, "THROTTLE_SET").unwrap();
            w.u64(0xDEAD_BEEF_u64);
            w.u32(2);
            let recv_packet = w.finish_inbound();
            let mut rr = PacketReader::new(&recv_packet);
            rr.header().unwrap();
            let events = parse_enumerate_input_events(&mut rr).unwrap();
            assert_eq!(events.entries.len(), 1);
            assert_eq!(events.entries[0].name, "THROTTLE_SET");
            assert_eq!(events.entries[0].hash, 0xDEAD_BEEF);
        }

        #[test]
        fn set_and_subscribe_input_event_have_expected_opcodes() {
            let set = send::set_input_event(4, 0x1234, &[1, 2, 3, 4]).finish(1);
            let mut r = PacketReader::new(&set);
            assert_eq!(
                r.header().unwrap().id,
                send::opcode::kittyhawk::SET_INPUT_EVENT
            );

            let sub = send::subscribe_input_event(4, 0x1234).finish(2);
            let mut r = PacketReader::new(&sub);
            assert_eq!(
                r.header().unwrap().id,
                send::opcode::kittyhawk::SUBSCRIBE_INPUT_EVENT
            );
        }
    }
}
