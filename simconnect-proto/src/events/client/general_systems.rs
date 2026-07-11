// Liquid Dropping System
/// Close the named (or indexed) liquid dropping system door. Parameters: \[0\]: Door name (or
/// index).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_DOOR_CLOSE: &str = "LIQUID_DROPPING_SYSTEM_DOOR_CLOSE";
/// Close all the doors that belong to the given command group index. Parameters: \[0\]: Door
/// command group index.
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_DOOR_COMMAND_GROUP_CLOSE: &str =
    "LIQUID_DROPPING_SYSTEM_DOOR_COMMAND_GROUP_CLOSE";
/// Open all the doors that belong to the given command group index. Parameters: \[0\]: Door
/// command group index.
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_DOOR_COMMAND_GROUP_OPEN: &str =
    "LIQUID_DROPPING_SYSTEM_DOOR_COMMAND_GROUP_OPEN";
/// Set all the doors that belong to the given command group index to the supplied value,
/// where 0 is fully closed and 16384 is fully open. Parameters: \[0\]: Door command group
/// index \[1\]: Door Target Position (Position 16k).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_DOOR_COMMAND_GROUP_SET: &str =
    "LIQUID_DROPPING_SYSTEM_DOOR_COMMAND_GROUP_SET";
/// Toggle all the doors that belong to the given command group index (so open doors will
/// close and closed doors will open). Note that the initial position is taken from the first
/// door found with the correct command group index, and then the toggled value for that door
/// is applied to all doors, even if their original position was different. Parameters: \[0\]:
/// Door command group index.
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_DOOR_COMMAND_GROUP_TOGGLE: &str =
    "LIQUID_DROPPING_SYSTEM_DOOR_COMMAND_GROUP_TOGGLE";
/// Open the named (or indexed) liquid dropping system door. Parameters: \[0\]: Door name (or
/// index).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_DOOR_OPEN: &str = "LIQUID_DROPPING_SYSTEM_DOOR_OPEN";
/// Set the named (or indexed) door to the supplied value, where 0 is fully closed and +16384
/// is fully open. Parameters: \[0\]: Door name (or index) \[1\]: Door Target Position
/// (Position 16k).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_DOOR_SET: &str = "LIQUID_DROPPING_SYSTEM_DOOR_SET";
/// Toggle the named (or indexed) liquid dropping system door (so open doors will close and
/// closed doors will open). Parameters: \[0\]: Door name (or index).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_DOOR_TOGGLE: &str = "LIQUID_DROPPING_SYSTEM_DOOR_TOGGLE";
/// Open the named (or indexed) liquid dropping system scoop. Parameters: \[0\]: Scoop name
/// (or index).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_SCOOP_CLOSE: &str = "LIQUID_DROPPING_SYSTEM_SCOOP_CLOSE";
/// Open the named (or indexed) liquid dropping system scoop. Parameters: \[0\]: Scoop name
/// (or index).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_SCOOP_OPEN: &str = "LIQUID_DROPPING_SYSTEM_SCOOP_OPEN";
/// Set the named (or indexed) scoop to the supplied value, where 0 is fully closed and 16384
/// is fully open. Parameters: \[0\]: Scoop name (or index) \[1\]: Scoop Target Position
/// (Position 16k).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_SCOOP_SET: &str = "LIQUID_DROPPING_SYSTEM_SCOOP_SET";
/// Toggle the named (or indexed) liquid dropping system scoop (so open scoops will close and
/// closed scoops will open). Parameters: \[0\]: Scoop name (or index).
#[cfg(feature = "sunrise")]
pub const LIQUID_DROPPING_SYSTEM_SCOOP_TOGGLE: &str = "LIQUID_DROPPING_SYSTEM_SCOOP_TOGGLE";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const FIREFIGHTING_SCOOP_DOORS: &str = "FIREFIGHTING_SCOOP_DOORS";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SPRAY_OFF: &str = "SPRAY_OFF";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SPRAY_ON: &str = "SPRAY_ON";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SPRAY_TOGGLE: &str = "SPRAY_TOGGLE";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SPRAY_SET: &str = "SPRAY_SET";

// Pneumatics System
/// Decrements the target temperature of the specified area by the given amount. Value is
/// given in °C. Parameters: \[0\]: Area name (or index) \[1\]: Value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_AREA_TEMPERATURE_DEC: &str = "PNEUMATICS_AREA_TEMPERATURE_DEC";
/// Increments the target temperature of the specified area by the given amount. Value is
/// given in °C. Parameters: \[0\]: Area name (or index) \[1\]: Value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_AREA_TEMPERATURE_INC: &str = "PNEUMATICS_AREA_TEMPERATURE_INC";
/// Sets the target temperature of the specified area to the given amount. Value is given in
/// °C. Parameters: \[0\]: Area name (or index) \[1\]: Value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_AREA_TEMPERATURE_SET: &str = "PNEUMATICS_AREA_TEMPERATURE_SET";
/// Sets the the speed of the given fan to the specified ratio where 0 is off and 1 is full
/// power. Parameters: \[0\]: Fan name (or index) \[1\]: Percent Over 100.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_FAN_SET: &str = "PNEUMATICS_FAN_SET";
/// Sets the given pack auto flow mode to OFF. Parameters: \[0\]: Pack name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_FLOW_AUTO_OFF: &str = "PNEUMATICS_PACK_FLOW_AUTO_OFF";
/// Sets the given pack auto flow mode to ON. Parameters: \[0\]: Pack name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_FLOW_AUTO_ON: &str = "PNEUMATICS_PACK_FLOW_AUTO_ON";
/// Sets the given pack auto flow mode to ON (1) or OFF (0). Parameters: \[0\]: Pack name (or
/// index) \[1\]: Bool.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_FLOW_AUTO_SET: &str = "PNEUMATICS_PACK_FLOW_AUTO_SET";
/// Sets the given pack flow state to HIGH. Parameters: \[0\]: Pack name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_FLOW_MODE_HIGH: &str = "PNEUMATICS_PACK_FLOW_MODE_HIGH";
/// Sets the given pack flow state to LOW. Parameters: \[0\]: Pack name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_FLOW_MODE_LOW: &str = "PNEUMATICS_PACK_FLOW_MODE_LOW";
/// Sets the given pack flow state to NORMAL. Parameters: \[0\]: Pack name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_FLOW_MODE_NORM: &str = "PNEUMATICS_PACK_FLOW_MODE_NORM";
/// Sets the given pack to use a different flow state, where: 0 = LOW 1 = NORMAL 2 = HIGH
/// Parameters: \[0\]: Pack name (or index) \[1\]: Enum.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_FLOW_MODE_SET: &str = "PNEUMATICS_PACK_FLOW_MODE_SET";
/// Turns OFF the given pack. Parameters: \[0\]: Pack name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_OFF: &str = "PNEUMATICS_PACK_OFF";
/// Turns ON the given pack. Parameters: \[0\]: Pack name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_ON: &str = "PNEUMATICS_PACK_ON";
/// Turns the given pack ON (1) or OFF (0) based on the supplied value. Parameters: \[0\]:
/// Pack name (or index) \[1\]: Bool.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_SET: &str = "PNEUMATICS_PACK_SET";
/// This can be used to toggle the given pack ON (1) or OFF (0). Parameters: \[0\]: Pack name
/// (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACK_TOGGLE: &str = "PNEUMATICS_PACK_TOGGLE";
/// Decrements the flow amount of the specified area by the given amount. Value should be
/// between 0 and 1. Parameters: \[0\]: Pack name (or index) \[1\]: Value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACKS_FLOW_DEC: &str = "PNEUMATICS_PACKS_FLOW_DEC";
/// Increments the flow amount of the specified area by the given amount. Value should be
/// between 0 and 1. Parameters: \[0\]: Pack name (or index) \[1\]: Value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACKS_FLOW_INC: &str = "PNEUMATICS_PACKS_FLOW_INC";
/// Increments the flow amount of the specified area by the given amount. Value should be
/// between 0 and 1. Parameters: \[0\]: Pack name (or index) \[1\]: Value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_PACKS_FLOW_SET: &str = "PNEUMATICS_PACKS_FLOW_SET";
/// Decrements the associated pressure target altitude for the whole aircraft by the given
/// amount. Value is in ft. Parameters: \[0\]: Value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_TARGET_CABIN_ALTITUDE_DEC: &str = "PNEUMATICS_TARGET_CABIN_ALTITUDE_DEC";
/// Increments the associated pressure target altitude for the whole aircraft by the given
/// amount. Value is in ft. Parameters: \[0\]: Value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_TARGET_CABIN_ALTITUDE_INC: &str = "PNEUMATICS_TARGET_CABIN_ALTITUDE_INC";
/// Sets the associated pressure target altitude for the whole aircraft to the given altitude.
/// Value is in ft. Parameters: \[0\]: value.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_TARGET_CABIN_ALTITUDE_SET: &str = "PNEUMATICS_TARGET_CABIN_ALTITUDE_SET";
/// Closes the given valve. Parameters: \[0\]: Valve name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_CLOSE: &str = "PNEUMATICS_VALVE_CLOSE";
/// Sets the operational mode of the given valve to AUTO. Parameters: \[0\]: Valve name (or
/// index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_MODE_AUTO: &str = "PNEUMATICS_VALVE_MODE_AUTO";
/// Sets the operational mode of the given valve to CLOSED. Parameters: \[0\]: Valve name (or
/// index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_MODE_CLOSED: &str = "PNEUMATICS_VALVE_MODE_CLOSED";
/// Sets the operational mode of the given valve to OPEN. Parameters: \[0\]: Valve name (or
/// index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_MODE_OPEN: &str = "PNEUMATICS_VALVE_MODE_OPEN";
/// Sets the operational mode of the given valve to the given value. The mode values are as
/// follows: 0 = AUTO 1 = MANUAL 2 = OPEN 3 = CLOSED Parameters: \[0\]: Valve name (or index)
/// \[1\]: Enum.
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_MODE_SET: &str = "PNEUMATICS_VALVE_MODE_SET";
/// Opens the given valve. Parameters: \[0\]: Valve name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_OPEN: &str = "PNEUMATICS_VALVE_OPEN";
/// Set the given valve to the supplied value, where 0 is fully closed and 16384 is fully
/// open. NOTE: This key event has issues when trying to set the valve value and you should be
/// using either the PNEUMATIC_VALVE_SET_EX1 version, or the PNEUMATICS VALVE TARGET STATUS
/// SimVar. Parameters: \[0\]: Valve name (or index) \[1\]: Valve target position (Position
/// 16k).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_SET: &str = "PNEUMATICS_VALVE_SET";
/// Set the given valve to the supplied value, where 0 is fully closed and 16384 is fully
/// open. Parameters: \[0\]: Valve name (or index) \[1\]: Valve target position (Position
/// 16k).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_SET_EX1: &str = "PNEUMATICS_VALVE_SET_EX1";
/// This can be used to toggle the given valve open (1) or closed (0). Parameters: \[0\]:
/// Valve name (or index).
#[cfg(feature = "sunrise")]
pub const PNEUMATICS_VALVE_TOGGLE: &str = "PNEUMATICS_VALVE_TOGGLE";

// Hydraulics System
/// This can be used to switch off a "Custom" actuator (it will fail on all other actuator
/// types). See here for more information: Custom Actuators Parameters: \[0\]: actuator name
/// (or index).
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_ACTUATOR_ACTIVE_OFF: &str = "HYDRAULIC_ACTUATOR_ACTIVE_OFF";
/// This can be used to switch on a "Custom" actuator (it will fail on all other actuator
/// types). See here for more information: Custom Actuators Parameters: \[0\]: actuator name
/// (or index).
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_ACTUATOR_ACTIVE_ON: &str = "HYDRAULIC_ACTUATOR_ACTIVE_ON";
/// This can be used to switch on or off a "Custom" actuator (it will fail on all other
/// actuator types). See here for more information: Custom Actuators Parameters: \[0\]:
/// actuator name (or index) \[1\] Bool.
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_ACTUATOR_ACTIVE_SET: &str = "HYDRAULIC_ACTUATOR_ACTIVE_SET";
/// This can be used to toggle on or off a "Custom" actuator (it will fail on all other
/// actuator types). See here for more information: Custom Actuators Parameters: \[0\]:
/// actuator name (or index).
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_ACTUATOR_ACTIVE_TOGGLE: &str = "HYDRAULIC_ACTUATOR_ACTIVE_TOGGLE";
/// This can be used to toggle the given hydraulic pump on or off. Parameters: \[0\]: pump
/// name (or index).
pub const HYDRAULIC_SWITCH_TOGGLE: &str = "HYDRAULIC_SWITCH_TOGGLE";
/// This will close the given hydraulic valve. Parameters: \[0\]: valve name (or index).
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_VALVE_CLOSE: &str = "HYDRAULIC_VALVE_CLOSE";
/// This will open the given hydraulic valve. Parameters: \[0\]: valve name (or index).
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_VALVE_OPEN: &str = "HYDRAULIC_VALVE_OPEN";
/// Set the given valve to the supplied value, where 0 is fully closed and 16384 is fully
/// open. NOTE: This key event has issues when trying to set the valve value and you should be
/// using either the HYDRAULIC_VALVE_SET_EX1 version, or the HYDRAULIC VALVE TARGET POS
/// SimVar. Parameters: \[0\]: valve name (or index) \[1\]: Valve target position (Position
/// 16k).
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_VALVE_SET: &str = "HYDRAULIC_VALVE_SET";
/// Set the given valve to the supplied value, where 0 is fully closed and 16384 is fully
/// open. Parameters: \[0\]: valve name (or index) \[1\]: Valve target position (Position
/// 16k).
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_VALVE_SET_EX1: &str = "HYDRAULIC_VALVE_SET_EX1";
/// This will toggle the given hydraulic valve open/closed. Parameters: \[0\]: valve name (or
/// index).
#[cfg(feature = "sunrise")]
pub const HYDRAULIC_VALVE_TOGGLE: &str = "HYDRAULIC_VALVE_TOGGLE";

// Landing Gear / Brakes
/// Turn the anti-skid braking system on or off.
pub const ANTISKID_BRAKES_TOGGLE: &str = "ANTISKID_BRAKES_TOGGLE";
/// Sets the left brake position from an axis controller (e.g. joystick) to the value given as
/// the parameter \[0\], from -16383 (0 braking) to +16383 (maximum braking). Note that this
/// is on a linear scale: -16383 = 0% 0 = 50% +16383 = 100% NOTE: This is now simply an alias
/// for AXIS_LEFT_BRAKE_SET, and as such should not be used. Parameters: \[0\]: the brake
/// position from -16383 to 16383.
pub const AXIS_LEFT_BRAKE_LINEAR_SET: &str = "AXIS_LEFT_BRAKE_LINEAR_SET";
/// Sets the left brake position from an axis controller (e.g. joystick) to the value given as
/// the parameter \[0\], from -16383 (0 braking) to +16383 (maximum braking). Note that this
/// is on a non-linear scale: -16383 = 0% -8191 = 8% 0 = 27% +8191 = 53% +16383 = 100%
/// Parameters: \[0\]: the brake position from -16383 to 16383.
pub const AXIS_LEFT_BRAKE_SET: &str = "AXIS_LEFT_BRAKE_SET";
/// Sets the right brake position from an axis controller (e.g. joystick) to the value given
/// as the parameter \[0\], from -16383 (0 braking) to +16383 (maximum braking). Note that
/// this is on a linear scale: -16383 = 0% 0 = 50% +16383 = 100% NOTE: This is now simply an
/// alias for AXIS_RIGHT_BRAKE_SET, and as such should not be used. Parameters: \[0\]: the
/// brake position from -16383 to 16383.
pub const AXIS_RIGHT_BRAKE_LINEAR_SET: &str = "AXIS_RIGHT_BRAKE_LINEAR_SET";
/// Sets the right brake position from an axis controller (e.g. joystick) to the value given
/// as the parameter \[0\], from -16383 (0 braking) to +16383 (maximum braking). Note that
/// this is on a non-linear scale: -16383 = 0% -8191 = 8% 0 = 27% +8191 = 53% +16383 = 100%
/// Parameters: \[0\]: the brake position from -16383 to 16383.
pub const AXIS_RIGHT_BRAKE_SET: &str = "AXIS_RIGHT_BRAKE_SET";
/// Increment brake pressure
pub const BRAKES: &str = "BRAKES";
/// Increments left brake pressure
pub const BRAKES_LEFT: &str = "BRAKES_LEFT";
/// Increments right brake pressure
pub const BRAKES_RIGHT: &str = "BRAKES_RIGHT";
/// Sets gear handle in DOWN position
pub const GEAR_DOWN: &str = "GEAR_DOWN";
/// Toggle gear emergency handle.
pub const GEAR_EMERGENCY_HANDLE_TOGGLE: &str = "GEAR_EMERGENCY_HANDLE_TOGGLE";
/// Increments emergency gear extension
pub const GEAR_PUMP: &str = "GEAR_PUMP";
/// Sets gear handle position up/down (0,1) Parameters: \[0\]: Position.
pub const GEAR_SET: &str = "GEAR_SET";
/// Toggle gear handle
pub const GEAR_TOGGLE: &str = "GEAR_TOGGLE";
/// Sets gear handle in UP position
pub const GEAR_UP: &str = "GEAR_UP";
/// Toggles the parking brake on/off
pub const PARKING_BRAKES: &str = "PARKING_BRAKES";
/// Enables the parking brake.
#[cfg(feature = "sunrise")]
pub const PARKING_BRAKES_OFF: &str = "PARKING_BRAKES_OFF";
/// Disables the parking brake.
#[cfg(feature = "sunrise")]
pub const PARKING_BRAKES_ON: &str = "PARKING_BRAKES_ON";
/// Set the parking brake on/off Parameters: \[0\]: Bool.
pub const PARKING_BRAKE_SET: &str = "PARKING_BRAKE_SET";
/// If the plane has retractable floats, moves the retract position from Extend to Neutral, or
/// Neutral to Retract.
pub const RETRACT_FLOAT_SWITCH_DEC: &str = "RETRACT_FLOAT_SWITCH_DEC";
/// If the plane has retractable floats, moves the retract position from Retract to Neutral,
/// or Neutral to Extend.
pub const RETRACT_FLOAT_SWITCH_INC: &str = "RETRACT_FLOAT_SWITCH_INC";

// Slings and Hoists
/// Toggle between pickup and release mode. Hold mode is automatic and cannot be selected.
pub const SLING_PICKUP_RELEASE: &str = "SLING_PICKUP_RELEASE";
/// Set the hoist to extend.
pub const HOIST_SWITCH_EXTEND: &str = "HOIST_SWITCH_EXTEND";
/// Set the hoist to retract.
pub const HOIST_SWITCH_RETRACT: &str = "HOIST_SWITCH_RETRACT";
/// Sets the mode to change the hoist sling to either extend (-) or retract (+) using the PLUS
/// and MINUS keys.
pub const HOIST_SWITCH_SELECT: &str = "HOIST_SWITCH_SELECT";
/// The hoist control switch setting. Should be set to one of the following values: -1 -
/// HOIST_RETRACT 0 - HOIST_OFF 1 - HOIST_EXTEND Parameters: \[0\] Value.
pub const HOIST_SWITCH_SET: &str = "HOIST_SWITCH_SET";
/// Toggles the hoist arm switch, extend or retract.
pub const HOIST_DEPLOY_TOGGLE: &str = "HOIST_DEPLOY_TOGGLE";
/// The hoist deployment setting. The value should be set to one of the following: 0 - set
/// hoist switch to retract the arm 1 - set hoist switch to extend the arm Parameters: \[0\]
/// Bool.
pub const HOIST_DEPLOY_SET: &str = "HOIST_DEPLOY_SET";

// Grapple Hook / Lead Pole
/// Close the grapple hook.
#[cfg(feature = "sunrise")]
pub const GRAPPLE_HOOK_OFF: &str = "GRAPPLE_HOOK_OFF";
/// Open the grapple hook.
#[cfg(feature = "sunrise")]
pub const GRAPPLE_HOOK_ON: &str = "GRAPPLE_HOOK_ON";
/// Toggle the grapple hook between open and closed.
#[cfg(feature = "sunrise")]
pub const GRAPPLE_HOOK_TOGGLE: &str = "GRAPPLE_HOOK_TOGGLE";
/// Set the grapple hook open value, between 0 (closed) and 16384 (open). Parameters: \[0\]: A
/// value between 0 and 16384.
#[cfg(feature = "sunrise")]
pub const GRAPPLE_HOOK_SET: &str = "GRAPPLE_HOOK_SET";
/// Extend the lead pole.
#[cfg(feature = "sunrise")]
pub const LEAD_POLE_OFF: &str = "LEAD_POLE_OFF";
/// Retract the lead pole.
#[cfg(feature = "sunrise")]
pub const LEAD_POLE_ON: &str = "LEAD_POLE_ON";
/// Toggle the lead pole between extended and retracted.
#[cfg(feature = "sunrise")]
pub const LEAD_POLE_TOGGLE: &str = "LEAD_POLE_TOGGLE";
/// Set the lead pole extended value, between 0 (retracted) and 16384 (extended). Parameters:
/// \[0\]: A value between 0 and 16384.
#[cfg(feature = "sunrise")]
pub const LEAD_POLE_SET: &str = "LEAD_POLE_SET";

// Night Vision
/// Switches off the night vision display.
#[cfg(feature = "sunrise")]
pub const NIGHT_VISION_DISPLAY_OFF: &str = "NIGHT_VISION_DISPLAY_OFF";
/// Switches on the night vision display (assuming that the aircraft night vision Available
/// parameter is 1/true)
#[cfg(feature = "sunrise")]
pub const NIGHT_VISION_DISPLAY_ON: &str = "NIGHT_VISION_DISPLAY_ON";
/// Sets the night vision display on (1) or off (0). Note that it can only be toggled on if
/// the aircraft night vision Available parameter is 1/true Parameters: \[0\] Bool.
#[cfg(feature = "sunrise")]
pub const NIGHT_VISION_DISPLAY_SET: &str = "NIGHT_VISION_DISPLAY_SET";
/// Toggles the night vision display between on (1) or off (0,). Note that it can only be
/// toggled on if the aircraft night vision Available parameter is 1/true
#[cfg(feature = "sunrise")]
pub const NIGHT_VISION_DISPLAY_TOGGLE: &str = "NIGHT_VISION_DISPLAY_TOGGLE";
/// Increments the night vision intensity by 10 (default) or by the amount supplied, up to a
/// maximum of 300. Parameters: \[0\] Float (optional).
#[cfg(feature = "sunrise")]
pub const NIGHT_VISION_INTENSITY_DEC: &str = "NIGHT_VISION_INTENSITY_DEC";
/// Decrements the night vision intensity by 10 (default) or by the amount supplied, down to a
/// minimum of 0. Parameters: \[0\] Float (optional).
#[cfg(feature = "sunrise")]
pub const NIGHT_VISION_INTENSITY_INC: &str = "NIGHT_VISION_INTENSITY_INC";
/// Sets the night vision intensity to the given value, which should be between 0 (lowest
/// intensity) and 300 (highest intensity). Parameters: \[0\] Float.
#[cfg(feature = "sunrise")]
pub const NIGHT_VISION_INTENSITY_SET: &str = "NIGHT_VISION_INTENSITY_SET";
