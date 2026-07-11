//! Little-endian byte cursor helpers used by `send`/`recv`.
//!
//! The wire format is a flat header (`i32` size, `u32` protocol version,
//! `u32` opcode/`RECV_ID`, `u32` send id for outbound packets) followed by
//! fixed-layout fields. We build/parse it with an explicit byte cursor
//! rather than `#[repr(C, packed)]` + pointer casts (the approach the old
//! C# client used): packed structs with multi-byte fields are only safely
//! read through raw pointers in C#, but the equivalent in Rust
//! (`#[repr(packed)]` + `&field`) is undefined behavior because it can
//! produce an unaligned reference. A cursor sidesteps the hazard entirely
//! and works identically on every target.

use crate::strings::{decode_fixed, encode_fixed, FixedStringError};

pub struct PacketWriter {
    buf: Vec<u8>,
}

impl PacketWriter {
    /// Reserves a packet with a placeholder header (patched by `finish`) for
    /// the given opcode. Client-to-server packets carry a 4th header field
    /// (a send id set by `finish`) that the sim echoes back in
    /// `RECV_EXCEPTION.dwSendID` for correlation — this is the outbound-only
    /// header shape. Server-to-client packets don't have this field; see
    /// [`Self::new_inbound`] for building/reading those (real inbound
    /// decoding is `PacketReader::header`, which reads the 3-field shape).
    pub fn new(opcode: u32, protocol_version: u32) -> Self {
        let mut buf = Vec::with_capacity(64);
        buf.extend_from_slice(&0i32.to_le_bytes()); // size, patched in `finish`
        buf.extend_from_slice(&protocol_version.to_le_bytes());
        buf.extend_from_slice(&opcode.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes()); // send id, set by the connection layer
        Self { buf }
    }

    /// Builds a packet in the server-to-client wire shape: `size`, `version`,
    /// `id` only — no send id field. Used by tests to synthesize realistic
    /// inbound packets (the sim never sends us packets built with
    /// [`Self::new`]'s 4-field shape); pair with [`Self::finish_inbound`].
    pub fn new_inbound(opcode: u32, protocol_version: u32) -> Self {
        let mut buf = Vec::with_capacity(64);
        buf.extend_from_slice(&0i32.to_le_bytes()); // size, patched in `finish_inbound`
        buf.extend_from_slice(&protocol_version.to_le_bytes());
        buf.extend_from_slice(&opcode.to_le_bytes());
        Self { buf }
    }

    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn i32(&mut self, v: i32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn i64(&mut self, v: i64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn f32(&mut self, v: f32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn f64(&mut self, v: f64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn bytes(&mut self, v: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(v);
        self
    }

    /// Appends a fixed-width Latin-1 wire string of `width` bytes.
    pub fn fixed_str(&mut self, width: usize, s: &str) -> Result<&mut Self, FixedStringError> {
        let start = self.buf.len();
        self.buf.resize(start + width, 0);
        encode_fixed(&mut self.buf[start..], s)?;
        Ok(self)
    }

    /// Sets the send id that the connection layer allocated for this packet,
    /// and finalizes the size field. Returns the completed packet bytes.
    pub fn finish(mut self, send_id: u32) -> Vec<u8> {
        self.buf[12..16].copy_from_slice(&send_id.to_le_bytes());
        let size = self.buf.len() as i32;
        self.buf[0..4].copy_from_slice(&size.to_le_bytes());
        self.buf
    }

    /// Finalizes the size field on a [`Self::new_inbound`]-built packet.
    /// There's no send id to patch — the inbound header shape doesn't have
    /// that field.
    pub fn finish_inbound(mut self) -> Vec<u8> {
        let size = self.buf.len() as i32;
        self.buf[0..4].copy_from_slice(&size.to_le_bytes());
        self.buf
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PacketHeader {
    pub size: i32,
    pub version: u32,
    pub id: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct PacketReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooShort;

impl std::fmt::Display for TooShort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "packet buffer too short for the requested field")
    }
}

impl std::error::Error for TooShort {}

impl<'a> PacketReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Reads the server-to-client wire header: `size`, `version`, `id` — 12
    /// bytes, three fields. Confirmed against a live MSFS2024 instance:
    /// under this reading, `RECV_SIMOBJECT_DATA.define_id` and
    /// `RECV_EXCEPTION.exception` both matched the actual request/error
    /// (`NameUnrecognized` for a bogus datum name, `CreateObjectFailed` for
    /// an invalid AI object title); reading a 4th "send id" field here (as
    /// this crate did previously) shifts every subsequent field by 4 bytes
    /// and silently produces a different-but-plausible-looking wrong value
    /// instead of an error. Client-to-server packets *do* have that 4th
    /// field — see [`PacketWriter::new`] — this asymmetry matches the
    /// public SDK's `SIMCONNECT_RECV` struct, which only has these 3
    /// members; `dwSendID` only exists inside `SIMCONNECT_RECV_EXCEPTION`
    /// itself (`recv::parse_exception`), not the generic header.
    pub fn header(&mut self) -> Result<PacketHeader, TooShort> {
        Ok(PacketHeader {
            size: self.i32()?,
            version: self.u32()?,
            id: self.u32()?,
        })
    }

    /// Reads the client-to-server wire header: `size`, `version`, `id`,
    /// `send_id` — 16 bytes, four fields (see [`PacketWriter::new`]). Only
    /// meaningful for parsing a packet this crate itself just built to send
    /// — the sim never sends anything in this shape, so production
    /// inbound-decoding code should use [`Self::header`] instead. Returns
    /// `(id, send_id)`.
    pub fn outbound_header(&mut self) -> Result<(u32, u32), TooShort> {
        let header = self.header()?;
        let send_id = self.u32()?;
        Ok((header.id, send_id))
    }

    pub fn u32(&mut self) -> Result<u32, TooShort> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn i32(&mut self) -> Result<i32, TooShort> {
        let b = self.take(4)?;
        Ok(i32::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn i64(&mut self) -> Result<i64, TooShort> {
        let b = self.take(8)?;
        Ok(i64::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn f32(&mut self) -> Result<f32, TooShort> {
        let b = self.take(4)?;
        Ok(f32::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn f64(&mut self) -> Result<f64, TooShort> {
        let b = self.take(8)?;
        Ok(f64::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn u64(&mut self) -> Result<u64, TooShort> {
        let b = self.take(8)?;
        Ok(u64::from_le_bytes(b.try_into().unwrap()))
    }

    /// A `BOOL` field stored as a 32-bit int (SimConnect's usual
    /// convention for boolean fields, e.g. `IsListItem`).
    pub fn bool32(&mut self) -> Result<bool, TooShort> {
        Ok(self.u32()? != 0)
    }

    pub fn fixed_str(&mut self, width: usize) -> Result<String, TooShort> {
        let b = self.take(width)?;
        Ok(decode_fixed(b))
    }

    /// The remainder of the packet, e.g. `RECV_SIMOBJECT_DATA`'s variable
    /// tail of datum values.
    pub fn rest(&mut self) -> &'a [u8] {
        let b = &self.buf[self.pos..];
        self.pos = self.buf.len();
        b
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], TooShort> {
        if self.pos + n > self.buf.len() {
            return Err(TooShort);
        }
        let b = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(b)
    }
}
