//! Shared trait/schema types for `#[derive(simconnect::DataDefinition)]`
//! (implemented in the sibling `simconnect-derive` proc-macro crate) and
//! `simconnect::client::SimConnect::define_data`. Lives here rather than in
//! `simconnect` so both the derive-macro-generated code and the runtime
//! client reference it through one path, `simconnect::proto::data_definition`
//! (re-exported), without either depending on the other in a cycle.

use crate::codec::TooShort;
use crate::enums::DataType;
use crate::strings::FixedStringError;

/// One field's registration info, as sent to `SimConnect_AddToDataDefinition`.
#[derive(Debug, Clone, Copy)]
pub struct FieldSpec {
    pub datum_name: &'static str,
    pub units_name: Option<&'static str>,
    pub epsilon: f32,
    pub data_type: DataType,
}

/// Implemented by `#[derive(simconnect::DataDefinition)]`. `SCHEMA` lists
/// fields in declaration order — the same order they must be registered
/// via `AddToDataDefinition` and the order `SimConnect` returns their
/// values in a `RECV_SIMOBJECT_DATA` payload.
pub trait DataDefinition: Sized {
    const SCHEMA: &'static [FieldSpec];

    /// Decodes a `RecvSimObjectData::data` payload registered with this
    /// type's `SCHEMA`.
    fn decode(data: &[u8]) -> Result<Self, TooShort>;

    /// Encodes `self` for `send::set_data_on_sim_object`. Fallible because
    /// a fixed-width string field (`String8`..`String260`, `MarkerState`'s
    /// name) can fail to encode (too long, or a codepoint outside Latin-1)
    /// — see `strings::FixedStringError`.
    fn encode(&self) -> Result<Vec<u8>, FixedStringError>;
}
