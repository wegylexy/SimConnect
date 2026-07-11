//! Known SimConnect event names — plain strings passed to
//! `send::map_client_event_to_sim_event`/`transmit_client_event`
//! (`client`)/`send::subscribe_to_system_event` (`system`), not new opcodes
//! or wire structures.
//!
//! COM radio frequency events: `client::radio_navigation::COM_RADIO_SET`/
//! `COM2_RADIO_SET`/`COM3_RADIO_SET` take a legacy 25 kHz `FrequencyBcd16`
//! -encoded value; their `_HZ` counterparts take a plain exact-Hz `u32` and
//! work regardless of the radio's spacing mode (25 kHz or 8.33 kHz) — see
//! [`crate::bcd`]'s module docs for why there's no dedicated 8.33 kHz wire
//! type: the `_HZ` events and the generic data-definition path
//! (`Units = "Hz"`/`"MHz"`) already carry exact values without one. Prefer
//! the `_HZ` variants for new code.

pub mod client;
pub mod system;
