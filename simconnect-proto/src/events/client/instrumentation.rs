// Aircraft Instruments
/// Decrements the attitude indicator pitch reference bars. If you hold down the key for more
/// than 2 seconds it will decrement by 30, if you hold down the key for more than 1 second it
/// will decrement by 10, otherwise it will decrement by 5.
pub const ATTITUDE_BARS_POSITION_DOWN: &str = "ATTITUDE_BARS_POSITION_DOWN";
/// Increments the attitude indicator pitch reference bars. If you hold down the key for more
/// than 2 seconds it will increment by 30, if you hold down the key for more than 1 second it
/// will increment by 10, otherwise it will increment by 5.
pub const ATTITUDE_BARS_POSITION_UP: &str = "ATTITUDE_BARS_POSITION_UP";
/// Sets attitude indicator pitch reference bars. Input value can be between -100 and 100.
/// Parameters: \[0\]: Index \[1\]: Value.
pub const ATTITUDE_BARS_POSITION_SET: &str = "ATTITUDE_BARS_POSITION_SET";
/// Syncs altimeter setting to sea level pressure, or 29.92 if above 18000 feet.
pub const BAROMETRIC: &str = "BAROMETRIC";
/// Set the altimeter setting to the pressure at the standard atmospheric level (1013.25 hPa).
/// Parameters: \[0\]: the index of the altimeter.
pub const BAROMETRIC_STD_PRESSURE: &str = "BAROMETRIC_STD_PRESSURE";
/// Enables a keystroke to be sent to a gauge that is in focus. The keystrokes can only be in
/// the range 0 to 9, A to Z, and the four keys: plus, minus, comma and period. This is
/// typically used to allow some keyboard entry to a complex device such as a GPS to enter
/// such things as ICAO codes using the keyboard, rather than turning dials.
pub const GAUGE_KEYSTROKE: &str = "GAUGE_KEYSTROKE";
/// Decrements heading indicator.
pub const GYRO_DRIFT_DEC: &str = "GYRO_DRIFT_DEC";
/// Increments heading indicator.
pub const GYRO_DRIFT_INC: &str = "GYRO_DRIFT_INC";
/// Sets heading indicator drift angle (degrees). Parameters: \[0\]: Drift angle (degrees).
pub const GYRO_DRIFT_SET: &str = "GYRO_DRIFT_SET";
/// Sets heading indicator drift angle (16K). Parameters: \[0\]: Drift angle (0 - 16383).
#[cfg(feature = "sunrise")]
pub const GYRO_DRIFT_SET_EX1: &str = "GYRO_DRIFT_SET_EX1";
/// Sets heading indicator to 0 drift error.
pub const HEADING_GYRO_SET: &str = "HEADING_GYRO_SET";
/// (no description provided by the vendor docs)
pub const INDUCTOR_COMPASS_REF_INC: &str = "INDUCTOR_COMPASS_REF_INC";
/// (no description provided by the vendor docs)
pub const INDUCTOR_COMPASS_REF_DEC: &str = "INDUCTOR_COMPASS_REF_DEC";
/// Increments altimeter setting.
pub const KOHLSMAN_INC: &str = "KOHLSMAN_INC";
/// Decrements altimeter setting.
pub const KOHLSMAN_DEC: &str = "KOHLSMAN_DEC";
/// Sets altimeter setting (Millibars * 16). Parameters: \[0\]: Value to set \[1\]: Altimeter
/// index.
pub const KOHLSMAN_SET: &str = "KOHLSMAN_SET";
/// Resets max/min indicated G force to 1.0.
pub const RESET_G_FORCE_INDICATOR: &str = "RESET_G_FORCE_INDICATOR";
/// Reset max indicated engine rpm to 0.
pub const RESET_MAX_RPM_INDICATOR: &str = "RESET_MAX_RPM_INDICATOR";
/// Cages attitude indicator at 0 pitch and bank.
pub const ATTITUDE_CAGE_BUTTON: &str = "ATTITUDE_CAGE_BUTTON";
/// Toggle the InterCom system on the audio panel.
pub const TOGGLE_ICS: &str = "TOGGLE_ICS";
/// (no description provided by the vendor docs)
pub const TOGGLE_SPEAKER: &str = "TOGGLE_SPEAKER";
/// Turn the turn indicator on or off.
pub const TOGGLE_TURN_INDICATOR_SWITCH: &str = "TOGGLE_TURN_INDICATOR_SWITCH";
/// Turn the variometer on or off.
pub const TOGGLE_VARIOMETER_SWITCH: &str = "TOGGLE_VARIOMETER_SWITCH";
/// Decrements airspeed indicators true airspeed reference card.
pub const TRUE_AIRSPEED_CAL_DEC: &str = "TRUE_AIRSPEED_CAL_DEC";
/// Increments airspeed indicators true airspeed reference card.
pub const TRUE_AIRSPEED_CAL_INC: &str = "TRUE_AIRSPEED_CAL_INC";
/// Sets airspeed indicators true airspeed reference card (degrees, where 0 is standard sea
/// level conditions). Parameters: \[0\]: Degrees.
pub const TRUE_AIRSPEED_CAL_SET: &str = "TRUE_AIRSPEED_CAL_SET";
/// Toggle the variometer sounds between on (1) and off (0). No longer used in the simulation.
pub const VARIOMETER_SOUND_TOGGLE: &str = "VARIOMETER_SOUND_TOGGLE";

// ATC
/// Activates the ATC window.
pub const ATC: &str = "ATC";
/// Closes the ATC menu screen.
pub const ATC_MENU_CLOSE: &str = "ATC_MENU_CLOSE";
/// Opens the ATC menu screen.
pub const ATC_MENU_OPEN: &str = "ATC_MENU_OPEN";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_0: &str = "ATC_MENU_0";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_1: &str = "ATC_MENU_1";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_2: &str = "ATC_MENU_2";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_3: &str = "ATC_MENU_3";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_4: &str = "ATC_MENU_4";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_5: &str = "ATC_MENU_5";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_6: &str = "ATC_MENU_6";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_7: &str = "ATC_MENU_7";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_8: &str = "ATC_MENU_8";
/// Selects ATC option 0 - 9.
pub const ATC_MENU_9: &str = "ATC_MENU_9";

// EGT
/// Selects EGT bug for +/-. Follow by SELECT_1, SELECT_2, SELECT_3, or SELECT_4 to target the
/// appropriate bug.
pub const EGT: &str = "EGT";
/// Decrements all EGT bugs.
pub const EGT_DEC: &str = "EGT_DEC";
/// Increments all EGT bugs.
pub const EGT_INC: &str = "EGT_INC";
/// Sets all EGT bugs to the given value. Input will be normalised between 0 and 1.
/// Parameters: \[0\]: Bug value (0 to 32768).
pub const EGT_SET: &str = "EGT_SET";
/// Decrements the specific EGT 1/2/3/4 bug value. Parameters: EGT1_DEC EGT2_DEC EGT3_DEC
/// EGT4_DEC.
pub const EGT1_DEC: &str = "EGT1_DEC";
/// Decrements the specific EGT 1/2/3/4 bug value. Parameters: EGT1_DEC EGT2_DEC EGT3_DEC
/// EGT4_DEC.
pub const EGT2_DEC: &str = "EGT2_DEC";
/// Decrements the specific EGT 1/2/3/4 bug value. Parameters: EGT1_DEC EGT2_DEC EGT3_DEC
/// EGT4_DEC.
pub const EGT3_DEC: &str = "EGT3_DEC";
/// Decrements the specific EGT 1/2/3/4 bug value. Parameters: EGT1_DEC EGT2_DEC EGT3_DEC
/// EGT4_DEC.
pub const EGT4_DEC: &str = "EGT4_DEC";
/// Increments the specific EGT 1/2/3/4 bug value. Parameters: EGT1_INC EGT2_INC EGT3_INC
/// EGT4_INC.
pub const EGT1_INC: &str = "EGT1_INC";
/// Increments the specific EGT 1/2/3/4 bug value. Parameters: EGT1_INC EGT2_INC EGT3_INC
/// EGT4_INC.
pub const EGT2_INC: &str = "EGT2_INC";
/// Increments the specific EGT 1/2/3/4 bug value. Parameters: EGT1_INC EGT2_INC EGT3_INC
/// EGT4_INC.
pub const EGT3_INC: &str = "EGT3_INC";
/// Increments the specific EGT 1/2/3/4 bug value. Parameters: EGT1_INC EGT2_INC EGT3_INC
/// EGT4_INC.
pub const EGT4_INC: &str = "EGT4_INC";
/// Sets the specific EGT 1/2/3/4 bug value. Parameters: EGT1_SET EGT2_SET EGT3_SET EGT4_SET
/// \[0\]: Bug value (0 to 32768).
pub const EGT1_SET: &str = "EGT1_SET";
/// Sets the specific EGT 1/2/3/4 bug value. Parameters: EGT1_SET EGT2_SET EGT3_SET EGT4_SET
/// \[0\]: Bug value (0 to 32768).
pub const EGT2_SET: &str = "EGT2_SET";
/// Sets the specific EGT 1/2/3/4 bug value. Parameters: EGT1_SET EGT2_SET EGT3_SET EGT4_SET
/// \[0\]: Bug value (0 to 32768).
pub const EGT3_SET: &str = "EGT3_SET";
/// Sets the specific EGT 1/2/3/4 bug value. Parameters: EGT1_SET EGT2_SET EGT3_SET EGT4_SET
/// \[0\]: Bug value (0 to 32768).
pub const EGT4_SET: &str = "EGT4_SET";
