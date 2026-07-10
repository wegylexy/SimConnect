//! Wire-format layer for SimConnect: packet encoding/decoding, enums, and
//! the protocol-version negotiation table. No I/O — see the `simconnect`
//! crate for the transport and connection.

pub mod bcd;
pub mod codec;
pub mod data;
pub mod data_definition;
pub mod enums;
pub mod events;
pub mod protocol;
pub mod recv;
pub mod send;
pub mod strings;
