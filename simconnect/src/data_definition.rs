//! Runtime side of `#[derive(simconnect::DataDefinition)]`. The trait
//! itself (`DataDefinition`) and its schema type (`FieldSpec`) live in
//! `simconnect_proto::data_definition` and are re-exported here so both
//! the derive-macro-generated code and this module reference them through
//! one path, `simconnect::data_definition` (or the flat
//! `simconnect::DataDefinition`).

use std::sync::Arc;

use simconnect_proto::codec::TooShort;
pub use simconnect_proto::data_definition::{DataDefinition, FieldSpec};
use simconnect_proto::strings::FixedStringError;

use crate::client::{ClientError, SimConnect};
use crate::connection::Connection;

/// Handle returned by [`SimConnect::define_data`]. Sends
/// `ClearDataDefinition` when dropped, best-effort. `Drop` can't `await`,
/// so cleanup is a fire-and-forget `tokio::spawn`ed task rather than an
/// inline send — if there's no runtime currently running (e.g. the guard
/// outlives the runtime), the clear is silently skipped, same as any other
/// "best effort, can't propagate a `Result` from `Drop`" cleanup. Mirrors
/// the *intent* of the old C# client's `IAsyncDisposable` `DataDefinition`
/// handle, not its mechanism (that one could actually await the clear).
pub struct DataDefinitionGuard<T: DataDefinition> {
    define_id: u32,
    connection: Arc<Connection>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DataDefinition> DataDefinitionGuard<T> {
    /// The `define_id` this guard registered `T::SCHEMA` under — pass to
    /// [`SimConnect::request_data_on_sim_object`]/
    /// [`SimConnect::set_data_on_sim_object`].
    pub fn define_id(&self) -> u32 {
        self.define_id
    }

    /// Decodes a `RecvSimObjectData::data` payload for this definition.
    pub fn decode(&self, data: &[u8]) -> Result<T, TooShort> {
        T::decode(data)
    }

    /// Encodes `value` for [`SimConnect::set_data_on_sim_object`].
    pub fn encode(&self, value: &T) -> Result<Vec<u8>, FixedStringError> {
        value.encode()
    }

    /// Encodes `value` and sends it as a `SetDataOnSimObject` targeting
    /// `object_id` (`0` for the user aircraft, or the id from a
    /// `RECV_ASSIGNED_OBJECT_ID` reply for an AI-created object) — the
    /// same call as [`SimConnect::set_data_on_sim_object`], but without
    /// needing to pass this guard's own `define_id`, encode `value`
    /// yourself, or spell out `DataSetFlags::empty()`/`array_count = 0`/
    /// `unit_size = value.encode()?.len()` for the common "write one
    /// value" case.
    pub async fn set_data_on_sim_object(
        &self,
        object_id: u32,
        value: &T,
    ) -> Result<u32, ClientError> {
        let bytes = value.encode()?;
        let packet = simconnect_proto::send::set_data_on_sim_object(
            self.connection.protocol_version_wire(),
            self.define_id,
            object_id,
            simconnect_proto::enums::DataSetFlags::empty(),
            0,
            bytes.len() as u32,
            &bytes,
        );
        Ok(self.connection.send(packet).await?)
    }
}

/// Handle returned by [`SimConnect::define_client_data`]. Unlike
/// [`DataDefinitionGuard`], this does *not* send `ClearClientDataDefinition`
/// on drop — a ClientData area and its definition are typically meant to
/// outlive the client that mapped them (other add-ons, or this same add-on
/// across a reconnect, may want to keep reading/writing the same area), so
/// tying its lifetime to this handle would be surprising. Call
/// [`SimConnect::clear_client_data_definition`] explicitly if you do want it
/// gone.
pub struct ClientDataDefinitionGuard<T: DataDefinition> {
    client_data_id: u32,
    define_id: u32,
    connection: Arc<Connection>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DataDefinition> ClientDataDefinitionGuard<T> {
    pub fn client_data_id(&self) -> u32 {
        self.client_data_id
    }

    pub fn define_id(&self) -> u32 {
        self.define_id
    }

    /// Decodes a `RecvClientData::data` payload (see
    /// `simconnect_proto::recv::parse_client_data`) for this definition.
    pub fn decode(&self, data: &[u8]) -> Result<T, TooShort> {
        T::decode(data)
    }

    /// Encodes `value` and sends it as a `SetClientData` write — the
    /// ClientData counterpart to
    /// [`DataDefinitionGuard::set_data_on_sim_object`].
    pub async fn set_client_data(&self, value: &T) -> Result<u32, ClientError> {
        let bytes = value.encode()?;
        let packet = simconnect_proto::send::set_client_data(
            self.connection.protocol_version_wire(),
            self.client_data_id,
            self.define_id,
            0,
            bytes.len() as u32,
            &bytes,
        );
        Ok(self.connection.send(packet).await?)
    }
}

impl<T: DataDefinition> Drop for DataDefinitionGuard<T> {
    fn drop(&mut self) {
        let define_id = self.define_id;
        let connection = Arc::clone(&self.connection);
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(async move {
                let packet = simconnect_proto::send::clear_data_definition(
                    connection.protocol_version_wire(),
                    define_id,
                );
                let _ = connection.send(packet).await;
            });
        }
    }
}

impl SimConnect {
    /// Registers every field of `T::SCHEMA` under `define_id` (one
    /// `AddToDataDefinition` per field, in declaration order — see
    /// `#[derive(DataDefinition)]`), returning a guard that decodes/encodes
    /// `T` and clears the definition when dropped.
    ///
    /// ```no_run
    /// use simconnect::DataDefinition;
    ///
    /// #[derive(DataDefinition)]
    /// struct Radios {
    ///     // `__1__mhz`: a numeric index segment and a unit-alias segment,
    ///     // classified by content (not position) and both stripped
    ///     // before deriving the name — together they give datum name
    ///     // `"COM ACTIVE FREQUENCY:1"` (colon, matching the real simvar;
    ///     // Rust identifiers can't contain `:` at all) and `units = "MHz"`.
    ///     com_active_frequency__1__mhz: f64,
    ///     com_standby_frequency__1__mhz: f64,
    ///     com_active_frequency__2__mhz: f64,
    /// }
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut sim = simconnect::SimConnect::open_tcp("my-addon", "127.0.0.1", 500).await?;
    /// let radios = sim.define_data::<Radios>(1).await?; // sends one AddToDataDefinition per field
    /// // `radios` drops here (or later, once you're done with it) ->
    /// // ClearDataDefinition is sent automatically (fire-and-forget, needs
    /// // a running tokio runtime at drop time).
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// `no_run` because this compiles the example (catching drift between
    /// the docs and the real API) without requiring a live sim connection
    /// to actually execute the doctest. `open_tcp` is used here instead of
    /// the Windows-only `open_local` so the doctest compiles on every
    /// platform this crate's CI might run on.
    pub async fn define_data<T: DataDefinition>(
        &self,
        define_id: u32,
    ) -> Result<DataDefinitionGuard<T>, ClientError> {
        for field in T::SCHEMA {
            self.add_to_data_definition(
                define_id,
                field.datum_name,
                field.units_name,
                field.data_type,
                field.epsilon,
                simconnect_proto::send::UNUSED,
            )
            .await?;
        }
        Ok(DataDefinitionGuard {
            define_id,
            connection: self.connection_handle(),
            _marker: std::marker::PhantomData,
        })
    }

    /// Registers `T::SCHEMA` as a ClientData definition instead of a
    /// SimObject data definition: each field goes through
    /// `AddToClientDataDefinition` at successive byte offsets (computed from
    /// `DataType::byte_width`) rather than `AddToDataDefinition` matched by
    /// simvar name — `T`'s fields are just being reused as a raw memory
    /// layout here, not simvar bindings, so `units_name` is ignored.
    /// Live-confirmed end to end against a real MSFS2024 instance (a
    /// two-`i32`-field struct mapped/created/defined/written with no
    /// exception reply).
    ///
    /// Does not call `MapClientDataNameToID`/`CreateClientData` — call
    /// [`Self::map_client_data_name_to_id`]/[`Self::create_client_data`]
    /// first to establish `client_data_id` (this mirrors the real
    /// `SimConnect_AddToClientDataDefinition` API, which likewise assumes
    /// the area already exists).
    ///
    /// # Panics
    ///
    /// Panics if any field's `DataType` has no fixed byte width (`StringV`,
    /// or `Invalid` — neither should appear in a real `#[derive(DataDefinition)]`
    /// schema) — a ClientData definition has no way to express a
    /// variable-length field at a static offset.
    pub async fn define_client_data<T: DataDefinition>(
        &self,
        client_data_id: u32,
        define_id: u32,
    ) -> Result<ClientDataDefinitionGuard<T>, ClientError> {
        let mut offset = 0u32;
        for field in T::SCHEMA {
            let width = field.data_type.byte_width().unwrap_or_else(|| {
                panic!(
                    "field with datum_name {:?} has DataType {:?}, which has no fixed byte width \
                     and can't be placed in a ClientData definition",
                    field.datum_name, field.data_type
                )
            });
            self.add_to_client_data_definition(
                define_id,
                offset,
                width,
                field.epsilon,
                simconnect_proto::send::UNUSED as u32,
            )
            .await?;
            offset += width;
        }
        Ok(ClientDataDefinitionGuard {
            client_data_id,
            define_id,
            connection: self.connection_handle(),
            _marker: std::marker::PhantomData,
        })
    }
}
