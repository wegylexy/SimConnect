// Ailerons
/// Centers aileron position. Note that this is simply an alias for the CENTER_AILER_RUDDER
/// event. Not currently used in the simulation.
pub const AILERON_CENTER: &str = "AILERON_CENTER";
/// Increments the left aileron by 1°. Note that this is simply an alias for the AILERONS_LEFT
/// event.
pub const AILERON_LEFT: &str = "AILERON_LEFT";
/// Increments the right aileron by 1°. Note that this is simply an alias for the
/// AILERONS_RIGHT event.
pub const AILERON_RIGHT: &str = "AILERON_RIGHT";
/// Sets the aileron position. Parameters: \[0\] Position (-16383 to 16384).
pub const AILERON_SET: &str = "AILERON_SET";
/// Enable (1, TRUE) or disable (0, FALSE) the aileron trim. Parameters: \[0\]: Bool.
pub const AILERON_TRIM_DISABLED_SET: &str = "AILERON_TRIM_DISABLED_SET";
/// Toggle the aileron trim disabled option between on (1) and off (0).
pub const AILERON_TRIM_DISABLED_TOGGLE: &str = "AILERON_TRIM_DISABLED_TOGGLE";
/// Increments the left aileron trim by 0.001.
pub const AILERON_TRIM_LEFT: &str = "AILERON_TRIM_LEFT";
/// Increments the right aileron trim by 0.001.
pub const AILERON_TRIM_RIGHT: &str = "AILERON_TRIM_RIGHT";
/// Sets the aileron trim. Parameters: \[0\] Position (-100 to 100).
pub const AILERON_TRIM_SET: &str = "AILERON_TRIM_SET";
/// Sets the aileron trim with extra precision. Parameters: \[0\] Position (-16383 to 16384).
pub const AILERON_TRIM_SET_EX1: &str = "AILERON_TRIM_SET_EX1";
/// Increments the left ailerons by 1°.
pub const AILERONS_LEFT: &str = "AILERONS_LEFT";
/// Increments the right ailerons by 1°.
pub const AILERONS_RIGHT: &str = "AILERONS_RIGHT";
/// Sets the aileron position. Parameters: \[0\] Position (-16383 to 16384).
pub const AXIS_AILERONS_SET: &str = "AXIS_AILERONS_SET";
/// Sets the aileron position. Note that this is simply an alias for the AILERON_SET event to
/// be used when gyro controls are enabled. Parameters: \[0\] Position (-16383 to 16384).
#[cfg(feature = "sunrise")]
pub const AXIS_SENSOR_AILERONS_SET: &str = "AXIS_SENSOR_AILERONS_SET";
/// Centers the aileron and rudder positions.
pub const CENTER_AILER_RUDDER: &str = "CENTER_AILER_RUDDER";

// Elevators
/// Sets the elevator trim position (input will be normalised to a value between -1 and 1).
/// Parameters: \[0\]: Trim position (-16383 to 16384).
pub const AXIS_ELEV_TRIM_SET: &str = "AXIS_ELEV_TRIM_SET";
/// Sets the elevator position (input will be normalised to a value between -1 and 1). Note
/// that this is simply an alias for the ELEVATOR_SET event, and will not work if gyro
/// controls are active. Parameters: \[0\]: Position (-16383 to 16384).
pub const AXIS_ELEVATOR_SET: &str = "AXIS_ELEVATOR_SET";
/// Sets the elevator position (input will be normalised to a value between -1 and 1). Note
/// that this is simply an alias for the ELEVATOR_SET event and will only work when gyro
/// controls are enabled. Parameters: \[0\]: Position (-16383 to 16384).
#[cfg(feature = "sunrise")]
pub const AXIS_SENSOR_ELEVATOR_SET: &str = "AXIS_SENSOR_ELEVATOR_SET";
/// Decrements the elevator by -0.05 (to a minimum of -1). When the key is released the
/// elevator will return to it's original position. Note that this is simply an alias for the
/// ELEVATOR_DOWN event.
pub const ELEV_DOWN: &str = "ELEV_DOWN";
/// Decrements the elevator trim by -0.0005. Holding down the key will cause the trim to
/// decrement faster over time.
pub const ELEV_TRIM_DN: &str = "ELEV_TRIM_DN";
/// Increments the elevator trim by 0.0005. Holding down the key will cause the trim to
/// increment faster over time.
pub const ELEV_TRIM_UP: &str = "ELEV_TRIM_UP";
/// Increments elevator by 0.05 (to a maximum of 1). When the key is released the elevator
/// will return to it's original position. Note that this is simply an alias for the
/// ELEVATOR_UP event.
pub const ELEV_UP: &str = "ELEV_UP";
/// Decrements the elevator by -0.05 (to a minimum of -1). When the key is released the
/// elevator will return to it's original position.
pub const ELEVATOR_DOWN: &str = "ELEVATOR_DOWN";
/// Sets elevator position (input will be normalised to a value between -1 and 1). Note that
/// this willnot work if gyro controls are active. Parameters: \[0\]: Position (-16383 to
/// 16384).
pub const ELEVATOR_SET: &str = "ELEVATOR_SET";
/// Sets the Elevator Trim Disabled to be on (TRUE) or off (FALSE). Parameters: \[0\]: Bool.
pub const ELEVATOR_TRIM_DISABLED_SET: &str = "ELEVATOR_TRIM_DISABLED_SET";
/// Toggles the Elevator Trim Disabled between on (1, TRUE) and off (0, FALSE).
pub const ELEVATOR_TRIM_DISABLED_TOGGLE: &str = "ELEVATOR_TRIM_DISABLED_TOGGLE";
/// Sets the elevator trim position. Parameters: \[0\]: Trim position (-16383 to 16384).
pub const ELEVATOR_TRIM_SET: &str = "ELEVATOR_TRIM_SET";
/// Increments the elevator by 0.05 (to a maximum of 1). When the key is released the elevator
/// will return to it's original position.
pub const ELEVATOR_UP: &str = "ELEVATOR_UP";

// Flaps
/// Sets flap handle to closest increment (-16383 - +16383) Parameters: \[0\]: Position
/// (-16383 to 16384).
pub const AXIS_FLAPS_SET: &str = "AXIS_FLAPS_SET";
/// Sets flap handle to first extension position
pub const FLAPS_1: &str = "FLAPS_1";
/// Sets flap handle to second extension position
pub const FLAPS_2: &str = "FLAPS_2";
/// Sets flap handle to third extension position
pub const FLAPS_3: &str = "FLAPS_3";
/// Sets flap handle to fourth extension position
pub const FLAPS_4: &str = "FLAPS_4";
/// Decrement flap handle by a value (0 to 16383). Parameters: \[0\]: Position (0 to 16383).
pub const FLAPS_CONTINUOUS_DECR: &str = "FLAPS_CONTINUOUS_DECR";
/// Increment flap handle by a value (0 to 16383). Parameters: \[0\]: Position (0 to 16383).
pub const FLAPS_CONTINUOUS_INCR: &str = "FLAPS_CONTINUOUS_INCR";
/// Set flap handle to any value (0 to 16383). Parameters: \[0\]: Position (0 to 16383).
pub const FLAPS_CONTINUOUS_SET: &str = "FLAPS_CONTINUOUS_SET";
/// Decrements flap handle position by one level.
pub const FLAPS_DECR: &str = "FLAPS_DECR";
/// No longer used in the simulation.
#[cfg(feature = "sunrise")]
pub const FLAPS_DETENTS_SET: &str = "FLAPS_DETENTS_SET";
/// Sets flap handle to full extension position.
pub const FLAPS_DOWN: &str = "FLAPS_DOWN";
/// Increments flap handle position by one level.
pub const FLAPS_INCR: &str = "FLAPS_INCR";
/// Set flap handle to closest increment (0 to 100%). Parameters: \[0\]: Position (0 to
/// 16383).
pub const FLAPS_SET: &str = "FLAPS_SET";
/// Sets flap handle to full retract position
pub const FLAPS_UP: &str = "FLAPS_UP";
/// Check to see if NT 361 Flight Trainer should be centered
pub const CENTER_NT361_CHECK: &str = "CENTER_NT361_CHECK";
/// CH Virtual Pilot Pro up - left hat keypress.
pub const CHVPP_LEFT_HAT_UP: &str = "CHVPP_LEFT_HAT_UP";
/// CH Virtual Pilot Pro down - left hat keypress.
pub const CHVPP_LEFT_HAT_DOWN: &str = "CHVPP_LEFT_HAT_DOWN";
/// CH Virtual Pilot Pro altitude hold and wing level.
pub const CHVPP_AP_ALT_WING: &str = "CHVPP_AP_ALT_WING";
/// No longer used in the simulation.
pub const MOUSE_AS_YOKE_RESUME: &str = "MOUSE_AS_YOKE_RESUME";
/// No longer used in the simulation.
pub const MOUSE_AS_YOKE_SUSPEND: &str = "MOUSE_AS_YOKE_SUSPEND";
/// No longer used in the simulation.
pub const MOUSE_AS_YOKE_TOGGLE: &str = "MOUSE_AS_YOKE_TOGGLE";

// Rudder
/// Turn the automatic rudder control feature on or off.
pub const AUTORUDDER_TOGGLE: &str = "AUTORUDDER_TOGGLE";
/// Sets rudder position. Parameters: \[0\]: Position (-16383 to 16384).
pub const AXIS_RUDDER_SET: &str = "AXIS_RUDDER_SET";
/// Move the rudder axis to yaw the aircraft left.
pub const RUDDER_AXIS_MINUS: &str = "RUDDER_AXIS_MINUS";
/// Move the rudder axis to yaw the aircraft right.
pub const RUDDER_AXIS_PLUS: &str = "RUDDER_AXIS_PLUS";
/// Centers rudder position
pub const RUDDER_CENTER: &str = "RUDDER_CENTER";
/// Increments rudder left
pub const RUDDER_LEFT: &str = "RUDDER_LEFT";
/// Increments rudder right
pub const RUDDER_RIGHT: &str = "RUDDER_RIGHT";
/// Sets rudder position. Note that if gyro controls are enabled, this will not work.
/// Parameters: \[0\]: Position (-16383 to 16384).
pub const RUDDER_SET: &str = "RUDDER_SET";
/// Sets the rudder position (input will be normalised to a value between -1 and 1). Note that
/// this is simply an alias for the RUDDER_SET event, and will only be used when gyro controls
/// are enabled. Parameters: \[0\]: Position (-16383 to 16384).
#[cfg(feature = "sunrise")]
pub const AXIS_SENSOR_RUDDER_SET: &str = "AXIS_SENSOR_RUDDER_SET";
/// Enables (TRUE) or disables (FALSE) the rudder trim. Parameters: \[0\]: Bool.
pub const RUDDER_TRIM_DISABLED_SET: &str = "RUDDER_TRIM_DISABLED_SET";
/// Toggles the rudder trim on (TRUE) or off (FALSE).
pub const RUDDER_TRIM_DISABLED_TOGGLE: &str = "RUDDER_TRIM_DISABLED_TOGGLE";
/// Increments rudder trim left
pub const RUDDER_TRIM_LEFT: &str = "RUDDER_TRIM_LEFT";
/// Reset the rudder trim.
pub const RUDDER_TRIM_RESET: &str = "RUDDER_TRIM_RESET";
/// Increments rudder trim right
pub const RUDDER_TRIM_RIGHT: &str = "RUDDER_TRIM_RIGHT";
/// Sets the rudder trim value, between -100 and 100. Parameters: \[0\]: Value.
pub const RUDDER_TRIM_SET: &str = "RUDDER_TRIM_SET";
/// Sets the rudder trim value, between -16383 and 16383. Parameters: \[0\]: Value.
pub const RUDDER_TRIM_SET_EX1: &str = "RUDDER_TRIM_SET_EX1";

// Slew
/// While in Slew mode, control the Z axis translation. Parameters: \[0\]: Position (-16383 to
/// 16384).
pub const AXIS_SLEW_AHEAD_SET: &str = "AXIS_SLEW_AHEAD_SET";
/// While in Slew mode, control the Y axis translation. Parameters: \[0\]: Position (-16383 to
/// 16384).
pub const AXIS_SLEW_ALT_SET: &str = "AXIS_SLEW_ALT_SET";
/// While in Slew mode, control the roll axis. Parameters: \[0\]: Position (-16383 to 16384).
pub const AXIS_SLEW_BANK_SET: &str = "AXIS_SLEW_BANK_SET";
/// While in Slew mode, control the yaw axis. Parameters: \[0\]: Position (-16383 to 16384).
pub const AXIS_SLEW_HEADING_SET: &str = "AXIS_SLEW_HEADING_SET";
/// While in Slew mode, control the pitch axis. Parameters: \[0\]: Position (-16383 to 16384).
pub const AXIS_SLEW_PITCH_SET: &str = "AXIS_SLEW_PITCH_SET";
/// While in Slew mode, control the X axis translation. Parameters: \[0\]: Position (-16383 to
/// 16384).
pub const AXIS_SLEW_SIDEWAYS_SET: &str = "AXIS_SLEW_SIDEWAYS_SET";
/// While in Slew mode, move the plane backwards.
pub const SLEW_AHEAD_MINUS: &str = "SLEW_AHEAD_MINUS";
/// While in Slew mode, move the plane forward.
pub const SLEW_AHEAD_PLUS: &str = "SLEW_AHEAD_PLUS";
/// While in Slew mode, move the plane down (fast).
pub const SLEW_ALTIT_DN_FAST: &str = "SLEW_ALTIT_DN_FAST";
/// While in Slew mode, move the plane down (slow).
pub const SLEW_ALTIT_DN_SLOW: &str = "SLEW_ALTIT_DN_SLOW";
/// While in Slew mode, stops the translation on the Y axis.
pub const SLEW_ALTIT_FREEZE: &str = "SLEW_ALTIT_FREEZE";
/// While in Slew mode, move the plane down.
pub const SLEW_ALTIT_MINUS: &str = "SLEW_ALTIT_MINUS";
/// While in Slew mode, move the plane up.
pub const SLEW_ALTIT_PLUS: &str = "SLEW_ALTIT_PLUS";
/// While in Slew mode, move the plane up (fast).
pub const SLEW_ALTIT_UP_FAST: &str = "SLEW_ALTIT_UP_FAST";
/// While in Slew mode, move the plane up (slow).
pub const SLEW_ALTIT_UP_SLOW: &str = "SLEW_ALTIT_UP_SLOW";
/// While in Slew mode, roll the plane left.
pub const SLEW_BANK_MINUS: &str = "SLEW_BANK_MINUS";
/// While in Slew mode, roll the plane right.
pub const SLEW_BANK_PLUS: &str = "SLEW_BANK_PLUS";
/// While in Slew mode, stop the translation on the X axis.
pub const SLEW_FREEZE: &str = "SLEW_FREEZE";
/// While in Slew mode, yaw the plane left.
pub const SLEW_HEADING_MINUS: &str = "SLEW_HEADING_MINUS";
/// While in Slew mode, yaw the plane right.
pub const SLEW_HEADING_PLUS: &str = "SLEW_HEADING_PLUS";
/// While in Slew mode, move the plane left.
pub const SLEW_LEFT: &str = "SLEW_LEFT";
/// Turns Slew mode off.
pub const SLEW_OFF: &str = "SLEW_OFF";
/// Turns Slew mode on.
pub const SLEW_ON: &str = "SLEW_ON";
/// While in Slew mode, pitch the plane down (fast).
pub const SLEW_PITCH_DN_FAST: &str = "SLEW_PITCH_DN_FAST";
/// While in Slew mode, pitch the plane down (slow).
pub const SLEW_PITCH_DN_SLOW: &str = "SLEW_PITCH_DN_SLOW";
/// While in Slew mode, stops the rotation on the X axis.
pub const SLEW_PITCH_FREEZE: &str = "SLEW_PITCH_FREEZE";
/// While in Slew mode, pitch the plane down.
pub const SLEW_PITCH_MINUS: &str = "SLEW_PITCH_MINUS";
/// While in Slew mode, pitch the plane up.
pub const SLEW_PITCH_PLUS: &str = "SLEW_PITCH_PLUS";
/// While in Slew mode, pitch the aircraft up (fast).
pub const SLEW_PITCH_UP_FAST: &str = "SLEW_PITCH_UP_FAST";
/// While in Slew mode, pitch the aircraft up (slow).
pub const SLEW_PITCH_UP_SLOW: &str = "SLEW_PITCH_UP_SLOW";
/// Stop slew and reset pitch, bank, and heading all to zero. Parameters: SKEY_LEW_RESET.
pub const SLEW_RESET: &str = "SLEW_RESET";
/// While in Slew mode, move the aircraft right.
pub const SLEW_RIGHT: &str = "SLEW_RIGHT";
/// Sets slew on/off (1,0) Parameters: \[0\]: Bool.
pub const SLEW_SET: &str = "SLEW_SET";
/// Toggles slew on/off
pub const SLEW_TOGGLE: &str = "SLEW_TOGGLE";

// Spoilers
/// Sets spoiler handle position. Parameters: \[0\]: Positon (0 - 1).
pub const AXIS_SPOILER_SET: &str = "AXIS_SPOILER_SET";
/// Sets auto-spoiler arming off (0).
pub const SPOILERS_ARM_OFF: &str = "SPOILERS_ARM_OFF";
/// Sets auto-spoiler arming on (1).
pub const SPOILERS_ARM_ON: &str = "SPOILERS_ARM_ON";
/// Sets auto-spoiler arming (0,1). Parameters: \[0\]: Bool.
pub const SPOILERS_ARM_SET: &str = "SPOILERS_ARM_SET";
/// Toggles arming of auto-spoilers between armed (1) and unarmed (0). Parameters:
/// SPOILERS_ARM_TOGGLE.
pub const SPOILERS_ARM_TOGGLE: &str = "SPOILERS_ARM_TOGGLE";
/// Decremement the spoilers by (down to a minimum of 0). Parameters: SPOILERS_DEC.
pub const SPOILERS_DEC: &str = "SPOILERS_DEC";
/// Increment the spoilers by (down to a minimum of 0). Parameters: SPOILERS_INC.
pub const SPOILERS_INC: &str = "SPOILERS_INC";
/// Sets spoiler handle to full retract position.
pub const SPOILERS_OFF: &str = "SPOILERS_OFF";
/// Sets spoiler handle to full extend position.
pub const SPOILERS_ON: &str = "SPOILERS_ON";
/// Increment or decrement the spoiler angles based on the speed and the distance of the
/// interaction from the user device (values are clamped between 0 to 16384). NOTE: This is
/// primarily for the PS5 controller touch-pad, but should work with any touch-input.
#[cfg(feature = "sunrise")]
pub const SPOILERS_RELATIVE_AXIS: &str = "SPOILERS_RELATIVE_AXIS";
/// Sets spoiler handle position. Parameters: \[0\]: Position (0 to 16383).
pub const SPOILERS_SET: &str = "SPOILERS_SET";
/// Toggles spoiler handle.
pub const SPOILERS_TOGGLE: &str = "SPOILERS_TOGGLE";

// Concorde
/// Not currently used in the simulation.
#[deprecated(note = "marked deprecated in the vendor SDK docs; do not use in new code")]
pub const INC_CONCORDE_NOSE_VISOR: &str = "INC_CONCORDE_NOSE_VISOR";
/// Not currently used in the simulation.
#[deprecated(note = "marked deprecated in the vendor SDK docs; do not use in new code")]
pub const DEC_CONCORDE_NOSE_VISOR: &str = "DEC_CONCORDE_NOSE_VISOR";
/// Not currently used in the simulation.
#[deprecated(note = "marked deprecated in the vendor SDK docs; do not use in new code")]
pub const CONCORDE_NOSE_VISOR_FULL_EXT: &str = "CONCORDE_NOSE_VISOR_FULL_EXT";
/// Not currently used in the simulation.
#[deprecated(note = "marked deprecated in the vendor SDK docs; do not use in new code")]
pub const CONCORDE_NOSE_VISOR_FULL_RET: &str = "CONCORDE_NOSE_VISOR_FULL_RET";
