// Autopilot
/// Toggles airspeed hold mode
pub const AP_AIRSPEED_HOLD: &str = "AP_AIRSPEED_HOLD";
/// Turns airspeed hold off
pub const AP_AIRSPEED_OFF: &str = "AP_AIRSPEED_OFF";
/// Turns airspeed hold on
pub const AP_AIRSPEED_ON: &str = "AP_AIRSPEED_ON";
/// Sets airspeed hold on/off (1,0) Parameters: \[0\]: TRUE/FALSE to enable/disable.
pub const AP_AIRSPEED_SET: &str = "AP_AIRSPEED_SET";
/// Sets the autopilot altitude to the current altitude. Parameters: AP_ALT_CURRENT_ALT_SET.
#[cfg(feature = "sunrise")]
pub const AP_ALT_CURRENT_ALT_SET: &str = "AP_ALT_CURRENT_ALT_SET";
/// Sets the heading of the autopilot of the given "slot" index to the current heading.
/// Parameters: AP_HDG_CURRENT_HDG_SET \[0\]: slot index.
#[cfg(feature = "sunrise")]
pub const AP_HDG_CURRENT_HDG_SET: &str = "AP_HDG_CURRENT_HDG_SET";
/// Decrements the reference altitude. Parameters: \[0\]: New reference altitude \[1\]: Index.
pub const AP_ALT_VAR_DEC: &str = "AP_ALT_VAR_DEC";
/// Increments the reference altitude. Parameters: \[0\]: New reference altitude \[1\]: Index.
pub const AP_ALT_VAR_INC: &str = "AP_ALT_VAR_INC";
/// Toggles altitude hold mode
pub const AP_ALT_HOLD: &str = "AP_ALT_HOLD";
/// Turns off altitude hold mode
pub const AP_ALT_HOLD_OFF: &str = "AP_ALT_HOLD_OFF";
/// Turns altitude hold mode on
pub const AP_ALT_HOLD_ON: &str = "AP_ALT_HOLD_ON";
/// Deactivate autopilot radio altitude mode.
pub const AP_ALT_RADIO_MODE_OFF: &str = "AP_ALT_RADIO_MODE_OFF";
/// Activate autopilot radio altitude mode.
pub const AP_ALT_RADIO_MODE_ON: &str = "AP_ALT_RADIO_MODE_ON";
/// Set autopilot radio altitude mode. Parameters: \[0\]: bool.
pub const AP_ALT_RADIO_MODE_SET: &str = "AP_ALT_RADIO_MODE_SET";
/// Toggle autopilot radio altitude mode.
pub const AP_ALT_RADIO_MODE_TOGGLE: &str = "AP_ALT_RADIO_MODE_TOGGLE";
/// Sets altitude reference in feet Parameters: \[0\]: New reference altitude \[1\]: Index.
pub const AP_ALT_VAR_SET_ENGLISH: &str = "AP_ALT_VAR_SET_ENGLISH";
/// Sets reference altitude in meters Parameters: \[0\]: New reference altitude \[1\]: Index.
pub const AP_ALT_VAR_SET_METRIC: &str = "AP_ALT_VAR_SET_METRIC";
/// Toggles approach hold (localizer and glide-slope)
pub const AP_APR_HOLD: &str = "AP_APR_HOLD";
/// Turns off approach hold mode
pub const AP_APR_HOLD_OFF: &str = "AP_APR_HOLD_OFF";
/// Turns both AP localizer and glide-slope modes on/armed
pub const AP_APR_HOLD_ON: &str = "AP_APR_HOLD_ON";
/// Toggle attitude hold mode
pub const AP_ATT_HOLD: &str = "AP_ATT_HOLD";
/// Turns off attitude hold mode
pub const AP_ATT_HOLD_OFF: &str = "AP_ATT_HOLD_OFF";
/// Turns on AP wing leveler and pitch hold mode
pub const AP_ATT_HOLD_ON: &str = "AP_ATT_HOLD_ON";
/// Turn off the Managed Avionics mode. This is linked to the SimVar AUTOPILOT AVIONICS
/// MANAGED.
pub const AP_AVIONICS_MANAGED_OFF: &str = "AP_AVIONICS_MANAGED_OFF";
/// Turn on the Managed Avionics mode. This is linked to the SimVar AUTOPILOT AVIONICS
/// MANAGED.
pub const AP_AVIONICS_MANAGED_ON: &str = "AP_AVIONICS_MANAGED_ON";
/// Set the autopilot managed avionics mode (TRUE for on, FALSE for off). Parameters: \[0\]:
/// TRUE/FALSE to enable/disable.
pub const AP_AVIONICS_MANAGED_SET: &str = "AP_AVIONICS_MANAGED_SET";
/// Toggle on/off the avionics managed mode on the autopilot.
pub const AP_AVIONICS_MANAGED_TOGGLE: &str = "AP_AVIONICS_MANAGED_TOGGLE";
/// Toggles the autopilot bank hold mode on / off.
pub const AP_BANK_HOLD: &str = "AP_BANK_HOLD";
/// Turns off the autopilot bank hold mode.
pub const AP_BANK_HOLD_OFF: &str = "AP_BANK_HOLD_OFF";
/// Turns on the autopilot bank hold mode.
pub const AP_BANK_HOLD_ON: &str = "AP_BANK_HOLD_ON";
/// Toggles the backcourse mode for the localizer hold
pub const AP_BC_HOLD: &str = "AP_BC_HOLD";
/// Turns off backcourse mode for localizer hold
pub const AP_BC_HOLD_OFF: &str = "AP_BC_HOLD_OFF";
/// Turns localizer back course hold mode on/armed
pub const AP_BC_HOLD_ON: &str = "AP_BC_HOLD_ON";
/// Toggles heading hold mode
pub const AP_HDG_HOLD: &str = "AP_HDG_HOLD";
/// Turns off heading hold mode
pub const AP_HDG_HOLD_OFF: &str = "AP_HDG_HOLD_OFF";
/// Turns heading hold mode on
pub const AP_HDG_HOLD_ON: &str = "AP_HDG_HOLD_ON";
/// Toggles localizer (only) hold mode
pub const AP_LOC_HOLD: &str = "AP_LOC_HOLD";
/// Turns off localizer hold mode
pub const AP_LOC_HOLD_OFF: &str = "AP_LOC_HOLD_OFF";
/// Turns AP localizer hold on/armed and glide-slope hold mode off
pub const AP_LOC_HOLD_ON: &str = "AP_LOC_HOLD_ON";
/// Decrements the reference mach by the amount set in the systems.cfg using the
/// mach_increment parameter. NOTE: along with the AP_MACH_VAR_DEC, AP_MACH_VAR_SET, and
/// AP_MACH_VAR_SET_EX1 keys, this value is clamped to to the values given in the systems.cfg
/// by the parameters min_Mach_ref and max_Mach_ref (or their default values if not set).
/// Parameters: \[0\]: the Index of the engine to target (1 - 4).
pub const AP_MACH_VAR_DEC: &str = "AP_MACH_VAR_DEC";
/// Increments the reference mach by the amount set in the systems.cfg using the
/// mach_increment parameter. NOTE: along with the AP_MACH_VAR_INC, AP_MACH_VAR_SET, and
/// AP_MACH_VAR_SET_EX1 keys, this value is clamped to to the values given in the systems.cfg
/// by the parameters min_Mach_ref and max_Mach_ref (or their default values if not set).
/// Parameters: \[0\]: Index of the engine to target (1 - 4).
pub const AP_MACH_VAR_INC: &str = "AP_MACH_VAR_INC";
/// Toggles mach hold
pub const AP_MACH_HOLD: &str = "AP_MACH_HOLD";
/// Turns mach hold off
pub const AP_MACH_OFF: &str = "AP_MACH_OFF";
/// Turns mach hold on
pub const AP_MACH_ON: &str = "AP_MACH_ON";
/// Sets mach hold on/off (1,0) Parameters: \[0\]: TRUE/FALSE to enable/disable.
pub const AP_MACH_SET: &str = "AP_MACH_SET";
/// Sets the mach reference. NOTE: along with the AP_MACH_VAR_DEC, AP_MACH_VAR_INC, and
/// AP_MACH_VAR_SET_EX1 keys, this value is clamped to to the values given in the systems.cfg
/// by the parameters min_Mach_ref and max_Mach_ref (or their default values if not set).
/// Parameters: \[0\]: Integer mach value / 100 (eg: 100 as value results as mach 1) \[1\]:
/// Index of the engine to target (1 - 4).
pub const AP_MACH_VAR_SET: &str = "AP_MACH_VAR_SET";
/// Sets the mach reference using a precise value. NOTE: along with the AP_MACH_VAR_DEC,
/// AP_MACH_VAR_INC, and AP_MACH_VAR_SET keys, this value is clamped to to the values given in
/// the systems.cfg by the parameters min_Mach_ref and max_Mach_ref (or their default values
/// if not set). Parameters: \[0\]: Integer mach value \ 1000000 (eg: 1000 as value results as
/// mach 0.001) \[1\]: Index of the engine to target (1 - 4).
pub const AP_MACH_VAR_SET_EX1: &str = "AP_MACH_VAR_SET_EX1";
/// Turns off the use of the mach value to compute airspeed used by AP.
pub const AP_MANAGED_SPEED_IN_MACH_OFF: &str = "AP_MANAGED_SPEED_IN_MACH_OFF";
/// Turns on the use of the mach value to compute airspeed used by AP.
pub const AP_MANAGED_SPEED_IN_MACH_ON: &str = "AP_MANAGED_SPEED_IN_MACH_ON";
/// Sets the use of the mach value to compute airspeed used by AP. Parameters: \[0\]: use
/// TRUE/FALSE to enabled/disable.
pub const AP_MANAGED_SPEED_IN_MACH_SET: &str = "AP_MANAGED_SPEED_IN_MACH_SET";
/// Toggle the use of the mach value to compute airspeed used by AP.
pub const AP_MANAGED_SPEED_IN_MACH_TOGGLE: &str = "AP_MANAGED_SPEED_IN_MACH_TOGGLE";
/// Toggles AP on/off
pub const AP_MASTER: &str = "AP_MASTER";
/// No longer used in the simulation. Use AP_MASTER instead.
pub const AP_MASTER_ALT: &str = "AP_MASTER_ALT";
/// Increment the AP max bank angle index. Note that if there is only one index possible then
/// using this event will do nothing. However if there are 2 or more available indices, this
/// event will increase the index by 1, and when the number passes the maximum available
/// indices - 1, it will loop back around again to index 0. For example, if you have 3
/// indices, the maximum index is 2, and incrementing past that will go to index 0. NOTE: for
/// further information on indices, please see the AP_MAX_BANK_SET event.
pub const AP_MAX_BANK_INC: &str = "AP_MAX_BANK_INC";
/// Decrement the AP max bank angle index. Note that if there is only one index possible then
/// using this event will do nothing. However if there are 2 or more available indices, this
/// event will decrease the index by 1, and when the number passes 0, it will loop back around
/// again to the max index - 1 position. For example, if there are 4 indices, and you
/// decrement from 0, the new index will be 3. NOTE: for further information on indices,
/// please see the AP_MAX_BANK_SET event.
pub const AP_MAX_BANK_DEC: &str = "AP_MAX_BANK_DEC";
/// Sets the autopilot max bank angle index to the parameter \[0\] value, where the value is
/// clamped between 0 and the number of available indices. The indices correspond to the
/// number of values set for the max_bank table, plus index 0 which corresponds to the auto
/// banking system. When auto_max_bank is enabled, setting the index to 0 will turn on the AP
/// auto banking. To give an example, if the max_bank table has 2 values and auto_max_bank is
/// enabled, then the indices for this event would be: 0: use auto banking 1: use the first
/// value in the max_bank table 2: use the second value in the max_bank table. If auto banking
/// is not enabled, then setting this to 0 will have no effect. Parameters: \[0\]: the index
/// to use for max bank angle..
pub const AP_MAX_BANK_SET: &str = "AP_MAX_BANK_SET";
/// Autopilot, hold the N1 percentage at its current level.
pub const AP_N1_HOLD: &str = "AP_N1_HOLD";
/// Decrement the autopilot N1 reference.
pub const AP_N1_REF_DEC: &str = "AP_N1_REF_DEC";
/// Increment the autopilot N1 reference.
pub const AP_N1_REF_INC: &str = "AP_N1_REF_INC";
/// Sets the autopilot N1 reference. Parameters: \[0\]: Integer N1 reference value \[1\]:
/// Index of the engine to target (1 - 4).
pub const AP_N1_REF_SET: &str = "AP_N1_REF_SET";
/// Sets the NAV (1 or 2) which is used by the Nav hold modes Parameters: \[0\]: the NAVindex
/// to use.
pub const AP_NAV_SELECT_SET: &str = "AP_NAV_SELECT_SET";
/// Toggles the nav hold mode
pub const AP_NAV1_HOLD: &str = "AP_NAV1_HOLD";
/// Turns off nav hold mode
pub const AP_NAV1_HOLD_OFF: &str = "AP_NAV1_HOLD_OFF";
/// Turns lateral hold mode on
pub const AP_NAV1_HOLD_ON: &str = "AP_NAV1_HOLD_ON";
/// Toggles altitude hold mode on/off
pub const AP_PANEL_ALTITUDE_HOLD: &str = "AP_PANEL_ALTITUDE_HOLD";
/// Turns altitude hold mode off
pub const AP_PANEL_ALTITUDE_OFF: &str = "AP_PANEL_ALTITUDE_OFF";
/// Turns altitude hold mode on (without capturing current altitude)
pub const AP_PANEL_ALTITUDE_ON: &str = "AP_PANEL_ALTITUDE_ON";
/// Sets altitude hold mode on/off (1,0) Parameters: \[0\]: TRUE/FALSE to enable/disable.
pub const AP_PANEL_ALTITUDE_SET: &str = "AP_PANEL_ALTITUDE_SET";
/// Toggles heading hold mode on/off
pub const AP_PANEL_HEADING_HOLD: &str = "AP_PANEL_HEADING_HOLD";
/// Turns heading mode off
pub const AP_PANEL_HEADING_OFF: &str = "AP_PANEL_HEADING_OFF";
/// Turns heading mode on (without capturing current heading)
pub const AP_PANEL_HEADING_ON: &str = "AP_PANEL_HEADING_ON";
/// Set heading mode on/off (1,0) Parameters: \[0\]: TRUE/FALSE to enable/disable.
pub const AP_PANEL_HEADING_SET: &str = "AP_PANEL_HEADING_SET";
/// Toggles mach hold
pub const AP_PANEL_MACH_HOLD: &str = "AP_PANEL_MACH_HOLD";
/// Turns off mach hold
pub const AP_PANEL_MACH_OFF: &str = "AP_PANEL_MACH_OFF";
/// Turns on mach hold
pub const AP_PANEL_MACH_ON: &str = "AP_PANEL_MACH_ON";
/// Sets mach hold on/off (1,0) Parameters: \[0\]: TRUE/FALSE to enable/disable.
pub const AP_PANEL_MACH_SET: &str = "AP_PANEL_MACH_SET";
/// Toggles airspeed hold mode
pub const AP_PANEL_SPEED_HOLD: &str = "AP_PANEL_SPEED_HOLD";
/// Turns off speed hold mode
pub const AP_PANEL_SPEED_OFF: &str = "AP_PANEL_SPEED_OFF";
/// Turns on speed hold mode
pub const AP_PANEL_SPEED_ON: &str = "AP_PANEL_SPEED_ON";
/// Set speed hold mode on/off (1,0) Parameters: \[0\]: TRUE/FALSE to enable/disable.
pub const AP_PANEL_SPEED_SET: &str = "AP_PANEL_SPEED_SET";
/// Turn off the AP mode that maintains a vertical speed.
pub const AP_PANEL_VS_OFF: &str = "AP_PANEL_VS_OFF";
/// Turn on the AP mode that maintains a vertical speed.
pub const AP_PANEL_VS_ON: &str = "AP_PANEL_VS_ON";
/// Enables or diables the AP mode that maintains a vertical speed. Parameters: \[0\]:
/// TRUE/FALSE to enable/disable.
pub const AP_PANEL_VS_SET: &str = "AP_PANEL_VS_SET";
/// Toggles the AP mode that maintains a vertical speed.
pub const AP_PANEL_VS_HOLD: &str = "AP_PANEL_VS_HOLD";
/// Toggles the AP mode that maintains the pitch and sets the pitch reference to 0°.
pub const AP_PITCH_LEVELER: &str = "AP_PITCH_LEVELER";
/// Turns off the AP mode that maintains the pitch and sets the pitch reference to 0°.
pub const AP_PITCH_LEVELER_OFF: &str = "AP_PITCH_LEVELER_OFF";
/// Turns on the AP mode that maintains the pitch and sets the pitch reference to 0°.
pub const AP_PITCH_LEVELER_ON: &str = "AP_PITCH_LEVELER_ON";
/// Decrements the pitch reference for pitch hold mode
pub const AP_PITCH_REF_INC_DN: &str = "AP_PITCH_REF_INC_DN";
/// Increments the pitch reference for pitch hold mode
pub const AP_PITCH_REF_INC_UP: &str = "AP_PITCH_REF_INC_UP";
/// Selects pitch reference for use with +/-
pub const AP_PITCH_REF_SELECT: &str = "AP_PITCH_REF_SELECT";
/// Sets the pitch reference value that will be maintained by the AP pitch hold mode. The
/// pitch value supplied as parameter \[0\] will be divided by 16384 before being multiplied
/// by the max_pitch value. Parameters: \[0\]: pitch value between -16384 and 16384.
pub const AP_PITCH_REF_SET: &str = "AP_PITCH_REF_SET";
/// Decrements airspeed hold reference
pub const AP_SPD_VAR_DEC: &str = "AP_SPD_VAR_DEC";
/// Increments airspeed hold reference
pub const AP_SPD_VAR_INC: &str = "AP_SPD_VAR_INC";
/// Sets airspeed reference in knots Parameters: \[0\]: value in Knots. \[1\]: the managed
/// index, from 1 to 4, or 0..
pub const AP_SPD_VAR_SET: &str = "AP_SPD_VAR_SET";
/// Set the airspeed reference, in knots, for the maintain speed AP mode. The speed supplied
/// as parameter \[0\] will be divided by 100 to give you more precision with the value, for
/// example: giving 55050 will result in an airspeed hold of 550.50 knots. For parameter
/// \[1\], giving a value of 0 instead of a single index from 1 to 4, then the speed value
/// will be copied to all managed indices. Parameters: \[0\]: value in Knots. \[1\]: the
/// managed index, from 1 to 4, or 0..
pub const AP_SPD_VAR_SET_EX1: &str = "AP_SPD_VAR_SET_EX1";
/// Toggles the AP mode that maintains a vertical speed.
pub const AP_VS_HOLD: &str = "AP_VS_HOLD";
/// Turn off the AP mode that maintains a vertical speed.
pub const AP_VS_OFF: &str = "AP_VS_OFF";
/// Turn on the AP mode that maintains a vertical speed.
pub const AP_VS_ON: &str = "AP_VS_ON";
/// Sets the AP mode that maintains a vertical speed. Parameters: \[0\]: TRUE/FALSE to
/// enable/disable.
pub const AP_VS_SET: &str = "AP_VS_SET";
/// Decrements vertical speed reference
pub const AP_VS_VAR_DEC: &str = "AP_VS_VAR_DEC";
/// Increments vertical speed reference
pub const AP_VS_VAR_INC: &str = "AP_VS_VAR_INC";
/// Sets the current managed index vertical speed reference to be the current vertical speed
/// (the current index can be set using AP_VS_SLOT_INDEX_SET). If no vertical speed indicator
/// is on the aircraft, the vertical speed reference will calculated based on the world Y-axis
/// velocity. Note that the resulting value will be clamped between the CFG parameters
/// min_vertical_speed_ref and max_vertical_speed_ref.
pub const AP_VS_VAR_SET_CURRENT: &str = "AP_VS_VAR_SET_CURRENT";
/// Sets reference vertical speed in feet per minute Parameters: \[0\]: New VS reference
/// \[1\]: Index.
pub const AP_VS_VAR_SET_ENGLISH: &str = "AP_VS_VAR_SET_ENGLISH";
/// Sets vertical speed reference in meters per minute Parameters: \[0\]: New VS reference
/// \[1\]: Index.
pub const AP_VS_VAR_SET_METRIC: &str = "AP_VS_VAR_SET_METRIC";
/// Toggles wing leveler mode
pub const AP_WING_LEVELER: &str = "AP_WING_LEVELER";
/// Turns off wing leveler mode
pub const AP_WING_LEVELER_OFF: &str = "AP_WING_LEVELER_OFF";
/// Turns wing leveler mode on
pub const AP_WING_LEVELER_ON: &str = "AP_WING_LEVELER_ON";
/// Turns airspeed hold mode on with current airspeed
pub const AP_PANEL_SPEED_HOLD_TOGGLE: &str = "AP_PANEL_SPEED_HOLD_TOGGLE";
/// Sets mach hold reference to current mach
pub const AP_PANEL_MACH_HOLD_TOGGLE: &str = "AP_PANEL_MACH_HOLD_TOGGLE";
/// Triggers both the AP_SPD_VAR_SET (with value 0) event and the AP_AIRSPEED_ON event. NOTE:
/// This is a legacy event and you should be calling the above mentioned events directly.
#[cfg(feature = "sunrise")]
pub const AP_PANEL_SPEED_ACQ_TOGGLE: &str = "AP_PANEL_SPEED_ACQ_TOGGLE";
/// Set if the AP should be disengaged or not. Parameters: \[0\]: boolean value to
/// enable/disable the disengage value.
pub const AUTOPILOT_DISENGAGE_SET: &str = "AUTOPILOT_DISENGAGE_SET";
/// Toggle the status of the AP disengage value.
pub const AUTOPILOT_DISENGAGE_TOGGLE: &str = "AUTOPILOT_DISENGAGE_TOGGLE";
/// Turns AP off
pub const AUTOPILOT_OFF: &str = "AUTOPILOT_OFF";
/// Turns AP on
pub const AUTOPILOT_ON: &str = "AUTOPILOT_ON";
/// No longer used in the simulation. Previously used by the Concorde flight model.
pub const AUTOPILOT_PANEL_CRUISE_SPEED: &str = "AUTOPILOT_PANEL_CRUISE_SPEED";
/// No longer used in the simulation. Previously used by the Concorde flight model.
pub const AUTOPILOT_PANEL_MAX_SPEED: &str = "AUTOPILOT_PANEL_MAX_SPEED";
/// Sets the index for the SimVar AUTOPILOT ALTITUDE LOCK VAR which the altitude hold mode
/// will track when captured. See alt_mode_slot_index for more information.
pub const ALTITUDE_SLOT_INDEX_SET: &str = "ALTITUDE_SLOT_INDEX_SET";
/// Toggles the autopilot FLC mode on or off. When on, the AP will adjust the engine power to
/// try and fly the aircraft at a pitch attitude corresponding to the desired flight profile
/// (climb or descent), while maintaining the airspeed reference.
pub const FLIGHT_LEVEL_CHANGE: &str = "FLIGHT_LEVEL_CHANGE";
/// Turns off the autopilot FLC mode.
pub const FLIGHT_LEVEL_CHANGE_OFF: &str = "FLIGHT_LEVEL_CHANGE_OFF";
/// Turn on the autopilot FLC mode. This mode adjusts engine power to fly the aircraft at a
/// pitch attitude corresponding to the desired flight profile (climb or descent), while
/// maintaining the airspeed reference.
pub const FLIGHT_LEVEL_CHANGE_ON: &str = "FLIGHT_LEVEL_CHANGE_ON";
/// Set autopilot heading slot index. Parameters: \[0\]: slot index from 1 to 4.
pub const HEADING_SLOT_INDEX_SET: &str = "HEADING_SLOT_INDEX_SET";
/// Sets the managed index for the RPM hold mode. Parameters: \[0\]: slot index from 1 to 4.
pub const RPM_SLOT_INDEX_SET: &str = "RPM_SLOT_INDEX_SET";
/// Sets the managed index for the speed hold mode. Parameters: \[0\]: slot index from 1 to 4.
pub const SPEED_SLOT_INDEX_SET: &str = "SPEED_SLOT_INDEX_SET";
/// Sets the managed index for the vertical speed hold mode. Parameters: \[0\]: slot index
/// from 1 to 4.
pub const VS_SLOT_INDEX_SET: &str = "VS_SLOT_INDEX_SET";

// Flight Assistance
/// Selects the airspeed reference for use with +/- Parameters: \[0\]: the reference value.
pub const AIRSPEED_BUG_SELECT: &str = "AIRSPEED_BUG_SELECT";
/// Selects the altitude reference for use with +/- Parameters: \[0\]: the reference value.
pub const ALTITUDE_BUG_SELECT: &str = "ALTITUDE_BUG_SELECT";
/// Toggles autothrottle arming mode
pub const AUTO_THROTTLE_ARM: &str = "AUTO_THROTTLE_ARM";
/// Disconnect the AutoThrottle of the AP for one engine or all engines. Parameters: \[0\]:
/// The engine to target. Use 0 to target all engines, or 1 to 16 to target a specific
/// engine..
pub const AUTO_THROTTLE_DISCONNECT: &str = "AUTO_THROTTLE_DISCONNECT";
/// Toggles Takeoff/Go Around mode
pub const AUTO_THROTTLE_TO_GA: &str = "AUTO_THROTTLE_TO_GA";
/// Sets the autobrake switch to either : the off position (position 1) when the auto_brakes
/// parameter is greater than 0. the RTO position (position 0) when the auto_brakes parameter
/// is 0.
pub const AUTOBRAKE_DISARM: &str = "AUTOBRAKE_DISARM";
/// Sets the autobrake switch to the maximum position (ie: the auto_brakes number, so that the
/// maximum braking force will be applied).
pub const AUTOBRAKE_HI_SET: &str = "AUTOBRAKE_HI_SET";
/// Sets the autobrake switch to the minimum position (position 2, the first position after
/// the off position).
pub const AUTOBRAKE_LO_SET: &str = "AUTOBRAKE_LO_SET";
/// Sets the autobrake switch to a medium position (the exact position will depend on the
/// number of autobreaks defined by the auto_brakes parameter).
pub const AUTOBRAKE_MED_SET: &str = "AUTOBRAKE_MED_SET";
/// Decrements the autobrake level by 1. When the level reaches 0, autobreaks will be off, and
/// the event will no longer decrement further.
pub const DECREASE_AUTOBRAKE_CONTROL: &str = "DECREASE_AUTOBRAKE_CONTROL";
/// Turn on or off the fly by wire Elevators and Ailerons computer.
pub const FLY_BY_WIRE_ELAC_TOGGLE: &str = "FLY_BY_WIRE_ELAC_TOGGLE";
/// Turn on or off the fly by wire Flight Augmentation computer.
pub const FLY_BY_WIRE_FAC_TOGGLE: &str = "FLY_BY_WIRE_FAC_TOGGLE";
/// Turn on or off the fly by wire Spoilers and Elevators computer.
pub const FLY_BY_WIRE_SEC_TOGGLE: &str = "FLY_BY_WIRE_SEC_TOGGLE";
/// Turn the ground proximity warning system (GPWS) on or off.
pub const GPWS_SWITCH_TOGGLE: &str = "GPWS_SWITCH_TOGGLE";
/// Decrements heading hold reference bug
pub const HEADING_BUG_DEC: &str = "HEADING_BUG_DEC";
/// Increments heading hold reference bug
pub const HEADING_BUG_INC: &str = "HEADING_BUG_INC";
/// Selects the heading bug for use with +/- Parameters: \[0\]: heading bug index.
pub const HEADING_BUG_SELECT: &str = "HEADING_BUG_SELECT";
/// Set the heading hold reference bug in degrees. The event takes integer values only, from
/// 0º to 360º. Parameters: \[0\]: Value in degrees \[1\]: Index.
pub const HEADING_BUG_SET: &str = "HEADING_BUG_SET";
/// Set the heading hold reference bug. This is the same as the HEADING_BUG_SET event only it
/// permits a much greater degree of precision by permitting the input of a larger integer
/// value that is then transformed by the simulation into a floating point heading. The way
/// this works is that the integer value you supply is multiplied by (360/16384) to give the
/// correct higher precision value, for example if you supply the value 477, you're heading
/// would be calculated like this: input value: 477 process: 477 * (360/16384) actual value:
/// 10.504º Parameters: \[0\]: Value between 0 and 16384 \[1\]: Index.
pub const AP_HEADING_BUG_SET_EX1: &str = "AP_HEADING_BUG_SET_EX1";
/// Increases the autobrake level by 1. When the level reaches the auto_brakes value, the
/// event will no longer increment further.
pub const INCREASE_AUTOBRAKE_CONTROL: &str = "INCREASE_AUTOBRAKE_CONTROL";
/// Uses the input parameter \[0\] to set the autobreak level from 0 (off) to the value set
/// for the auto_brakes parameter (maximum breaking). If a value greater than that specified
/// for auto_brakes is given as input, it will be clamped to the auto_brakes value.
/// Parameters: \[0\]: autobreak level.
pub const SET_AUTOBRAKE_CONTROL: &str = "SET_AUTOBRAKE_CONTROL";
/// Synchronizes flight director pitch with current aircraft pitch
pub const SYNC_FLIGHT_DIRECTOR_PITCH: &str = "SYNC_FLIGHT_DIRECTOR_PITCH";
/// Toggles flight director on/off
pub const TOGGLE_FLIGHT_DIRECTOR: &str = "TOGGLE_FLIGHT_DIRECTOR";
/// Toggles yaw damper on/off
pub const YAW_DAMPER_TOGGLE: &str = "YAW_DAMPER_TOGGLE";
/// Turns yaw damper on
pub const YAW_DAMPER_ON: &str = "YAW_DAMPER_ON";
/// Turns yaw damper off
pub const YAW_DAMPER_OFF: &str = "YAW_DAMPER_OFF";
/// Sets yaw damper on/off (1,0) Parameters: \[0\]: enable/disable yaw damper (TRUE, FALSE).
pub const YAW_DAMPER_SET: &str = "YAW_DAMPER_SET";
/// Selects the vertical speed reference for use with +/- Parameters: \[0\]: reference value.
pub const VSI_BUG_SELECT: &str = "VSI_BUG_SELECT";

// G1000 (Multi-Function Display)
/// Clears the current input.
pub const G1000_MFD_CLEAR_BUTTON: &str = "G1000_MFD_CLEAR_BUTTON";
/// Toggles on or off a screen cursor.
pub const G1000_MFD_CURSOR_BUTTON: &str = "G1000_MFD_CURSOR_BUTTON";
/// Turn to the Direct To page.
pub const G1000_MFD_DIRECTTO_BUTTON: &str = "G1000_MFD_DIRECTTO_BUTTON";
/// Enters the current input.
pub const G1000_MFD_ENTER_BUTTON: &str = "G1000_MFD_ENTER_BUTTON";
/// The multi-function display (MFD) should display its current flight plan.
pub const G1000_MFD_FLIGHTPLAN_BUTTON: &str = "G1000_MFD_FLIGHTPLAN_BUTTON";
/// Step down through the page groups.
pub const G1000_MFD_GROUP_KNOB_DEC: &str = "G1000_MFD_GROUP_KNOB_DEC";
/// Step up through the page groups.
pub const G1000_MFD_GROUP_KNOB_INC: &str = "G1000_MFD_GROUP_KNOB_INC";
/// If a segmented flight plan is highlighted, activates the associated menu.
pub const G1000_MFD_MENU_BUTTON: &str = "G1000_MFD_MENU_BUTTON";
/// Step down through the individual pages.
pub const G1000_MFD_PAGE_KNOB_DEC: &str = "G1000_MFD_PAGE_KNOB_DEC";
/// Step up through the individual pages.
pub const G1000_MFD_PAGE_KNOB_INC: &str = "G1000_MFD_PAGE_KNOB_INC";
/// Turn to the Procedure page.
pub const G1000_MFD_PROCEDURE_BUTTON: &str = "G1000_MFD_PROCEDURE_BUTTON";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY1: &str = "G1000_MFD_SOFTKEY1";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY2: &str = "G1000_MFD_SOFTKEY2";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY3: &str = "G1000_MFD_SOFTKEY3";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY4: &str = "G1000_MFD_SOFTKEY4";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY5: &str = "G1000_MFD_SOFTKEY5";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY6: &str = "G1000_MFD_SOFTKEY6";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY7: &str = "G1000_MFD_SOFTKEY7";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY8: &str = "G1000_MFD_SOFTKEY8";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY9: &str = "G1000_MFD_SOFTKEY9";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY10: &str = "G1000_MFD_SOFTKEY10";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY11: &str = "G1000_MFD_SOFTKEY11";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_MFD_SOFTKEY12: &str = "G1000_MFD_SOFTKEY12";
/// Zoom in on the current map.
pub const G1000_MFD_ZOOMIN_BUTTON: &str = "G1000_MFD_ZOOMIN_BUTTON";
/// Zoom out on the current map.
pub const G1000_MFD_ZOOMOUT_BUTTON: &str = "G1000_MFD_ZOOMOUT_BUTTON";
/// Clears the current input.
pub const G1000_PFD_CLEAR_BUTTON: &str = "G1000_PFD_CLEAR_BUTTON";
/// Turns on or off a screen cursor.
pub const G1000_PFD_CURSOR_BUTTON: &str = "G1000_PFD_CURSOR_BUTTON";
/// Turn to the Direct To page.
pub const G1000_PFD_DIRECTTO_BUTTON: &str = "G1000_PFD_DIRECTTO_BUTTON";
/// Enters the current input.
pub const G1000_PFD_ENTER_BUTTON: &str = "G1000_PFD_ENTER_BUTTON";
/// The primary flight display (PFD) should display its current flight plan.
pub const G1000_PFD_FLIGHTPLAN_BUTTON: &str = "G1000_PFD_FLIGHTPLAN_BUTTON";
/// Step up through the page groups.
pub const G1000_PFD_GROUP_KNOB_INC: &str = "G1000_PFD_GROUP_KNOB_INC";
/// Step down through the page groups.
pub const G1000_PFD_GROUP_KNOB_DEC: &str = "G1000_PFD_GROUP_KNOB_DEC";
/// If a segmented flight plan is highlighted, activates the associated menu.
pub const G1000_PFD_MENU_BUTTON: &str = "G1000_PFD_MENU_BUTTON";
/// Step up through the individual pages.
pub const G1000_PFD_PAGE_KNOB_INC: &str = "G1000_PFD_PAGE_KNOB_INC";
/// Step down through the individual pages.
pub const G1000_PFD_PAGE_KNOB_DEC: &str = "G1000_PFD_PAGE_KNOB_DEC";
/// Turn to the Procedure page.
pub const G1000_PFD_PROCEDURE_BUTTON: &str = "G1000_PFD_PROCEDURE_BUTTON";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY1: &str = "G1000_PFD_SOFTKEY1";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY2: &str = "G1000_PFD_SOFTKEY2";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY3: &str = "G1000_PFD_SOFTKEY3";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY4: &str = "G1000_PFD_SOFTKEY4";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY5: &str = "G1000_PFD_SOFTKEY5";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY6: &str = "G1000_PFD_SOFTKEY6";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY7: &str = "G1000_PFD_SOFTKEY7";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY8: &str = "G1000_PFD_SOFTKEY8";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY9: &str = "G1000_PFD_SOFTKEY9";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY10: &str = "G1000_PFD_SOFTKEY10";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY11: &str = "G1000_PFD_SOFTKEY11";
/// Initiate the action for the icon displayed in the softkey position.
pub const G1000_PFD_SOFTKEY12: &str = "G1000_PFD_SOFTKEY12";
/// Zoom in on the current map.
pub const G1000_PFD_ZOOMIN_BUTTON: &str = "G1000_PFD_ZOOMIN_BUTTON";
/// Zoom out on the current map.
pub const G1000_PFD_ZOOMOUT_BUTTON: &str = "G1000_PFD_ZOOMOUT_BUTTON";

// Virtual Copilot
/// Triggers action noted in Flying Tips
pub const VIRTUAL_COPILOT_ACTION: &str = "VIRTUAL_COPILOT_ACTION";
/// Sets Flying Tips on/off (1,0) Parameters: \[0\]: Enable or disable (TRUE/FALSE).
pub const VIRTUAL_COPILOT_SET: &str = "VIRTUAL_COPILOT_SET";
/// Turns Flying Tips on/off
pub const VIRTUAL_COPILOT_TOGGLE: &str = "VIRTUAL_COPILOT_TOGGLE";

// G-Limiter
/// (no description provided by the vendor docs)
pub const G_LIMITER_OFF: &str = "G_LIMITER_OFF";
/// (no description provided by the vendor docs)
pub const G_LIMITER_ON: &str = "G_LIMITER_ON";
/// (no description provided by the vendor docs)
pub const G_LIMITER_SET: &str = "G_LIMITER_SET";
/// (no description provided by the vendor docs)
pub const G_LIMITER_TOGGLE: &str = "G_LIMITER_TOGGLE";
