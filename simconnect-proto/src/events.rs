//! Known SimConnect client event *names* — plain strings passed to
//! `send::map_client_event_to_sim_event`/`transmit_client_event`, not new
//! opcodes or wire structures. Listed here so callers don't have to
//! rediscover them.
//!
//! COM radio frequency events: `COM_RADIO_SET`/`COM2_RADIO_SET`/
//! `COM3_RADIO_SET` take a legacy 25 kHz `FrequencyBcd16`-encoded value;
//! their `_HZ` counterparts take a plain exact-Hz `u32` and work
//! regardless of the radio's spacing mode (25 kHz or 8.33 kHz) — see
//! [`crate::bcd`]'s module docs for why there's no dedicated 8.33 kHz
//! wire type: the `_HZ` events and the generic data-definition path
//! (`Units = "Hz"`/`"MHz"`) already carry exact values without one.

/// COM1, legacy 25 kHz BCD16-encoded frequency.
pub const COM_RADIO_SET: &str = "COM_RADIO_SET";
/// COM1, exact Hz. Prefer this over `COM_RADIO_SET` for new code — it
/// works for both 25 kHz and 8.33 kHz-spaced radios.
pub const COM_RADIO_SET_HZ: &str = "COM_RADIO_SET_HZ";
/// COM2, legacy 25 kHz BCD16-encoded frequency.
pub const COM2_RADIO_SET: &str = "COM2_RADIO_SET";
/// COM2, exact Hz.
pub const COM2_RADIO_SET_HZ: &str = "COM2_RADIO_SET_HZ";
/// COM3, legacy 25 kHz BCD16-encoded frequency.
pub const COM3_RADIO_SET: &str = "COM3_RADIO_SET";
/// COM3, exact Hz.
pub const COM3_RADIO_SET_HZ: &str = "COM3_RADIO_SET_HZ";
