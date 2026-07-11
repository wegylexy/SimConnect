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

/// Known SimConnect *system* event names — plain strings passed to
/// `send::subscribe_to_system_event`/`unsubscribe_from_system_event`, not new
/// opcodes or wire structures (unlike the client events above, these fire
/// unprompted whenever the sim's own state changes, not in response to a
/// `transmit_client_event` call). The `RecvEvent` a subscription produces
/// has `group_id == u32::MAX` (`SIMCONNECT_UNUSED`, confirmed against a
/// live MSFS2024 capture) and `event_id` equal to whatever id was passed to
/// `subscribe_to_system_event`, not tied to the event name string itself —
/// `event_id` is how a caller with multiple subscriptions tells them apart
/// in its `recv()` loop.
pub mod system {
    /// Fires whenever the simulation loop starts/stops running —
    /// `dwData == 1`/`0`. Confirmed against a live MSFS2024 capture to
    /// already read `1` at the ready-to-fly menu screen and unchanged
    /// during an active flight — it tracks the sim engine's own run state,
    /// not "in a flight" vs. "at a menu" as such; don't use it to
    /// distinguish those. `dwData` is otherwise undocumented as anything
    /// other than a plain 0/1, so this crate doesn't offer a decode helper
    /// (unlike [`PAUSE_EX1`]).
    pub const SIM: &str = "Sim";

    /// Legacy pause notification — `dwData == 1` while paused (of any
    /// kind), `0` otherwise. Confirmed against a live MSFS2024 capture:
    /// `dwData == 1` both for an Esc-menu pause and for toggling the
    /// in-sim play/pause icon (Active Pause), at the ready-to-fly screen
    /// and in-flight alike. Can't distinguish *which* kind of pause (menu,
    /// active pause, etc.) — see [`PAUSE_EX1`], which replaces this for
    /// that.
    ///
    /// Also confirmed to read `1` (with [`PAUSE_EX1`] `== PauseStateEx1::SIM`)
    /// at the main menu *after* having flown and returned to it, vs. `0`/
    /// `PauseStateEx1::OFF` at a genuinely fresh app launch before any
    /// flight has started — MSFS keeps that flight/world session loaded
    /// behind the main menu rather than tearing it down, and counts that
    /// as the same kind of pause as pressing Esc mid-flight, even though
    /// [`SIM`] reads `1` (running) in both menu states. The `0`/`OFF` case
    /// extends to the "Ready to fly" gate screen for a freshly-loaded new
    /// flight too (also separately confirmed live) — it's not just literal
    /// app launch that reads unpaused, it's specifically "no flight has
    /// been through its play/pause lifecycle yet".
    pub const PAUSE: &str = "Pause";

    /// Bitmask pause notification (see
    /// [`crate::enums::PauseStateEx1`] for the bit layout and what's
    /// confirmed vs. documented-but-unverified) — added because [`PAUSE`]
    /// alone can't tell an Esc-menu pause apart from other pause states.
    /// Gated behind `kittyhawk` alongside every other post-FSX addition
    /// this crate tracks, even though (unlike most of those) this is a
    /// plain string passed to the same FSX-era
    /// `subscribe_to_system_event` opcode as [`SIM`]/[`PAUSE`] — no new
    /// wire opcode or struct, just a magic string this crate hasn't
    /// independently confirmed predates MSFS2020.
    #[cfg(feature = "kittyhawk")]
    pub const PAUSE_EX1: &str = "Pause_EX1";
}
