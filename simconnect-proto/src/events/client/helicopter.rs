// General
/// Disable the auto-hover function of the helicopter, if one is available. NOTE: This is
/// currently not implemented in the Simulation
pub const AUTO_HOVER_OFF: &str = "AUTO_HOVER_OFF";
/// Enable the auto-hover function of the helicopter - if one is available. NOTE: This is
/// currently not implemented in the Simulation
pub const AUTO_HOVER_ON: &str = "AUTO_HOVER_ON";
/// Set the auto-hover - if available - to either on (True, 1) or off (False, 0). NOTE: This
/// is currently not implemented in the Simulation Parameters: \[0\]: True/False (1, 0).
pub const AUTO_HOVER_SET: &str = "AUTO_HOVER_SET";
/// Toggle the auto-hover - if available - between on (True, 1) and off (False, 0). NOTE: This
/// is currently not implemented in the Simulation
pub const AUTO_HOVER_TOGGLE: &str = "AUTO_HOVER_TOGGLE";
/// Set the collective pitch angle (a value from 0 to 1 interpolated from the 0 to 16384
/// input). Parameters: \[0\]: Set the collective (0 to 16384).
pub const AXIS_COLLECTIVE_SET: &str = "AXIS_COLLECTIVE_SET";
/// Set the steering axis value from -1 to 1 (interpolated from the -16384 to 16384 input).
/// Parameters: \[0\]: Set the steering amount (-16384 to 16384).
pub const AXIS_STEERING_SET: &str = "AXIS_STEERING_SET";
/// Sets the tail rotor speed as a value from 0 to 1 (interpolated from the 0 to 16384 input).
/// Parameters: \[0\]: Set tail rotor speed (0 to 16384).
pub const AXIS_TAIL_ROTOR_SET: &str = "AXIS_TAIL_ROTOR_SET";
/// Decrease the engine collective by the amount specified for the collective_increment
/// parameter. If the parameter is not set then a default value of 0.05 will be used. Minimum
/// value on decrement is clamped to 0.
pub const COLLECTIVE_DECR: &str = "COLLECTIVE_DECR";
/// Increase the engine collective by the amount specified for the collective_increment
/// parameter. If the parameter is not set then a default value of 0.05 will be used. Maximum
/// on increment value is clamped to 1.
pub const COLLECTIVE_INCR: &str = "COLLECTIVE_INCR";
/// Increment or decrement the collective based on the speed and the distance of the
/// interaction from the user device (values are clamped between 0 to 16384). NOTE: This is
/// primarily for the PS5 controller touch-pad, but should work with any touch-input.
#[cfg(feature = "sunrise")]
pub const COLLECTIVE_RELATIVE_AXIS: &str = "COLLECTIVE_RELATIVE_AXIS";
/// If the helicopter has an engine trimmer, this event can be used to decrease the nominal
/// engine/rotor RPM that the governor is trying to maintain for the indexed engine. The
/// amount that the trim will be adjusted by is set using the engine_trim_rate CFG parameter,
/// and the min and max achievable values are set using engine_trim_min and engine_trim_max.
/// Alternatively, you may supply a value that will override that set in the engine_trim_rate
/// CFG parameter (the min and max values will still be used). An engine index of 0 targets
/// all engines, and any other value targets that specific engine. Parameters: \[0\]: value
/// \[1\]: engine.
pub const DECREASE_HELO_GOV_BEEP: &str = "DECREASE_HELO_GOV_BEEP";
/// Set the helicopter force trim release button on (1) or off (0). When it is true (1) the
/// link between the user joystick and the cyclic is broken, and the user can move the
/// joystick freely while the helicopter cyclic control will be "frozen" at its current
/// position. This allows the user to move the trimmer, press this button and set the joystick
/// to the physical center, then release this button to restore the link between the joystick
/// and the cyclic again - but now from this new point of cyclic-vs-joystick position.
/// Parameters: \[0\]: Bool.
#[cfg(feature = "sunrise")]
pub const HELICOPTER_FORCE_TRIM_RELEASE_BUTTON_SET: &str =
    "HELICOPTER_FORCE_TRIM_RELEASE_BUTTON_SET";
/// If the helicopter has an engine trimmer, this event can be used to increase the nominal
/// engine/rotor RPM that the governor is trying to maintain for the indexed engine. The
/// amount that the trim will be adjusted by is set using the engine_trim_rate CFG parameter,
/// and the min and max achievable values are set using engine_trim_min and engine_trim_max.
/// Alternatively, you may supply a value that will override that set in the engine_trim_rate
/// CFG parameter (the min and max values will still be used). An engine index of 0 targets
/// all engines, and any other value targets that specific engine. Parameters: \[0\]: value
/// \[1\]: engine.
pub const INCREASE_HELO_GOV_BEEP: &str = "INCREASE_HELO_GOV_BEEP";
/// If true (1) then assistance to automatically trim your helicopter has been enabled, if
/// false (0) then it has not. Parameters: \[0\]: Bool.
#[cfg(feature = "sunrise")]
pub const QUICK_TRIM: &str = "QUICK_TRIM";
/// This is used to set the helicopter engine trimmer to the given value directly as a
/// negative or positive deviation from 1, where 1 is the rated nominal engine RPM. The final
/// engine trimmer value will be limited according to the engine_trim_min and engine_trim_max
/// settings. An engine index of 0 targets all engines, and any other value targets that
/// specific engine. Parameters: \[0\]: value \[1\]: engine.
pub const SET_HELO_GOV_BEEP: &str = "SET_HELO_GOV_BEEP";

// Cyclic
/// Set the lateral cyclic axis as a value between -16384 and 16384. Parameters: \[0\]: Set
/// the lateral cyclic (-16384 to 16384).
pub const AXIS_CYCLIC_LATERAL_SET: &str = "AXIS_CYCLIC_LATERAL_SET";
/// Set the longitudinal cyclic axis. Parameters: \[0\]: Set the longitudinal cyclic (-16384
/// to 16384).
pub const AXIS_CYCLIC_LONGITUDINAL_SET: &str = "AXIS_CYCLIC_LONGITUDINAL_SET";
/// Change the lateral cyclic (left) by -0.098 when pressed. If held down, the change will
/// happen more rapidly.
pub const CYCLIC_LATERAL_LEFT: &str = "CYCLIC_LATERAL_LEFT";
/// Change the lateral cyclic (right) by 0.098 when pressed. If held down, the change will
/// happen more rapidly.
pub const CYCLIC_LATERAL_RIGHT: &str = "CYCLIC_LATERAL_RIGHT";
/// Change the longitudinal cyclic (down) by -0.049 when pressed. If held down, the change
/// will happen more rapidly.
pub const CYCLIC_LONGITUDINAL_DOWN: &str = "CYCLIC_LONGITUDINAL_DOWN";
/// Change the longitudinal cyclic (up) by 0.049 when pressed. If held down, the change will
/// happen more rapidly.
pub const CYCLIC_LONGITUDINAL_UP: &str = "CYCLIC_LONGITUDINAL_UP";

// Throttle Control
/// Set the throttle 1 or 2 value from 0 to 1 (interpolated from the 0 to 16384 input).
/// Parameters: \[0\]: Throttle value (0 to 16384).
pub const AXIS_HELICOPTER_THROTTLE1_SET: &str = "AXIS_HELICOPTER_THROTTLE1_SET";
/// Set the throttle 1 or 2 value from 0 to 1 (interpolated from the 0 to 16384 input).
/// Parameters: \[0\]: Throttle value (0 to 16384).
pub const AXIS_HELICOPTER_THROTTLE2_SET: &str = "AXIS_HELICOPTER_THROTTLE2_SET";
/// Set all throttles to a value from 0 to 1 (interpolated from the 0 to 16384 input).
/// Parameters: \[0\]: Throttle value (0 to 16384).
pub const AXIS_HELICOPTER_THROTTLE_SET: &str = "AXIS_HELICOPTER_THROTTLE_SET";
/// Cut throttle 1 or 2.
pub const HELICOPTER_THROTTLE1_CUT: &str = "HELICOPTER_THROTTLE1_CUT";
/// Cut throttle 1 or 2.
pub const HELICOPTER_THROTTLE2_CUT: &str = "HELICOPTER_THROTTLE2_CUT";
/// By default this will decrement throttle 1 or 2 by 1/128, to a minimum of 0. If you provide
/// an input parameter then this will be internally normalised to a value between 0 and 1 and
/// used to decrement instead. Parameters: \[0\]: Decrement value (0 to 16384).
pub const HELICOPTER_THROTTLE1_DEC: &str = "HELICOPTER_THROTTLE1_DEC";
/// By default this will decrement throttle 1 or 2 by 1/128, to a minimum of 0. If you provide
/// an input parameter then this will be internally normalised to a value between 0 and 1 and
/// used to decrement instead. Parameters: \[0\]: Decrement value (0 to 16384).
pub const HELICOPTER_THROTTLE2_DEC: &str = "HELICOPTER_THROTTLE2_DEC";
/// Set throttle 1 or 2 to full.
pub const HELICOPTER_THROTTLE1_FULL: &str = "HELICOPTER_THROTTLE1_FULL";
/// Set throttle 1 or 2 to full.
pub const HELICOPTER_THROTTLE2_FULL: &str = "HELICOPTER_THROTTLE2_FULL";
/// By default this will increment throttle 1 or 2 by 1/128, to a maximum of 1. If you provide
/// an input parameter then this will be internally normalised to a value between 0 and 1 and
/// used to increment instead. Parameters: \[0\]: Increment value (0 to 16384).
pub const HELICOPTER_THROTTLE1_INC: &str = "HELICOPTER_THROTTLE1_INC";
/// By default this will increment throttle 1 or 2 by 1/128, to a maximum of 1. If you provide
/// an input parameter then this will be internally normalised to a value between 0 and 1 and
/// used to increment instead. Parameters: \[0\]: Increment value (0 to 16384).
pub const HELICOPTER_THROTTLE2_INC: &str = "HELICOPTER_THROTTLE2_INC";
/// Set throttle 1 or 2 based on the input value. The input is between 0 and 16384, which will
/// be normalised to a value between 0 and 1. Parameters: \[0\]: Throttle value (0 to 16384).
pub const HELICOPTER_THROTTLE1_SET: &str = "HELICOPTER_THROTTLE1_SET";
/// Set throttle 1 or 2 based on the input value. The input is between 0 and 16384, which will
/// be normalised to a value between 0 and 1. Parameters: \[0\]: Throttle value (0 to 16384).
pub const HELICOPTER_THROTTLE2_SET: &str = "HELICOPTER_THROTTLE2_SET";
/// Cut all throttles.
pub const HELICOPTER_THROTTLE_CUT: &str = "HELICOPTER_THROTTLE_CUT";
/// By default this will decrement all throttles by 1/128, to a minimum of 0. If you provide
/// an input parameter then this will be internally normalised to a value between 0 and 1 and
/// used to decrement instead. Parameters: \[0\]: Decrement value (0 to 16384).
pub const HELICOPTER_THROTTLE_DEC: &str = "HELICOPTER_THROTTLE_DEC";
/// Set all throttles to full.
pub const HELICOPTER_THROTTLE_FULL: &str = "HELICOPTER_THROTTLE_FULL";
/// By default this will increment all throttles by 1/128, to a maximum of 1. If you provide
/// an input parameter then this will be internally normalised to a value between 0 and 1 and
/// used to increment instead. Parameters: \[0\]: Increment value (0 to 16384).
pub const HELICOPTER_THROTTLE_INC: &str = "HELICOPTER_THROTTLE_INC";
/// Increment or decrement the throttle based on the speed and the distance of the interaction
/// from the user device (values are clamped between 0 to 16384). NOTE: This is primarily for
/// the PS5 controller touch-pad, but should work with any touch-input.
#[cfg(feature = "sunrise")]
pub const HELICOPTER_THROTTLE_RELATIVE_AXIS: &str = "HELICOPTER_THROTTLE_RELATIVE_AXIS";
/// Set all throttles based on the input value. The input is between 0 and 16384, which will
/// be normalised to a value between 0 and 1. Parameters: \[0\]: Throttle value (0 to 16384).
pub const HELICOPTER_THROTTLE_SET: &str = "HELICOPTER_THROTTLE_SET";

// Rotor Control
/// Set the rotor brake. This takes a value between 0 to 16384, which will be interpreted by
/// the simulation as a percentage where 0 is fully off and 16384 is fully on. Parameters:
/// \[0\]: Brake Lever position (0 to 16384).
pub const AXIS_ROTOR_BRAKE_SET: &str = "AXIS_ROTOR_BRAKE_SET";
/// Set the tail (or second) rotor brake. This takes a value between 0 to 16384, which will be
/// interpreted by the simulation as a percentage where 0 is fully off and 16384 is fully on.
/// Parameters: \[0\]: Brake Lever Position (0 to 16384).
pub const ROTOR_AXIS_TAIL_ROTOR_SET: &str = "ROTOR_AXIS_TAIL_ROTOR_SET";
/// Sets rotor brake switch on. Deprecated in favour of ROTOR_BRAKE_ON. Parameters: \[0\]:
/// Bool.
pub const ROTOR_BRAKE: &str = "ROTOR_BRAKE";
/// Sets the rotor brake lock to on (1) or off (0). Parameters: \[0\]: Bool.
#[cfg(feature = "sunrise")]
pub const ROTOR_BRAKE_LOCK_SET: &str = "ROTOR_BRAKE_LOCK_SET";
/// Switches off the rotor brake switch .
pub const ROTOR_BRAKE_OFF: &str = "ROTOR_BRAKE_OFF";
/// Switches on the rotor brake switch .
pub const ROTOR_BRAKE_ON: &str = "ROTOR_BRAKE_ON";
/// Toggle the rotor brake switch between on (1) and off (0).
pub const ROTOR_BRAKE_TOGGLE: &str = "ROTOR_BRAKE_TOGGLE";
/// Sets the rotor clutch switch to on (1) or off (0). Parameters: \[0\]: Bool.
pub const ROTOR_CLUTCH_SWITCH_SET: &str = "ROTOR_CLUTCH_SWITCH_SET";
/// Toggles the rotor clutch switch between on (1) and off (0).
pub const ROTOR_CLUTCH_SWITCH_TOGGLE: &str = "ROTOR_CLUTCH_SWITCH_TOGGLE";
/// Sets the rotor governor switch to off (0). An index of 0 targets all engines, and any
/// other value targets that specific engine. Parameters: \[0\]: engine.
pub const ROTOR_GOV_SWITCH_OFF: &str = "ROTOR_GOV_SWITCH_OFF";
/// Sets the rotor governor switch to on (1). An index of 0 targets all engines, and any other
/// value targets that specific engine. Parameters: \[0\]: engine.
pub const ROTOR_GOV_SWITCH_ON: &str = "ROTOR_GOV_SWITCH_ON";
/// Sets the rotor governor switch to on/off (1,0). An index of 0 targets all engines, and any
/// other value targets that specific engine. Parameters: \[0\]: Bool \[1\]: engine.
pub const ROTOR_GOV_SWITCH_SET: &str = "ROTOR_GOV_SWITCH_SET";
/// Toggles the rotor governor switch between on (1) and off (0). An index of 0 targets all
/// engines, and any other value targets that specific engine. Parameters: \[0\]: engine.
pub const ROTOR_GOV_SWITCH_TOGGLE: &str = "ROTOR_GOV_SWITCH_TOGGLE";
/// Decrements the roll (lateral) rotor trim by the amount specified by the parameter
/// right_trim_step.
pub const ROTOR_LATERAL_TRIM_DEC: &str = "ROTOR_LATERAL_TRIM_DEC";
/// Increments the roll (lateral) rotor trim by the amount specified by the parameter
/// right_trim_step.
pub const ROTOR_LATERAL_TRIM_INC: &str = "ROTOR_LATERAL_TRIM_INC";
/// Sets the roll (lateral) rotor trim to a value between -1 and 1 (interpolated from the
/// +/-16384 input value). Parameters: \[0\]: Pitch angle (+/- 16384).
pub const ROTOR_LATERAL_TRIM_SET: &str = "ROTOR_LATERAL_TRIM_SET";
/// Decrements the pitch (longitudinal) rotor trim by the amount specified by the parameter
/// front_trim_step.
pub const ROTOR_LONGITUDINAL_TRIM_DEC: &str = "ROTOR_LONGITUDINAL_TRIM_DEC";
/// Increments the pitch (longitudinal) rotor trim by the amount specified by the parameter
/// front_trim_step.
pub const ROTOR_LONGITUDINAL_TRIM_INC: &str = "ROTOR_LONGITUDINAL_TRIM_INC";
/// Sets the pitch (longitudinal) rotor trim to a value between -1 and 1 (interpolated from
/// the +/-16384 input value). Parameters: \[0\]: Pitch angle (+/- 16384).
pub const ROTOR_LONGITUDINAL_TRIM_SET: &str = "ROTOR_LONGITUDINAL_TRIM_SET";
/// Resets the rotor trim values to their default.
pub const ROTOR_TRIM_RESET: &str = "ROTOR_TRIM_RESET";
/// Decrements the tail rotor by 0.1.
pub const TAIL_ROTOR_DECR: &str = "TAIL_ROTOR_DECR";
/// Increments the tail rotor by 0.1.
pub const TAIL_ROTOR_INCR: &str = "TAIL_ROTOR_INCR";

// Engine Control
/// For a helicopter, toggle the engine 1/2 governor switch between ON (1) and OFF (0).
pub const HELICOPTER_ENGINE_1_GOVERNOR_SWITCH_OFF: &str = "HELICOPTER_ENGINE_1_GOVERNOR_SWITCH_OFF";
/// For a helicopter, toggle the engine 1/2 governor switch between ON (1) and OFF (0).
pub const HELICOPTER_ENGINE_2_GOVERNOR_SWITCH_OFF: &str = "HELICOPTER_ENGINE_2_GOVERNOR_SWITCH_OFF";
/// For a helicopter, set the engine 1/2 governor switch ON.
pub const HELICOPTER_ENGINE_1_GOVERNOR_SWITCH_ON: &str = "HELICOPTER_ENGINE_1_GOVERNOR_SWITCH_ON";
/// For a helicopter, set the engine 1/2 governor switch ON.
pub const HELICOPTER_ENGINE_2_GOVERNOR_SWITCH_ON: &str = "HELICOPTER_ENGINE_2_GOVERNOR_SWITCH_ON";
/// For a helicopter, set the engine 1/2 governor switch OFF.
pub const HELICOPTER_ENGINE_1_GOVERNOR_SWITCH_TOGGLE: &str =
    "HELICOPTER_ENGINE_1_GOVERNOR_SWITCH_TOGGLE";
/// For a helicopter, set the engine 1/2 governor switch OFF.
pub const HELICOPTER_ENGINE_2_GOVERNOR_SWITCH_TOGGLE: &str =
    "HELICOPTER_ENGINE_2_GOVERNOR_SWITCH_TOGGLE";
/// For a helicopter, set the engine 1/2 governor switch to either ON (1) or OFF (0).
/// Parameters: \[0\]: Bool.
pub const HELICOPTER_ENGINE_1_GOVERNOR_SWITCH_SET: &str = "HELICOPTER_ENGINE_1_GOVERNOR_SWITCH_SET";
/// For a helicopter, set the engine 1/2 governor switch to either ON (1) or OFF (0).
/// Parameters: \[0\]: Bool.
pub const HELICOPTER_ENGINE_2_GOVERNOR_SWITCH_SET: &str = "HELICOPTER_ENGINE_2_GOVERNOR_SWITCH_SET";
/// For a helicopter, increase the engine 1/2 trim RPM by the given value amount. Parameters:
/// \[0\]: value.
pub const HELICOPTER_ENGINE_1_BEEP_TRIM_INCREASE: &str = "HELICOPTER_ENGINE_1_BEEP_TRIM_INCREASE";
/// For a helicopter, increase the engine 1/2 trim RPM by the given value amount. Parameters:
/// \[0\]: value.
pub const HELICOPTER_ENGINE_2_BEEP_TRIM_INCREASE: &str = "HELICOPTER_ENGINE_2_BEEP_TRIM_INCREASE";
/// For a helicopter, decrease the engine 1/2 trim RPM by the given value amount. Parameters:
/// \[0\]: value.
pub const HELICOPTER_ENGINE_1_BEEP_TRIM_DECREASE: &str = "HELICOPTER_ENGINE_1_BEEP_TRIM_DECREASE";
/// For a helicopter, decrease the engine 1/2 trim RPM by the given value amount. Parameters:
/// \[0\]: value.
pub const HELICOPTER_ENGINE_2_BEEP_TRIM_DECREASE: &str = "HELICOPTER_ENGINE_2_BEEP_TRIM_DECREASE";
/// For a helicopter, set the engine 1/2 trim RPM to the given value. Parameters: \[0\]:
/// value.
pub const HELICOPTER_ENGINE_1_BEEP_TRIM_SET: &str = "HELICOPTER_ENGINE_1_BEEP_TRIM_SET";
/// For a helicopter, set the engine 1/2 trim RPM to the given value. Parameters: \[0\]:
/// value.
pub const HELICOPTER_ENGINE_2_BEEP_TRIM_SET: &str = "HELICOPTER_ENGINE_2_BEEP_TRIM_SET";
