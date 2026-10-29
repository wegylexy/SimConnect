#![no_std]

//! Wire-format layer for SimConnect: packet encoding/decoding, enums, and
//! the protocol-version negotiation table. No I/O — see the `simconnect`
//! crate for the transport and connection.
//!
//! This crate uses `alloc` for owned packet buffers and decoded strings, but
//! does not require the Rust standard library. A `no_std` application must
//! provide a global allocator to use those APIs.

extern crate alloc;

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
