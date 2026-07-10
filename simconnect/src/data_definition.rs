//! Runtime side of `#[derive(simconnect::DataDefinition)]`. The trait
//! itself (`DataDefinition`) and its schema type (`FieldSpec`) live in
//! `simconnect_proto::data_definition` and are re-exported here so both
//! the derive-macro-generated code and this module reference them through
//! one path, `simconnect::data_definition` (or the flat
//! `simconnect::DataDefinition`).

use std::sync::Arc;

pub use simconnect_proto::data_definition::{DataDefinition, FieldSpec};
use simconnect_proto::codec::TooShort;
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
}
