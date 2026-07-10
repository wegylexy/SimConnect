//! Fixed-width wire strings.
//!
//! SimConnect encodes `SIMCONNECT_STRINGnn` fields as fixed-size,
//! null-terminated byte buffers. The bytes are **Latin-1** (ISO-8859-1), not
//! ASCII or UTF-8: raw title/ICAO/livery strings from the sim can contain
//! byte values above 0x7F, which ASCII would mangle. Latin-1 maps every byte
//! 0x00-0xFF to the identically-numbered Unicode scalar value, so it round-trips
//! losslessly without a dependency — this is the same fix applied to the
//! prior C# client (see the `d652be7` commit on the old `main` branch).

use std::fmt;

/// Error returned when a string can't be encoded as fixed-width Latin-1,
/// either because it contains a codepoint above U+00FF or because it (plus
/// its NUL terminator) doesn't fit in the destination buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedStringError {
    NotLatin1,
    TooLong,
}

impl fmt::Display for FixedStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotLatin1 => write!(f, "string contains a codepoint above U+00FF"),
            Self::TooLong => write!(f, "string does not fit in the fixed-width field"),
        }
    }
}

impl std::error::Error for FixedStringError {}

/// Encodes `s` as Latin-1 into `buf`, NUL-terminating and zero-padding the
/// remainder. `buf.len()` is the full wire width (e.g. 256 for `String256`).
pub fn encode_fixed(buf: &mut [u8], s: &str) -> Result<(), FixedStringError> {
    buf.fill(0);
    if s.chars().count() >= buf.len() {
        return Err(FixedStringError::TooLong);
    }
    for (i, c) in s.chars().enumerate() {
        let cp = c as u32;
        if cp > 0xFF {
            return Err(FixedStringError::NotLatin1);
        }
        buf[i] = cp as u8;
    }
    Ok(())
}

/// Decodes a Latin-1, NUL-terminated (or buffer-filling) fixed-width wire
/// string.
pub fn decode_fixed(buf: &[u8]) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    buf[..end].iter().map(|&b| b as char).collect()
}

macro_rules! fixed_string_type {
    ($name:ident, $width:expr) => {
        #[doc = concat!(
            "Fixed-width, ", stringify!($width),
            "-byte Latin-1 wire string (`SIMCONNECT_STRING", stringify!($width), "`)."
        )]
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $name(pub String);

        impl $name {
            pub const WIDTH: usize = $width;

            pub fn write_le(
                &self,
                buf: &mut Vec<u8>,
            ) -> Result<(), FixedStringError> {
                let start = buf.len();
                buf.resize(start + Self::WIDTH, 0);
                encode_fixed(&mut buf[start..], &self.0)
            }

            pub fn read_le(
                r: &mut crate::codec::PacketReader,
            ) -> Result<Self, crate::codec::TooShort> {
                Ok(Self(r.fixed_str(Self::WIDTH)?))
            }
        }
    };
}

fixed_string_type!(String8, 8);
fixed_string_type!(String32, 32);
fixed_string_type!(String64, 64);
fixed_string_type!(String128, 128);
fixed_string_type!(String256, 256);
fixed_string_type!(String260, 260);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_ascii() {
        let mut buf = [0u8; 32];
        encode_fixed(&mut buf, "N12345").unwrap();
        assert_eq!(decode_fixed(&buf), "N12345");
    }

    #[test]
    fn round_trips_high_latin1_byte() {
        let mut buf = [0u8; 8];
        encode_fixed(&mut buf, "caf\u{e9}").unwrap(); // "café"
        assert_eq!(decode_fixed(&buf), "caf\u{e9}");
    }

    #[test]
    fn rejects_string_above_latin1_range() {
        let mut buf = [0u8; 8];
        assert_eq!(
            encode_fixed(&mut buf, "\u{2708}").unwrap_err(), // airplane emoji
            FixedStringError::NotLatin1
        );
    }

    #[test]
    fn rejects_overlong_string() {
        let mut buf = [0u8; 4];
        assert_eq!(
            encode_fixed(&mut buf, "toolong").unwrap_err(),
            FixedStringError::TooLong
        );
    }

    #[test]
    fn decode_stops_at_nul_even_with_trailing_garbage() {
        let buf = [b'A', b'B', 0, b'C'];
        assert_eq!(decode_fixed(&buf), "AB");
    }

    #[test]
    fn string8_round_trips() {
        let mut buf = Vec::new();
        let value = String8("N123".to_string());
        value.write_le(&mut buf).unwrap();
        assert_eq!(buf.len(), 8);
        let mut r = crate::codec::PacketReader::new(&buf);
        assert_eq!(String8::read_le(&mut r).unwrap(), value);
    }

    #[test]
    fn string256_round_trips() {
        let mut buf = Vec::new();
        let value = String256("Cessna 172 Skyhawk".to_string());
        value.write_le(&mut buf).unwrap();
        assert_eq!(buf.len(), 256);
        let mut r = crate::codec::PacketReader::new(&buf);
        assert_eq!(String256::read_le(&mut r).unwrap(), value);
    }
}
