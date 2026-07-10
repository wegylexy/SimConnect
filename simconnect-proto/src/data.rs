//! General-purpose SimConnect data-definition value types, ported from the
//! prior C# client's `Data.cs`: `Waypoint`, `LatLonAlt`, `Xyz`,
//! `MarkerState`. Each is little-endian field concatenation with no packet
//! header — these are sub-values used *inside* a data definition's raw
//! byte buffer (`RecvSimObjectData::data`, `send::set_data_on_sim_object`'s
//! `data` argument), not whole packets on their own.

use crate::codec::{PacketReader, TooShort};
use crate::strings::{encode_fixed, FixedStringError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatLonAlt {
    /// degrees
    pub latitude: f64,
    /// degrees
    pub longitude: f64,
    /// meters
    pub altitude: f64,
}

impl LatLonAlt {
    pub fn write_le(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.latitude.to_le_bytes());
        buf.extend_from_slice(&self.longitude.to_le_bytes());
        buf.extend_from_slice(&self.altitude.to_le_bytes());
    }

    pub fn read_le(r: &mut PacketReader) -> Result<Self, TooShort> {
        Ok(Self {
            latitude: r.f64()?,
            longitude: r.f64()?,
            altitude: r.f64()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    pub fn write_le(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.x.to_le_bytes());
        buf.extend_from_slice(&self.y.to_le_bytes());
        buf.extend_from_slice(&self.z.to_le_bytes());
    }

    pub fn read_le(r: &mut PacketReader) -> Result<Self, TooShort> {
        Ok(Self {
            x: r.f64()?,
            y: r.f64()?,
            z: r.f64()?,
        })
    }
}

/// `SIMCONNECT_DATA_WAYPOINT`. `flags` is a raw `u32` (`WaypointFlags` bits)
/// rather than a typed bitflags value here, to avoid a dependency from
/// this general-purpose module back onto `enums::WaypointFlags` — callers
/// can wrap/unwrap with `WaypointFlags::from_bits`/`.bits()` themselves.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Waypoint {
    /// degrees
    pub latitude: f64,
    /// degrees
    pub longitude: f64,
    /// feet
    pub altitude: f64,
    pub flags: u32,
    /// knots
    pub speed: f64,
    /// percent
    pub throttle: f64,
}

impl Waypoint {
    pub fn write_le(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.latitude.to_le_bytes());
        buf.extend_from_slice(&self.longitude.to_le_bytes());
        buf.extend_from_slice(&self.altitude.to_le_bytes());
        buf.extend_from_slice(&self.flags.to_le_bytes());
        buf.extend_from_slice(&self.speed.to_le_bytes());
        buf.extend_from_slice(&self.throttle.to_le_bytes());
    }

    pub fn read_le(r: &mut PacketReader) -> Result<Self, TooShort> {
        Ok(Self {
            latitude: r.f64()?,
            longitude: r.f64()?,
            altitude: r.f64()?,
            flags: r.u32()?,
            speed: r.f64()?,
            throttle: r.f64()?,
        })
    }
}

/// `SIMCONNECT_DATA_INITPOSITION`: initial position/attitude for the user
/// aircraft or an AI-created object (`send::ai_create_non_atc_aircraft`/
/// `ai_create_simulated_object`). Field order confirmed against the
/// official struct definition (`Latitude`/`Longitude`/`Altitude`/`Pitch`/
/// `Bank`/`Heading`/`OnGround`(`DWORD`)/`Airspeed`(`DWORD`)).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitPosition {
    /// degrees
    pub latitude: f64,
    /// degrees
    pub longitude: f64,
    /// feet
    pub altitude: f64,
    /// degrees
    pub pitch: f64,
    /// degrees
    pub bank: f64,
    /// degrees
    pub heading: f64,
    pub on_ground: bool,
    /// knots
    pub airspeed: u32,
}

impl InitPosition {
    pub fn write_le(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.latitude.to_le_bytes());
        buf.extend_from_slice(&self.longitude.to_le_bytes());
        buf.extend_from_slice(&self.altitude.to_le_bytes());
        buf.extend_from_slice(&self.pitch.to_le_bytes());
        buf.extend_from_slice(&self.bank.to_le_bytes());
        buf.extend_from_slice(&self.heading.to_le_bytes());
        buf.extend_from_slice(&(self.on_ground as u32).to_le_bytes());
        buf.extend_from_slice(&self.airspeed.to_le_bytes());
    }

    pub fn read_le(r: &mut PacketReader) -> Result<Self, TooShort> {
        Ok(Self {
            latitude: r.f64()?,
            longitude: r.f64()?,
            altitude: r.f64()?,
            pitch: r.f64()?,
            bank: r.f64()?,
            heading: r.f64()?,
            on_ground: r.bool32()?,
            airspeed: r.u32()?,
        })
    }
}

/// `SIMCONNECT_DATA_MARKERSTATE`: a fixed 64-byte Latin-1 name plus a
/// boolean state stored as a 32-bit int (SimConnect's usual `BOOL`
/// convention).
#[derive(Debug, Clone, PartialEq)]
pub struct MarkerState {
    pub name: String,
    pub state: bool,
}

impl MarkerState {
    pub fn write_le(&self, buf: &mut Vec<u8>) -> Result<(), FixedStringError> {
        let start = buf.len();
        buf.resize(start + 64, 0);
        encode_fixed(&mut buf[start..], &self.name)?;
        buf.extend_from_slice(&(self.state as u32).to_le_bytes());
        Ok(())
    }

    pub fn read_le(r: &mut PacketReader) -> Result<Self, TooShort> {
        Ok(Self {
            name: r.fixed_str(64)?,
            state: r.bool32()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::PacketReader;

    fn round_trip<T: PartialEq + std::fmt::Debug>(
        value: T,
        write: impl Fn(&T, &mut Vec<u8>),
        read: impl Fn(&mut PacketReader) -> Result<T, TooShort>,
    ) {
        let mut buf = Vec::new();
        write(&value, &mut buf);
        let mut r = PacketReader::new(&buf);
        assert_eq!(read(&mut r).unwrap(), value);
    }

    #[test]
    fn lat_lon_alt_round_trips() {
        round_trip(
            LatLonAlt {
                latitude: 47.44,
                longitude: -122.30,
                altitude: 433.0,
            },
            LatLonAlt::write_le,
            LatLonAlt::read_le,
        );
    }

    #[test]
    fn xyz_round_trips() {
        round_trip(
            Xyz {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Xyz::write_le,
            Xyz::read_le,
        );
    }

    #[test]
    fn waypoint_round_trips() {
        round_trip(
            Waypoint {
                latitude: 1.0,
                longitude: 2.0,
                altitude: 3.0,
                flags: 0x100000,
                speed: 250.0,
                throttle: 75.0,
            },
            Waypoint::write_le,
            Waypoint::read_le,
        );
    }

    #[test]
    fn init_position_round_trips() {
        round_trip(
            InitPosition {
                latitude: 47.44,
                longitude: -122.30,
                altitude: 433.0,
                pitch: 0.0,
                bank: 0.0,
                heading: 270.0,
                on_ground: true,
                airspeed: 0,
            },
            InitPosition::write_le,
            InitPosition::read_le,
        );
    }

    #[test]
    fn marker_state_round_trips() {
        let mut buf = Vec::new();
        let value = MarkerState {
            name: "Marker1".to_string(),
            state: true,
        };
        value.write_le(&mut buf).unwrap();
        assert_eq!(buf.len(), 68);
        let mut r = PacketReader::new(&buf);
        assert_eq!(MarkerState::read_le(&mut r).unwrap(), value);
    }
}
