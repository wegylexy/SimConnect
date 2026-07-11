//! Packed binary-coded digit types used for a handful of SimConnect data
//! definitions (transponder codes, radio frequencies), ported from the prior
//! C# client's `BCO16`/`FrequencyBCD16` (`Data.cs`).
//!
//! The "16" in every one of these names is the total bit width, not a
//! digit count that happens to match: a nibble (4 bits) holds one digit
//! (decimal 0-9 for "BCD", octal 0-7 for "BCO"), and 4 nibbles × 4 bits =
//! 16 bits — so "BCD16"/"BCO16" always means exactly 4 packed digits,
//! most significant first, filling a 16-bit value. (This crate stores
//! that 16-bit value in a `u16` field, but see [`Bco16::write_le`] and
//! [`FrequencyBcd16::write_le`]: the wire format for both actually widens
//! it to 4 bytes, matching the C# originals' `int`-typed backing field.)

/// A 16-bit value with four base-10 digits packed one per nibble (standard
/// BCD), e.g. `TRANSPONDER CODE:1` reads back as decimal squawk digits
/// packed this way in some data definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bcd16(pub u16);

/// A 16-bit value with four base-8 digits packed one per nibble ("binary
/// coded octal"), used for `TRANSPONDER CODE:1` as an octal squawk code
/// (e.g. squawk 1200 is stored with nibbles 1, 2, 0, 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bco16(pub u16);

fn pack_digits(digits: [u8; 4], radix: u8) -> u16 {
    debug_assert!(digits.iter().all(|&d| d < radix));
    (digits[0] as u16) << 12 | (digits[1] as u16) << 8 | (digits[2] as u16) << 4 | digits[3] as u16
}

fn unpack_nibbles(raw: u16) -> [u8; 4] {
    [
        ((raw >> 12) & 0xF) as u8,
        ((raw >> 8) & 0xF) as u8,
        ((raw >> 4) & 0xF) as u8,
        (raw & 0xF) as u8,
    ]
}

impl Bcd16 {
    /// Packs a 0-9999 decimal value into BCD nibbles.
    pub fn from_decimal(value: u16) -> Self {
        assert!(value <= 9999, "BCD16 only holds 4 decimal digits");
        let digits = [
            (value / 1000 % 10) as u8,
            (value / 100 % 10) as u8,
            (value / 10 % 10) as u8,
            (value % 10) as u8,
        ];
        Self(pack_digits(digits, 10))
    }

    /// Unpacks back to a 0-9999 decimal value.
    pub fn to_decimal(self) -> u16 {
        let d = unpack_nibbles(self.0);
        d[0] as u16 * 1000 + d[1] as u16 * 100 + d[2] as u16 * 10 + d[3] as u16
    }
}

impl Bco16 {
    /// Packs a 4-digit octal squawk code (each digit 0-7, e.g. `1200`) into
    /// BCO nibbles.
    pub fn from_octal_digits(value: u16) -> Self {
        let digits = [
            (value / 1000 % 10) as u8,
            (value / 100 % 10) as u8,
            (value / 10 % 10) as u8,
            (value % 10) as u8,
        ];
        assert!(digits.iter().all(|&d| d < 8), "squawk digits must be 0-7");
        Self(pack_digits(digits, 8))
    }

    /// Unpacks back to the 4-digit octal squawk code, e.g. `1200`.
    pub fn to_octal_digits(self) -> u16 {
        let d = unpack_nibbles(self.0);
        d[0] as u16 * 1000 + d[1] as u16 * 100 + d[2] as u16 * 10 + d[3] as u16
    }

    /// Wire representation is 4 bytes (`Int32`), not 2 — the C# original's
    /// `BCO16.Data` field was declared `readonly int Data`, and its
    /// reflection-based data-definition registration mapped it to
    /// `DataType.Int32`/`sizeof(BCO16)` (4 bytes), even though only the
    /// low 16 bits are ever meaningful. Round-trips exactly since this
    /// type never uses more than 4 nibbles.
    pub fn write_le(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(self.0 as i32).to_le_bytes());
    }

    pub fn read_le(r: &mut crate::codec::PacketReader) -> Result<Self, crate::codec::TooShort> {
        Ok(Self(r.i32()? as u16))
    }
}

/// Packed BCD encoding of a `1xx.xx` MHz VHF COM frequency at legacy 25 kHz
/// (and coarser 50/100 kHz) channel spacing, ported from the prior C#
/// client's `FrequencyBCD16` (`Data.cs`), including its worked example
/// (118.000 MHz -> `Data == 0x1800`).
///
/// The four nibbles store the decimal digits of `(kHz / 10) % 10_000` (tens
/// of kHz), most significant first — but a frequency like 118.000 MHz, in
/// tens-of-kHz, is `11800`: **five** digits, one more than 4 nibbles hold.
/// The fix is that the leading digit is never anything *but* `1` across
/// the entire aviation VHF COM band (118.000-136.990 MHz all start with
/// "1"), so it's simply never stored — only the remaining 4 digits are
/// (`1800` for 118.000 MHz, packed as nibbles `1,8,0,0` = `0x1800`,
/// matching the worked example above), and decode adds the `1` back as an
/// implied leading `10_000` before scaling to kHz (see [`Self::to_khz`]).
/// Because 25 kHz spacing produces values not evenly divisible by 10 kHz
/// (`.025`, `.075`), the low nibble *also* doubles as a sentinel: digit
/// `2` or `7` there means "add back the 5 kHz that truncating to
/// tens-of-kHz dropped".
///
/// **This format has no room for 8.33 kHz channel spacing.** There's no
/// replacement type in this crate for that, either: reading/writing an
/// exact 8.33 kHz frequency is just a plain `f64`/`u32` Hz value through
/// the generic data-definition and event-transmit paths, not something
/// needing a packed wire type at all.
/// (An earlier version of this port tried to overload this format's spare
/// nibble values for 8.33 kHz sub-channels, then tried a dedicated
/// exact-Hz wrapper type computed from equal thirds of each 25 kHz block;
/// both are preserved in git history as worked examples of why they don't
/// fit: nibble overloading fails because several 8.33 kHz channels differ
/// from a 25 kHz channel by exactly 5 kHz while decoding to the same
/// truncated nibble, and the equal-thirds math produces frequencies like
/// 118.008333... MHz that don't match the real ICAO 8.33 kHz channel
/// plan's round displayed values, e.g. 118.005/118.010 MHz.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrequencyBcd16(pub u16);

impl FrequencyBcd16 {
    /// Encodes a frequency given in kHz (e.g. `118_000` for 118.000 MHz).
    /// Only exact multiples of 25 kHz (mod 50 kHz remainder 0 or 25) round
    /// trip losslessly; anything else silently loses its 5 kHz remainder,
    /// same as the original C# encoder.
    pub fn from_khz(khz: u32) -> Self {
        let x_full = khz / 10; // truncates the ones-of-kHz digit
                               // Drop the implied leading "1" of the 1xx.xx MHz band (see `to_khz`).
        let x = x_full % 10_000;
        let digits = [
            (x / 1000 % 10) as u8,
            (x / 100 % 10) as u8,
            (x / 10 % 10) as u8,
            (x % 10) as u8,
        ];
        Self(pack_digits(digits, 10))
    }

    pub fn from_hz(hz: u32) -> Self {
        Self::from_khz(hz / 1000)
    }

    /// Decodes back to kHz.
    ///
    /// The leading "1" of the `1xx.xx` MHz aviation VHF COM band isn't
    /// stored in the nibbles (every value in the band starts with it) — it
    /// has to be added back as an implied `10000` on `x` (tens of kHz)
    /// before scaling to kHz.
    pub fn to_khz(self) -> u32 {
        let d = unpack_nibbles(self.0);
        let x = d[0] as u32 * 1000 + d[1] as u32 * 100 + d[2] as u32 * 10 + d[3] as u32;
        let remainder = if matches!(d[3], 2 | 7) { 5 } else { 0 };
        (10_000 + x) * 10 + remainder
    }

    pub fn to_hz(self) -> u32 {
        self.to_khz() * 1000
    }

    /// Wire representation is 4 bytes (`Int32`), matching the C# original's
    /// `readonly int Data` field and its `DataType.Int32`/`sizeof
    /// (FrequencyBCD16)` (4 bytes) data-definition mapping — see
    /// [`Bco16::write_le`]'s doc for the same note.
    pub fn write_le(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(self.0 as i32).to_le_bytes());
    }

    pub fn read_le(r: &mut crate::codec::PacketReader) -> Result<Self, crate::codec::TooShort> {
        Ok(Self(r.i32()? as u16))
    }
}

// There is deliberately no `Frequency8_33Khz` (or similar wrapper) type
// here. Reading/writing an exact-Hz COM frequency (25 kHz or 8.33 kHz
// alike) is just a plain `f64`/`u32` through the generic data-definition
// path (request the datum with `Units = "Hz"` or `"MHz"` and
// `DataType::Float64` instead of `"Frequency BCD16"`) or the `_HZ` client
// events in `simconnect_proto::events` — there's no packed wire format to
// wrap. An earlier draft of this port added a dedicated type for it before
// that was confirmed; it's gone now rather than kept as unused scaffolding.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcd_round_trip() {
        assert_eq!(Bcd16::from_decimal(1234).to_decimal(), 1234);
        assert_eq!(Bcd16::from_decimal(0).to_decimal(), 0);
    }

    #[test]
    fn bco_round_trip_squawk() {
        let squawk = Bco16::from_octal_digits(1200);
        assert_eq!(squawk.to_octal_digits(), 1200);
        // Nibbles are 1, 2, 0, 0 -> 0x1200, not decimal 1200 reinterpreted.
        assert_eq!(squawk.0, 0x1200);
    }

    #[test]
    #[should_panic(expected = "squawk digits must be 0-7")]
    fn bco_rejects_digit_8_or_9() {
        Bco16::from_octal_digits(1289);
    }

    #[test]
    fn frequency_matches_prior_csharp_worked_example() {
        // The old C# client's own `ToString` comment: Data == 0x1800 for
        // 118.000 MHz (the low nibble 0 needs no correction).
        let f = FrequencyBcd16::from_khz(118_000);
        assert_eq!(f.0, 0x1800);
        assert_eq!(f.to_khz(), 118_000);
    }

    #[test]
    fn frequency_round_trips_25khz_values() {
        for khz in [118_000, 118_025, 118_050, 118_075, 127_000, 136_975] {
            let f = FrequencyBcd16::from_khz(khz);
            assert_eq!(f.to_khz(), khz, "round trip failed for {khz} kHz");
        }
    }

    #[test]
    fn frequency_hz_helpers_agree_with_khz() {
        let f = FrequencyBcd16::from_hz(127_500_000);
        assert_eq!(f.to_hz(), 127_500_000);
    }
}
