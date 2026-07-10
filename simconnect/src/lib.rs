//! Pure-Rust SimConnect client. Talks the wire protocol directly (see
//! `simconnect-proto`) over a local named pipe or TCP — no `SimConnect.dll`
//! dependency, so this crate can be statically linked into any Rust binary.
//!
//! [`client::SimConnect`] is genuinely non-blocking end to end —
//! `tokio::net::TcpStream`, or tokio's own IOCP-backed `NamedPipeClient` on
//! Windows — not a blocking client moved onto a worker thread. There is no
//! separate sync API: this crate requires tokio unconditionally, on the
//! belief that one correct async core beats a blocking core plus a
//! hand-duplicated async wrapper.

pub mod cfg;
pub mod client;
pub mod connection;
pub mod data_definition;
pub mod transport;

pub use client::SimConnect;
pub use connection::OpenError;
pub use data_definition::DataDefinition;

pub use simconnect_proto as proto;

/// `#[derive(DataDefinition)]` — see [`data_definition`]'s module docs for
/// a full worked example. Shares the name `DataDefinition` with the trait
/// re-exported above; that's intentional (they occupy separate
/// namespaces — macro vs. type — the same way `serde::Serialize` names
/// both its derive macro and its trait), so `use simconnect::DataDefinition;`
/// brings in both at once.
pub use simconnect_derive::DataDefinition;
