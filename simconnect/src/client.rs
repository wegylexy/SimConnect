//! Async convenience API over [`Connection`], covering the same opcode set
//! as the prior C# client's public methods. Each method encodes and sends
//! one packet, returning the send id the sim will echo back in
//! `RECV_EXCEPTION`/success replies for correlation.
//!
//! Real non-blocking I/O throughout — `tokio::net::TcpStream`, or tokio's
//! own IOCP-backed `NamedPipeClient` on Windows — not a blocking client
//! moved onto a worker thread. There is no separate sync client: this
//! crate requires tokio unconditionally, on the belief that one correct,
//! non-blocking implementation beats a blocking core plus a second
//! hand-duplicated async wrapper.
//!
//! All methods take `&self`, not `&mut self`: `Connection` locks its read
//! and write sides independently (see `connection.rs`'s module docs), so
//! there's no outer mutability to require exclusive access to `SimConnect`
//! itself — issuing a `send`-style call concurrently with an in-flight
//! `recv_ref()` is fine, and blocks nothing but (briefly) another sender.

use std::io;
use std::sync::Arc;

use simconnect_proto::enums::{
    group_priority, DataRequestFlags, DataSetFlags, DataType, EventFlags, Period, SimObjectType,
};
use simconnect_proto::events;
use simconnect_proto::protocol::ProtocolVersion;
use simconnect_proto::strings::FixedStringError;

use simconnect_proto::bcd::FrequencyBcd16;

pub use crate::connection::RecvGuard;
use crate::connection::{Connection, OpenError};
use crate::transport::{self, Transport};

#[cfg(debug_assertions)]
macro_rules! send_pkt {
    ($self:expr, $packet:expr, $desc:expr) => {
        $self.connection.send_with_desc($packet, || $desc).await
    };
    ($self:expr, $packet:expr) => {
        $self
            .connection
            .send_with_desc($packet, || String::new())
            .await
    };
}

#[cfg(not(debug_assertions))]
macro_rules! send_pkt {
    ($self:expr, $packet:expr, $desc:expr) => {
        $self.connection.send($packet).await
    };
    ($self:expr, $packet:expr) => {
        $self.connection.send($packet).await
    };
}

pub struct SimConnect {
    connection: Arc<Connection>,
}

#[derive(Debug)]
pub enum ClientError {
    Io(io::Error),
    Encode(FixedStringError),
    /// [`SimConnect::set_com_frequency`] was asked for a frequency that
    /// isn't on the 25 kHz grid (`hz % 25_000 != 0`) while talking to a
    /// pre-MSFS2020 sim, which has no `_HZ` event to send it with — the
    /// legacy `COM_RADIO_SET` event can only carry 25 kHz-aligned values.
    UnrepresentableOnLegacyRadio {
        hz: u32,
    },
}

impl From<io::Error> for ClientError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<FixedStringError> for ClientError {
    fn from(e: FixedStringError) -> Self {
        Self::Encode(e)
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Encode(e) => write!(f, "{e}"),
            Self::UnrepresentableOnLegacyRadio { hz } => write!(
                f,
                "{hz} Hz is not on the 25 kHz channel grid and the negotiated \
                 protocol has no exact-Hz radio event to send it with"
            ),
        }
    }
}

impl std::error::Error for ClientError {}

/// Which COM radio to address — used by the frequency-set compatibility
/// helpers below to pick the right event name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComRadio {
    Com1,
    Com2,
    Com3,
}

impl ComRadio {
    pub(crate) fn hz_event_name(self) -> &'static str {
        use events::client::radio_navigation::{
            COM2_RADIO_SET_HZ, COM3_RADIO_SET_HZ, COM_RADIO_SET_HZ,
        };
        match self {
            Self::Com1 => COM_RADIO_SET_HZ,
            Self::Com2 => COM2_RADIO_SET_HZ,
            Self::Com3 => COM3_RADIO_SET_HZ,
        }
    }

    pub(crate) fn bcd16_event_name(self) -> &'static str {
        use events::client::radio_navigation::{COM2_RADIO_SET, COM3_RADIO_SET, COM_RADIO_SET};
        match self {
            Self::Com1 => COM_RADIO_SET,
            Self::Com2 => COM2_RADIO_SET,
            Self::Com3 => COM3_RADIO_SET,
        }
    }

    pub(crate) fn index(self) -> u8 {
        match self {
            Self::Com1 => 1,
            Self::Com2 => 2,
            Self::Com3 => 3,
        }
    }
}

/// Active vs. standby frequency, for [`SimConnect::add_com_frequency_definition`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComFrequencySlot {
    Active,
    Standby,
}

impl ComFrequencySlot {
    pub(crate) fn datum_name_prefix(self) -> &'static str {
        match self {
            Self::Active => "COM ACTIVE FREQUENCY",
            Self::Standby => "COM STANDBY FREQUENCY",
        }
    }
}

impl SimConnect {
    /// Connects over the local named pipe (Windows only), negotiating the
    /// newest protocol version the sim accepts.
    #[cfg(windows)]
    pub async fn open_local(application_name: &str) -> Result<Self, OpenError> {
        Self::open_with(application_name, || async {
            transport::connect_pipe(transport::DEFAULT_PIPE_NAME)
                .await
                .map(|p| Box::new(p) as Box<dyn Transport>)
        })
        .await
    }

    /// Like [`Self::open_local`], but tries every local pipe name this crate knows about instead
    /// of only MSFS's — in order: `transport::DEFAULT_PIPE_NAME` (MSFS 2024 and 2020 alike, see
    /// that constant's own doc comment), `transport::PREPAR3D_PIPE_NAMES` (newest to oldest),
    /// `transport::FSX_PIPE_NAME`, then `transport::scan_for_pipe_names("simconnect")` as a final
    /// catch-all for an installation none of the above name explicitly. The first reachable pipe
    /// wins; if none connect, the last attempt's own error is returned.
    #[cfg(windows)]
    pub async fn open_any_local(application_name: &str) -> Result<Self, OpenError> {
        let mut candidates = vec![transport::DEFAULT_PIPE_NAME.to_string()];
        candidates.extend(transport::PREPAR3D_PIPE_NAMES.iter().map(|s| s.to_string()));
        candidates.push(transport::FSX_PIPE_NAME.to_string());
        candidates.extend(transport::scan_for_pipe_names("simconnect"));

        let mut last_err = None;
        for pipe_name in candidates {
            let result = Self::open_with(application_name, || {
                let pipe_name = pipe_name.clone();
                async move {
                    transport::connect_pipe(&pipe_name)
                        .await
                        .map(|p| Box::new(p) as Box<dyn Transport>)
                }
            })
            .await;
            match result {
                Ok(sim) => return Ok(sim),
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or(OpenError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "no local SimConnect pipe found",
        ))))
    }

    /// Connects over TCP (remote, or local if the sim's SimConnect.cfg
    /// enabled a TCP listener).
    pub async fn open_tcp(
        application_name: &str,
        host: &str,
        port: u16,
    ) -> Result<Self, OpenError> {
        let host = host.to_string();
        Self::open_with(application_name, move || {
            let host = host.clone();
            async move {
                transport::connect_tcp(&host, port)
                    .await
                    .map(|s| Box::new(s) as Box<dyn Transport>)
            }
        })
        .await
    }

    pub async fn open_with<F, Fut>(application_name: &str, connect: F) -> Result<Self, OpenError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = io::Result<Box<dyn Transport>>>,
    {
        Ok(Self {
            connection: Arc::new(Connection::open(application_name, connect).await?),
        })
    }

    /// Provides a [`crate::data_definition::DataDefinitionGuard`] its own
    /// handle to this connection, so it can send `ClearDataDefinition`
    /// from `Drop` independent of this `SimConnect`'s lifetime.
    pub(crate) fn connection_handle(&self) -> Arc<Connection> {
        Arc::clone(&self.connection)
    }

    pub fn protocol(&self) -> ProtocolVersion {
        self.connection.protocol
    }

    /// Awaits the next raw inbound packet, copied into a fresh `Vec<u8>`.
    /// Callers dispatch on the header's `RecvId` and decode with
    /// `simconnect_proto::recv`. For a polling loop where this allocation
    /// shows up, prefer [`Self::recv_ref`], which reuses one buffer across
    /// calls instead.
    pub async fn recv(&self) -> io::Result<Vec<u8>> {
        Ok(self.recv_ref().await?.to_vec())
    }

    /// Zero-copy counterpart to [`Self::recv`]: awaits the next raw
    /// inbound packet into the connection's own reused buffer instead of
    /// allocating a fresh `Vec<u8>`, returning a guard that `Deref`s to
    /// `&[u8]`. Holds only the connection's read-side lock — a concurrent
    /// `send`-style call is never blocked by a `RecvGuard` still being
    /// parsed.
    pub async fn recv_ref(&self) -> io::Result<RecvGuard<'_>> {
        self.connection.recv().await
    }

    pub async fn map_client_event_to_sim_event(
        &self,
        event_id: u32,
        event_name: &str,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::map_client_event_to_sim_event(
            self.connection.protocol_version_wire(),
            event_id,
            event_name,
        )?;
        Ok(send_pkt!(
            self,
            packet,
            format!("MapClientEventToSimEvent(event_id: {event_id}, event: {event_name:?})")
        )?)
    }

    pub async fn transmit_client_event(
        &self,
        object_id: u32,
        event_id: u32,
        data: i32,
        group_id: u32,
        flags: EventFlags,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::transmit_client_event(
            self.connection.protocol_version_wire(),
            object_id,
            event_id,
            data,
            group_id,
            flags,
        );
        send_pkt!(
            self,
            packet,
            format!(
                "TransmitClientEvent(object_id: {object_id}, event_id: {event_id}, data: {data})"
            )
        )
    }

    pub async fn add_client_event_to_notification_group(
        &self,
        group_id: u32,
        event_id: u32,
        maskable: bool,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::add_client_event_to_notification_group(
            self.connection.protocol_version_wire(),
            group_id,
            event_id,
            maskable,
        );
        self.connection.send(packet).await
    }

    pub async fn set_notification_group_priority(
        &self,
        group_id: u32,
        priority: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::set_notification_group_priority(
            self.connection.protocol_version_wire(),
            group_id,
            priority,
        );
        self.connection.send(packet).await
    }

    pub async fn add_to_data_definition(
        &self,
        define_id: u32,
        datum_name: &str,
        units_name: Option<&str>,
        datum_type: DataType,
        epsilon: f32,
        datum_id: i32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::add_to_data_definition(
            self.connection.protocol_version_wire(),
            define_id,
            datum_name,
            units_name,
            datum_type,
            epsilon,
            datum_id,
        )?;
        Ok(send_pkt!(
            self,
            packet,
            format!(
                "AddToDataDefinition(define_id: {define_id}, datum: {datum_name:?}, units: {:?})",
                units_name.unwrap_or("")
            )
        )?)
    }

    /// Registers a COM radio frequency datum at exact precision, for both
    /// 25 kHz and 8.33 kHz-spaced channels alike — the read-side
    /// counterpart to [`Self::set_com_frequency`].
    ///
    /// Unlike the write side, there's no version branching here: this
    /// always requests `Units = "MHz"` with `DataType::Float64` instead of
    /// `"Frequency BCD16"`, and that's not an MSFS2020 addition — `MHz`/
    /// `KHz` unit conversion for `COM ACTIVE/STANDBY FREQUENCY` is
    /// documented, working FSX-era SimConnect usage. Confirmed two ways:
    /// FSDeveloper forum examples predating MSFS2020, and the prior C#
    /// client's own test harness (`SimConnect.Test/Program.cs` on the old
    /// `dev` branch), which already read `COM ACTIVE/STANDBY FREQUENCY:1/2`
    /// via `[DataDefinition(..., "kHz")]` bound to a plain `int` — the
    /// same technique, just `kHz`+`int` there instead of `MHz`+`f64` here.
    /// The BCD16 precision problem this crate worked around for *writing*
    /// never applied to *reading* in the first place — it was always
    /// possible to just ask for a different unit.
    pub async fn add_com_frequency_definition(
        &self,
        define_id: u32,
        radio: ComRadio,
        slot: ComFrequencySlot,
    ) -> Result<u32, ClientError> {
        let datum_name = format!("{}:{}", slot.datum_name_prefix(), radio.index());
        self.add_to_data_definition(
            define_id,
            &datum_name,
            Some("MHz"),
            DataType::Float64,
            0.0,
            simconnect_proto::send::UNUSED,
        )
        .await
    }

    /// Opcodes/layout for the whole ClientData API are cross-confirmed by
    /// two independent reimplementations agreeing exactly, and
    /// live-confirmed against a real MSFS2024 instance — see
    /// `simconnect_proto::send`'s `map_client_data_name_to_id` doc comment.
    pub async fn map_client_data_name_to_id(
        &self,
        client_data_name: &str,
        client_data_id: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::map_client_data_name_to_id(
            self.connection.protocol_version_wire(),
            client_data_name,
            client_data_id,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    pub async fn create_client_data(
        &self,
        client_data_id: u32,
        size: u32,
        read_only: bool,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::create_client_data(
            self.connection.protocol_version_wire(),
            client_data_id,
            size,
            read_only,
        );
        self.connection.send(packet).await
    }

    pub async fn add_to_client_data_definition(
        &self,
        define_id: u32,
        offset: u32,
        size_or_type: u32,
        epsilon: f32,
        datum_id: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::add_to_client_data_definition(
            self.connection.protocol_version_wire(),
            define_id,
            offset,
            size_or_type,
            epsilon,
            datum_id,
        );
        self.connection.send(packet).await
    }

    pub async fn clear_client_data_definition(&self, define_id: u32) -> io::Result<u32> {
        let packet = simconnect_proto::send::clear_client_data_definition(
            self.connection.protocol_version_wire(),
            define_id,
        );
        self.connection.send(packet).await
    }

    pub async fn request_client_data(
        &self,
        client_data_id: u32,
        request_id: u32,
        define_id: u32,
        period: Period,
        flags: DataRequestFlags,
        origin: u32,
        interval: u32,
        limit: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::request_client_data(
            self.connection.protocol_version_wire(),
            client_data_id,
            request_id,
            define_id,
            period,
            flags,
            origin,
            interval,
            limit,
        );
        self.connection.send(packet).await
    }

    /// `array_count`/`unit_size` follow the same convention as
    /// [`Self::set_data_on_sim_object`] — see that method's doc comment.
    pub async fn set_client_data(
        &self,
        client_data_id: u32,
        define_id: u32,
        array_count: u32,
        unit_size: u32,
        data: &[u8],
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::set_client_data(
            self.connection.protocol_version_wire(),
            client_data_id,
            define_id,
            array_count,
            unit_size,
            data,
        );
        self.connection.send(packet).await
    }

    pub async fn request_data_on_sim_object(
        &self,
        request_id: u32,
        define_id: u32,
        object_id: u32,
        period: Period,
        flags: DataRequestFlags,
        origin: u32,
        interval: u32,
        limit: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::request_data_on_sim_object(
            self.connection.protocol_version_wire(),
            request_id,
            define_id,
            object_id,
            period,
            flags,
            origin,
            interval,
            limit,
        );
        send_pkt!(
            self,
            packet,
            format!("RequestDataOnSimObject(request_id: {request_id}, define_id: {define_id}, object_id: {object_id}, period: {period:?})")
        )
    }

    pub async fn request_data_on_sim_object_type(
        &self,
        request_id: u32,
        define_id: u32,
        radius_meters: u32,
        object_type: SimObjectType,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::request_data_on_sim_object_type(
            self.connection.protocol_version_wire(),
            request_id,
            define_id,
            radius_meters,
            object_type,
        );
        send_pkt!(
            self,
            packet,
            format!("RequestDataOnSimObjectType(request_id: {request_id}, define_id: {define_id}, type: {object_type:?})")
        )
    }

    /// `array_count`/`unit_size` are `SimConnect_SetDataOnSimObject`'s
    /// `ArrayCount`/`cbUnitSize` verbatim — see
    /// `simconnect_proto::send::set_data_on_sim_object`'s doc comment for
    /// what to pass for the common single-value case vs. an actual array
    /// write.
    pub async fn set_data_on_sim_object(
        &self,
        define_id: u32,
        object_id: u32,
        flags: DataSetFlags,
        array_count: u32,
        unit_size: u32,
        data: &[u8],
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::set_data_on_sim_object(
            self.connection.protocol_version_wire(),
            define_id,
            object_id,
            flags,
            array_count,
            unit_size,
            data,
        );
        send_pkt!(
            self,
            packet,
            format!("SetDataOnSimObject(define_id: {define_id}, object_id: {object_id})")
        )
    }

    pub async fn subscribe_to_system_event(
        &self,
        event_id: u32,
        event_name: &str,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::subscribe_to_system_event(
            self.connection.protocol_version_wire(),
            event_id,
            event_name,
        )?;
        Ok(send_pkt!(
            self,
            packet,
            format!("SubscribeToSystemEvent(event_id: {event_id}, event: {event_name:?})")
        )?)
    }

    pub async fn unsubscribe_from_system_event(&self, event_id: u32) -> io::Result<u32> {
        let packet = simconnect_proto::send::unsubscribe_from_system_event(
            self.connection.protocol_version_wire(),
            event_id,
        );
        send_pkt!(
            self,
            packet,
            format!("UnsubscribeFromSystemEvent(event_id: {event_id})")
        )
    }

    /// Creates an AI-controlled aircraft currently parked with no flight
    /// plan. The server-assigned object id arrives later, out of band, as
    /// a `RECV_ASSIGNED_OBJECT_ID` carrying this call's `request_id` —
    /// dispatch on the header's `RecvId::AssignedObjectId` and decode with
    /// `simconnect_proto::recv::parse_assigned_object_id`. Follow up with
    /// [`Self::ai_set_aircraft_flight_plan`] to set it in motion.
    pub async fn ai_create_parked_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        airport_id: &str,
        request_id: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::ai_create_parked_atc_aircraft(
            self.connection.protocol_version_wire(),
            container_title,
            tail_number,
            airport_id,
            request_id,
        )?;
        Ok(send_pkt!(
            self,
            packet,
            format!("AiCreateParkedAtcAircraft(title: {container_title:?}, tail: {tail_number:?}, airport: {airport_id:?}, request_id: {request_id})")
        )?)
    }

    /// Creates an AI-controlled aircraft already underway on a flight
    /// plan, on the ground or airborne — typically IFR, in constant radio
    /// contact with ATC. See [`Self::ai_create_parked_atc_aircraft`] for
    /// how to retrieve the assigned object id.
    pub async fn ai_create_enroute_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        flight_number: i32,
        flight_plan_path: &str,
        flight_plan_position: f64,
        touch_and_go: bool,
        request_id: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::ai_create_enroute_atc_aircraft(
            self.connection.protocol_version_wire(),
            container_title,
            tail_number,
            flight_number,
            flight_plan_path,
            flight_plan_position,
            touch_and_go,
            request_id,
        )?;
        Ok(send_pkt!(
            self,
            packet,
            format!("AiCreateEnrouteAtcAircraft(title: {container_title:?}, tail: {tail_number:?}, request_id: {request_id})")
        )?)
    }

    /// Creates an aircraft not under ATC control (typically VFR) — also
    /// the entry point for helicopters/gliders/balloons, which have no
    /// internal AI pilot. See
    /// [`Self::ai_create_parked_atc_aircraft`] for how to retrieve the
    /// assigned object id.
    pub async fn ai_create_non_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        init_position: &simconnect_proto::data::InitPosition,
        request_id: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::ai_create_non_atc_aircraft(
            self.connection.protocol_version_wire(),
            container_title,
            tail_number,
            init_position,
            request_id,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    /// Creates an AI-controlled object other than an aircraft (ground
    /// vehicles, boats, and other `sim.cfg`-defined simulation objects).
    /// See [`Self::ai_create_parked_atc_aircraft`] for how to retrieve the
    /// assigned object id.
    pub async fn ai_create_simulated_object(
        &self,
        container_title: &str,
        init_position: &simconnect_proto::data::InitPosition,
        request_id: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::ai_create_simulated_object(
            self.connection.protocol_version_wire(),
            container_title,
            init_position,
            request_id,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    /// MSFS2024 `_EX1` variant of [`Self::ai_create_simulated_object`],
    /// adding a `livery` parameter for modular SimObjects. Opcode/layout
    /// cross-confirmed against an independent reimplementation, and
    /// live-confirmed against a real MSFS2024 instance: called with the
    /// user's own aircraft title and an empty livery string, it returned a
    /// genuine `RECV_ID::AssignedObjectId` (not an exception), and the
    /// resulting object was cleaned up successfully with
    /// [`Self::ai_remove_object`].
    #[cfg(feature = "sunrise")]
    pub async fn ai_create_simulated_object_ex1(
        &self,
        container_title: &str,
        livery: &str,
        init_position: &simconnect_proto::data::InitPosition,
        request_id: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::ai_create_simulated_object_ex1(
            self.connection.protocol_version_wire(),
            container_title,
            livery,
            init_position,
            request_id,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    /// MSFS2024 `_EX1` variant of [`Self::ai_create_enroute_atc_aircraft`],
    /// adding a `livery` parameter. Opcode/layout cross-confirmed against an
    /// independent reimplementation, not yet verified against a live sim
    /// capture by this crate.
    #[cfg(feature = "sunrise")]
    pub async fn ai_create_enroute_atc_aircraft_ex1(
        &self,
        container_title: &str,
        livery: &str,
        tail_number: &str,
        flight_number: i32,
        flight_plan_path: &str,
        flight_plan_position: f64,
        touch_and_go: bool,
        request_id: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::ai_create_enroute_atc_aircraft_ex1(
            self.connection.protocol_version_wire(),
            container_title,
            livery,
            tail_number,
            flight_number,
            flight_plan_path,
            flight_plan_position,
            touch_and_go,
            request_id,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    /// Transfers control of an AI-created object to this client — without
    /// this, the AI system and the client may fight over control with
    /// unpredictable results. `object_id` is the id from the
    /// `RECV_ASSIGNED_OBJECT_ID` reply to whichever `ai_create_*` call
    /// created it.
    pub async fn ai_release_control(&self, object_id: u32, request_id: u32) -> io::Result<u32> {
        let packet = simconnect_proto::send::ai_release_control(
            self.connection.protocol_version_wire(),
            object_id,
            request_id,
        );
        self.connection.send(packet).await
    }

    /// Removes an AI-created object. A client can only remove objects it
    /// created, not ones created by another client or by the sim itself.
    pub async fn ai_remove_object(&self, object_id: u32, request_id: u32) -> io::Result<u32> {
        let packet = simconnect_proto::send::ai_remove_object(
            self.connection.protocol_version_wire(),
            object_id,
            request_id,
        );
        self.connection.send(packet).await
    }

    /// Sets or changes an AI-controlled aircraft's flight plan —
    /// typically called some time after
    /// [`Self::ai_create_parked_atc_aircraft`] to set it in motion.
    /// `flight_plan_path` is a `.pln` file path (extension optional; a
    /// bare filename resolves against the default Flight Simulator Files
    /// directory).
    pub async fn ai_set_aircraft_flight_plan(
        &self,
        object_id: u32,
        flight_plan_path: &str,
        request_id: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::ai_set_aircraft_flight_plan(
            self.connection.protocol_version_wire(),
            object_id,
            flight_plan_path,
            request_id,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    /// Sets a COM radio's frequency, automatically picking the exact-Hz
    /// event on a connection that negotiated MSFS2020+ ("KittyHawk" or
    /// newer) and the legacy 25 kHz BCD16 event otherwise (FSX/pre-2020,
    /// where the `_HZ` events don't exist). This is the method to reach
    /// for by default.
    ///
    /// Neither of these events is an official SimConnect API function —
    /// `SimConnect.h` only defines the raw event *names*
    /// (`COM_RADIO_SET`/`COM_RADIO_SET_HZ`) sent through the ordinary
    /// client-event path; picking between them based on what a connection
    /// negotiated is this crate's own convenience, not a mirrored SDK
    /// function.
    ///
    /// On the legacy path, `hz` must be exactly on the 25 kHz grid
    /// (`hz % 25_000 == 0`) — anything finer (an 8.33 kHz-only channel)
    /// can't be sent to a sim with no `_HZ` event at all, so this returns
    /// [`ClientError::UnrepresentableOnLegacyRadio`] rather than silently
    /// rounding to the nearest 25 kHz channel. Use
    /// [`Self::set_com_frequency_bcd16`] directly if silent rounding is
    /// what you actually want.
    ///
    /// [`Self::set_com_frequency_hz`]/[`Self::set_com_frequency_bcd16`]
    /// exist underneath this for callers who need to force one specific
    /// path regardless of what was negotiated (e.g. talking to an old
    /// aircraft/gauge that only listens for the plain event even on a
    /// modern sim).
    pub async fn set_com_frequency(
        &self,
        radio: ComRadio,
        event_id: u32,
        hz: u32,
    ) -> Result<u32, ClientError> {
        if self.negotiated_at_least_kittyhawk() {
            self.set_com_frequency_hz(radio, event_id, hz).await
        } else if hz % 25_000 == 0 {
            self.set_com_frequency_bcd16(radio, event_id, hz / 1000)
                .await
        } else {
            Err(ClientError::UnrepresentableOnLegacyRadio { hz })
        }
    }

    #[cfg(feature = "kittyhawk")]
    fn negotiated_at_least_kittyhawk(&self) -> bool {
        self.protocol().at_least_kittyhawk()
    }

    #[cfg(not(feature = "kittyhawk"))]
    fn negotiated_at_least_kittyhawk(&self) -> bool {
        // The `kittyhawk` feature is off, so `ProtocolVersion` has no
        // KittyHawk/SunRise entries to negotiate up to in the first place —
        // always fall back to the legacy path.
        false
    }

    /// Sets a COM radio's frequency using the exact-Hz client event
    /// (`COM_RADIO_SET_HZ`/`COM2_RADIO_SET_HZ`/`COM3_RADIO_SET_HZ`), which
    /// works for both 25 kHz and 8.33 kHz-spaced radios, unconditionally —
    /// prefer [`Self::set_com_frequency`] unless you specifically need to
    /// force this path regardless of the negotiated protocol version.
    /// `event_id` is the client-side id to map the event name to (caller's
    /// choice, must be unique per mapped event on this connection).
    ///
    /// Transmits with `EventFlags::GROUP_ID_IS_PRIORITY` and
    /// `group_priority::HIGHEST` as the group id — live-confirmed
    /// necessary: passing group `0` with no flags (this method's original
    /// implementation) gets a silent `RECV_EXCEPTION::UnrecognizedId`,
    /// because group `0` was never actually created via
    /// `AddClientEventToNotificationGroup`. The priority-flag form bypasses
    /// needing a real notification group at all.
    pub async fn set_com_frequency_hz(
        &self,
        radio: ComRadio,
        event_id: u32,
        hz: u32,
    ) -> Result<u32, ClientError> {
        self.map_client_event_to_sim_event(event_id, radio.hz_event_name())
            .await?;
        Ok(self
            .transmit_client_event(
                0,
                event_id,
                hz as i32,
                group_priority::HIGHEST,
                EventFlags::GROUP_ID_IS_PRIORITY,
            )
            .await?)
    }

    /// Sets a COM radio's frequency using the legacy 25 kHz
    /// `FrequencyBcd16`-encoded client event, unconditionally — prefer
    /// [`Self::set_com_frequency`] unless you specifically need to force
    /// this path (e.g. an old aircraft/gauge that only listens for
    /// `COM_RADIO_SET` even on a modern sim). Loses precision on 8.33 kHz
    /// channels; see [`FrequencyBcd16`]'s docs.
    ///
    /// See [`Self::set_com_frequency_hz`]'s doc comment for why this
    /// transmits with `EventFlags::GROUP_ID_IS_PRIORITY` +
    /// `group_priority::HIGHEST` rather than group `0`/no flags.
    pub async fn set_com_frequency_bcd16(
        &self,
        radio: ComRadio,
        event_id: u32,
        khz: u32,
    ) -> Result<u32, ClientError> {
        self.map_client_event_to_sim_event(event_id, radio.bcd16_event_name())
            .await?;
        let bcd = FrequencyBcd16::from_khz(khz).0;
        Ok(self
            .transmit_client_event(
                0,
                event_id,
                bcd as i32,
                group_priority::HIGHEST,
                EventFlags::GROUP_ID_IS_PRIORITY,
            )
            .await?)
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn add_to_facility_definition(
        &self,
        define_id: u32,
        field_name: &str,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::add_to_facility_definition(
            self.connection.protocol_version_wire(),
            define_id,
            field_name,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn subscribe_to_facilities(
        &self,
        facility_list_type: u32,
        request_id: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::subscribe_to_facilities(
            self.connection.protocol_version_wire(),
            facility_list_type,
            request_id,
        );
        self.connection.send(packet).await
    }

    /// Legacy FSX-era facility list request — see
    /// `simconnect_proto::send::request_facilities_list`'s doc comment for
    /// how its opcode/layout were captured.
    pub async fn request_facilities_list(
        &self,
        facility_list_type: u32,
        request_id: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::request_facilities_list(
            self.connection.protocol_version_wire(),
            facility_list_type,
            request_id,
        );
        self.connection.send(packet).await
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn request_facility_data(
        &self,
        define_id: u32,
        request_id: u32,
        icao: &str,
        region: Option<&str>,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::request_facility_data(
            self.connection.protocol_version_wire(),
            define_id,
            request_id,
            icao,
            region,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn request_facility_data_ex1(
        &self,
        define_id: u32,
        request_id: u32,
        icao: &str,
        region: Option<&str>,
        facility_type: Option<char>,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::request_facility_data_ex1(
            self.connection.protocol_version_wire(),
            define_id,
            request_id,
            icao,
            region,
            facility_type,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn request_jetway_data(
        &self,
        airport_icao: &str,
        parking_indices: &[i32],
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::request_jetway_data(
            self.connection.protocol_version_wire(),
            airport_icao,
            parking_indices,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn enumerate_controllers(&self) -> io::Result<u32> {
        let packet =
            simconnect_proto::send::enumerate_controllers(self.connection.protocol_version_wire());
        self.connection.send(packet).await
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn enumerate_input_events(&self, request_id: u32) -> io::Result<u32> {
        let packet = simconnect_proto::send::enumerate_input_events(
            self.connection.protocol_version_wire(),
            request_id,
        );
        self.connection.send(packet).await
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn subscribe_input_event(&self, input_event_hash: u64) -> io::Result<u32> {
        let packet = simconnect_proto::send::subscribe_input_event(
            self.connection.protocol_version_wire(),
            input_event_hash,
        );
        self.connection.send(packet).await
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn set_input_event(&self, input_event_hash: u64, value: &[u8]) -> io::Result<u32> {
        let packet = simconnect_proto::send::set_input_event(
            self.connection.protocol_version_wire(),
            input_event_hash,
            value,
        );
        self.connection.send(packet).await
    }

    /// Opcode/layout ground-truthed — see
    /// `simconnect_proto::send::subscribe_to_flow_event`.
    #[cfg(feature = "sunrise")]
    pub async fn subscribe_to_flow_event(&self) -> io::Result<u32> {
        let packet = simconnect_proto::send::subscribe_to_flow_event(
            self.connection.protocol_version_wire(),
        );
        self.connection.send(packet).await
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn get_input_event(&self, request_id: u32, hash: u64) -> io::Result<u32> {
        let packet = simconnect_proto::send::get_input_event(
            self.connection.protocol_version_wire(),
            request_id,
            hash,
        );
        self.connection.send(packet).await
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn enumerate_input_event_params(&self, hash: u64) -> io::Result<u32> {
        let packet = simconnect_proto::send::enumerate_input_event_params(
            self.connection.protocol_version_wire(),
            hash,
        );
        self.connection.send(packet).await
    }

    pub async fn request_system_state(
        &self,
        request_id: u32,
        state: &str,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::request_system_state(
            self.connection.protocol_version_wire(),
            request_id,
            state,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    pub async fn set_system_state(
        &self,
        state: &str,
        integer: u32,
        float: f32,
        string: &str,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::set_system_state(
            self.connection.protocol_version_wire(),
            state,
            integer,
            float,
            string,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    /// The counterpart to [`Self::subscribe_to_facilities`] — opcode
    /// ground-truthed.
    #[cfg(feature = "kittyhawk")]
    pub async fn unsubscribe_to_facilities(&self, facility_list_type: u32) -> io::Result<u32> {
        let packet = simconnect_proto::send::unsubscribe_to_facilities(
            self.connection.protocol_version_wire(),
            facility_list_type,
        );
        self.connection.send(packet).await
    }

    pub async fn request_reserved_key(
        &self,
        event_id: u32,
        key_choice_1: &str,
        key_choice_2: &str,
        key_choice_3: &str,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::request_reserved_key(
            self.connection.protocol_version_wire(),
            event_id,
            key_choice_1,
            key_choice_2,
            key_choice_3,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    pub async fn remove_input_event(
        &self,
        group_id: u32,
        input_definition: &str,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::remove_input_event(
            self.connection.protocol_version_wire(),
            group_id,
            input_definition,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    #[cfg(feature = "sunrise")]
    pub async fn camera_acquire(&self, client_id: &str) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::camera_acquire(
            self.connection.protocol_version_wire(),
            client_id,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn camera_set_relative_6dof(
        &self,
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
        pitch_deg: f32,
        bank_deg: f32,
        heading_deg: f32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::camera_set_relative_6dof(
            self.connection.protocol_version_wire(),
            delta_x,
            delta_y,
            delta_z,
            pitch_deg,
            bank_deg,
            heading_deg,
        );
        self.connection.send(packet).await
    }

    #[cfg(feature = "sunrise")]
    pub async fn subscribe_to_comm_bus_event(
        &self,
        event_id: u32,
        event_name: &str,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::subscribe_to_comm_bus_event(
            self.connection.protocol_version_wire(),
            event_id,
            event_name,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    #[cfg(feature = "sunrise")]
    pub async fn call_comm_bus_event(
        &self,
        event_name: &str,
        broadcast_to: u32,
        data: &[u8],
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::call_comm_bus_event(
            self.connection.protocol_version_wire(),
            event_name,
            broadcast_to,
            data,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    pub async fn menu_add_item(
        &self,
        menu_item: &str,
        menu_event_id: u32,
        data: u32,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::menu_add_item(
            self.connection.protocol_version_wire(),
            menu_item,
            menu_event_id,
            data,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    /// `_EX1` variant of [`Self::request_facilities_list`] — opcode
    /// ground-truthed.
    #[cfg(feature = "kittyhawk")]
    pub async fn request_facilities_list_ex1(
        &self,
        facility_list_type: u32,
        request_id: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::request_facilities_list_ex1(
            self.connection.protocol_version_wire(),
            facility_list_type,
            request_id,
        );
        self.connection.send(packet).await
    }

    #[cfg(feature = "kittyhawk")]
    pub async fn request_all_facilities(
        &self,
        facility_list_type: u32,
        request_id: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::request_all_facilities(
            self.connection.protocol_version_wire(),
            facility_list_type,
            request_id,
        );
        self.connection.send(packet).await
    }

    /// `_EX1` variant of [`Self::map_input_event_to_client_event`] — opcode
    /// ground-truthed.
    #[cfg(feature = "kittyhawk")]
    #[allow(clippy::too_many_arguments)]
    pub async fn map_input_event_to_client_event_ex1(
        &self,
        group_id: u32,
        input_definition: &str,
        down_event_id: u32,
        down_value: u32,
        up_event_id: u32,
        up_value: u32,
        maskable: bool,
    ) -> Result<u32, ClientError> {
        let packet = simconnect_proto::send::map_input_event_to_client_event_ex1(
            self.connection.protocol_version_wire(),
            group_id,
            input_definition,
            down_event_id,
            down_value,
            up_event_id,
            up_value,
            maskable,
        )?;
        Ok(self.connection.send(packet).await?)
    }

    /// `_EX1` variant of [`Self::transmit_client_event`] — opcode
    /// ground-truthed.
    #[cfg(feature = "kittyhawk")]
    #[allow(clippy::too_many_arguments)]
    pub async fn transmit_client_event_ex1(
        &self,
        object_id: u32,
        event_id: u32,
        group_id: u32,
        flags: u32,
        data0: u32,
        data1: u32,
        data2: u32,
        data3: u32,
        data4: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::transmit_client_event_ex1(
            self.connection.protocol_version_wire(),
            object_id,
            event_id,
            group_id,
            flags,
            data0,
            data1,
            data2,
            data3,
            data4,
        );
        self.connection.send(packet).await
    }

    #[cfg(feature = "sunrise")]
    pub async fn enumerate_sim_objects_and_liveries(
        &self,
        request_id: u32,
        object_type: u32,
    ) -> io::Result<u32> {
        let packet = simconnect_proto::send::enumerate_sim_objects_and_liveries(
            self.connection.protocol_version_wire(),
            request_id,
            object_type,
        );
        self.connection.send(packet).await
    }

    /// Looks up the human-readable description for a previously sent `send_id`.
    /// Exists only under `#[cfg(debug_assertions)]`.
    #[cfg(debug_assertions)]
    pub fn describe_send(&self, send_id: u32) -> Option<String> {
        self.connection.describe_send(send_id)
    }
}
