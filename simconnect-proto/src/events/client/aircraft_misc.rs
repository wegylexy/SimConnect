// Aircraft Failures
/// Not currently used in the simulation. Parameters: KEY_LOW_HEIGHT_WARNING_GAUGE_WILL_SET or
/// KEY_LOW_HIGHT_WARNING_GAUGE_WILL_SET.
pub const LOW_HEIGHT_WARNING_GAUGE_WILL_SET: &str = "LOW_HEIGHT_WARNING_GAUGE_WILL_SET";
/// Not currently used in the simulation. Parameters: KEY_LOW_HEIGHT_WARNING_GAUGE_WILL_SET or
/// KEY_LOW_HIGHT_WARNING_GAUGE_WILL_SET.
pub const LOW_HIGHT_WARNING_GAUGE_WILL_SET: &str = "LOW_HIGHT_WARNING_GAUGE_WILL_SET";
/// Not currently used in the simulation. Parameters: KEY_LOW_HEIGHT_WARNING_SET or
/// KEY_LOW_HIGHT_WARNING_SET.
pub const LOW_HEIGHT_WARNING_SET: &str = "LOW_HEIGHT_WARNING_SET";
/// Not currently used in the simulation. Parameters: KEY_LOW_HEIGHT_WARNING_SET or
/// KEY_LOW_HIGHT_WARNING_SET.
pub const LOW_HIGHT_WARNING_SET: &str = "LOW_HIGHT_WARNING_SET";
/// Acknowledges a master warning system caution alert.
pub const MASTER_CAUTION_ACKNOWLEDGE: &str = "MASTER_CAUTION_ACKNOWLEDGE";
/// Switches off a master warning system caution alert.
pub const MASTER_CAUTION_OFF: &str = "MASTER_CAUTION_OFF";
/// Switches on a master warning system caution alert.
pub const MASTER_CAUTION_ON: &str = "MASTER_CAUTION_ON";
/// Set on or off a master warning system caution alert. Parameters: \[0\]: bool.
pub const MASTER_CAUTION_SET: &str = "MASTER_CAUTION_SET";
/// Toggle on or off a master warning system caution alert.
pub const MASTER_CAUTION_TOGGLE: &str = "MASTER_CAUTION_TOGGLE";
/// Acknowldeges a master warning system warning alert.
pub const MASTER_WARNING_ACKNOWLEDGE: &str = "MASTER_WARNING_ACKNOWLEDGE";
/// Switches off a master warning system warning alert.
pub const MASTER_WARNING_OFF: &str = "MASTER_WARNING_OFF";
/// Switches on a master warning system warning alert.
pub const MASTER_WARNING_ON: &str = "MASTER_WARNING_ON";
/// Set on or off a master warning system warning alert. Parameters: \[0\]: bool.
pub const MASTER_WARNING_SET: &str = "MASTER_WARNING_SET";
/// Toggle on or off a master warning system warning alert.
pub const MASTER_WARNING_TOGGLE: &str = "MASTER_WARNING_TOGGLE";
/// Toggle electrical system failure
pub const TOGGLE_ELECTRICAL_FAILURE: &str = "TOGGLE_ELECTRICAL_FAILURE";
/// Toggle engine 1/2/3/4 failure Parameters: TOGGLE_ENGINE1_FAILURE TOGGLE_ENGINE2_FAILURE
/// TOGGLE_ENGINE3_FAILURE TOGGLE_ENGINE4_FAILURE.
pub const TOGGLE_ENGINE1_FAILURE: &str = "TOGGLE_ENGINE1_FAILURE";
/// Toggle engine 1/2/3/4 failure Parameters: TOGGLE_ENGINE1_FAILURE TOGGLE_ENGINE2_FAILURE
/// TOGGLE_ENGINE3_FAILURE TOGGLE_ENGINE4_FAILURE.
pub const TOGGLE_ENGINE2_FAILURE: &str = "TOGGLE_ENGINE2_FAILURE";
/// Toggle engine 1/2/3/4 failure Parameters: TOGGLE_ENGINE1_FAILURE TOGGLE_ENGINE2_FAILURE
/// TOGGLE_ENGINE3_FAILURE TOGGLE_ENGINE4_FAILURE.
pub const TOGGLE_ENGINE3_FAILURE: &str = "TOGGLE_ENGINE3_FAILURE";
/// Toggle engine 1/2/3/4 failure Parameters: TOGGLE_ENGINE1_FAILURE TOGGLE_ENGINE2_FAILURE
/// TOGGLE_ENGINE3_FAILURE TOGGLE_ENGINE4_FAILURE.
pub const TOGGLE_ENGINE4_FAILURE: &str = "TOGGLE_ENGINE4_FAILURE";
/// Toggles hydraulic system failure
pub const TOGGLE_HYDRAULIC_FAILURE: &str = "TOGGLE_HYDRAULIC_FAILURE";
/// Toggles left brake failure
pub const TOGGLE_LEFT_BRAKE_FAILURE: &str = "TOGGLE_LEFT_BRAKE_FAILURE";
/// Toggles blocked pitot tube
pub const TOGGLE_PITOT_BLOCKAGE: &str = "TOGGLE_PITOT_BLOCKAGE";
/// Toggles right brake failure
pub const TOGGLE_RIGHT_BRAKE_FAILURE: &str = "TOGGLE_RIGHT_BRAKE_FAILURE";
/// Toggles blocked static port
pub const TOGGLE_STATIC_PORT_BLOCKAGE: &str = "TOGGLE_STATIC_PORT_BLOCKAGE";
/// Toggles brake failure (both)
pub const TOGGLE_TOTAL_BRAKE_FAILURE: &str = "TOGGLE_TOTAL_BRAKE_FAILURE";
/// Toggle vacuum system failure
pub const TOGGLE_VACUUM_FAILURE: &str = "TOGGLE_VACUUM_FAILURE";

// Gliders
/// Decrements the MacCready setting. Default decrement value is 0.1m/s, however holding down
/// the key for more than 1 second will increase the amount to 0.5m/s, and holding it down for
/// more than 2 seconds will further increase this to 1m/s. Note that the resulting value is
/// clamped between 0 and 5 m/s.
pub const MAC_CREADY_SETTING_DEC: &str = "MAC_CREADY_SETTING_DEC";
/// Increments the MacCready setting. Default increment value is 0.1m/s, however holding down
/// the key for more than 1 second will increase the amount to 0.5m/s, and holding it down for
/// more than 2 seconds will further increase this to 1m/s. Note that the resulting value is
/// clamped between 0 and 5 m/s.
pub const MAC_CREADY_SETTING_INC: &str = "MAC_CREADY_SETTING_INC";
/// Set the MacCready setting to a value between 0 and 5 m/s. Parameters: \[0\]: MacCready
/// value in m/s.
pub const MAC_CREADY_SETTING_SET: &str = "MAC_CREADY_SETTING_SET";
/// Sets the tail hook handle. Takes one of the following values: 1 - set tail hook 0 -
/// retract tail hook Parameters: \[0\]: TRUE/FALSE to set or retract the tailhook..
pub const SET_TAIL_HOOK_HANDLE: &str = "SET_TAIL_HOOK_HANDLE";
/// Toggles tail hook.
pub const TOGGLE_TAIL_HOOK_HANDLE: &str = "TOGGLE_TAIL_HOOK_HANDLE";
/// Release a towed aircraft, usually a glider.
pub const TOW_PLANE_RELEASE: &str = "TOW_PLANE_RELEASE";
/// Request a tow plane. The user aircraft must be tow-able, stationary, on the ground and not
/// already attached for this to succeed.
pub const TOW_PLANE_REQUEST: &str = "TOW_PLANE_REQUEST";

// Doors
/// This event can be used to close all the aircraft doors setup using interactive points of
/// the following types: Main exit (0) Cargo exit/door (1) Emergency exit (2) Drop Exit (7)
/// The event requires 2 parameters where: the first is the interactive point name (or index +
/// 1). Note that giving 0 here will target all doors, and index values are offset by 1, so
/// interactive_point.0 is index 1, interactive_point.1 is index 2, etc... the second is a
/// boolean used to control whether the full animation should be played (0) or not (1). If set
/// to false (1), then the animation will be skipped and the door will instantly change state.
/// Parameters: \[0\] Door name (or index) \[1\] Skip Animation (Bool).
#[cfg(feature = "sunrise")]
pub const CLOSE_AIRCRAFT_DOORS: &str = "CLOSE_AIRCRAFT_DOORS";
/// This event can be used to close all the aircraft doors that are set up to take damage when
/// the aircraft speed is above the Exit Open Failure Speed (regardless of the actual speed of
/// the aircraft at the time the event is called). All doors must be setup using interactive
/// points of the following types: Main exit (0) Cargo exit/door (1) Emergency exit (2) Drop
/// Exit (7) The event requires 2 parameters where: the first is the interactive point name
/// (or index + 1). Note that giving 0 here will target all doors, and index values are offset
/// by 1, so interactive_point.0 is index 1, interactive_point.1 is index 2, etc... the second
/// is a boolean used to control whether the full animation should be played (0) or not (1).
/// If set to false (1), then the animation will be skipped and the door will instantly change
/// state. Parameters: \[0\] Door name (or index) \[1\] Skip Animation (Bool).
#[cfg(feature = "sunrise")]
pub const CLOSE_AIRCRAFT_DOORS_CRASHING: &str = "CLOSE_AIRCRAFT_DOORS_CRASHING";
/// This event can be used to open all the aircraft doors setup using interactive points of
/// the following types: Main exit (0) Cargo exit/door (1) Emergency exit (2) Drop Exit (7)
/// The event requires 2 parameters where: the first is the interactive point name (or index +
/// 1). Note that giving 0 here will target all doors, and index values are offset by 1, so
/// interactive_point.0 is index 1, interactive_point.1 is index 2, etc... the second is a
/// boolean used to control whether the full animation should be played (0) or not (1). If set
/// to false (1), then the animation will be skipped and the door will instantly change state.
/// Parameters: \[0\] Door name (or index) \[1\] Skip Animation (Bool).
#[cfg(feature = "sunrise")]
pub const OPEN_AIRCRAFT_DOORS: &str = "OPEN_AIRCRAFT_DOORS";
/// This event can be used to set the state of all the aircraft doors setup using interactive
/// points of the following types: Main exit (0) Cargo exit/door (1) Emergency exit (2) Drop
/// Exit (7) The event requires 3 parameters where: the first is the interactive point name
/// (or index + 1). Note that giving 0 here will target all doors, and index values are offset
/// by 1, so interactive_point.0 is index 1, interactive_point.1 is index 2, etc... the second
/// is a boolean used to control whether the full animation should be played (0) or not (1).
/// If set to false (1), then the animation will be skipped and the door will instantly change
/// state. the third is a 16k value which corresponds to the new state of the door, where 0 is
/// fully closed (the default), and 16384 is fully open. Parameters: \[0\] Door name (or
/// index) \[1\] Skip Animation (Bool) \[2\] Door state (0 - 16384).
#[cfg(feature = "sunrise")]
pub const SET_AIRCRAFT_DOORS: &str = "SET_AIRCRAFT_DOORS";
/// Toggle the state of all doors (eg: Main Exit, Cargo Exit, Emergency Exit or Drop Exit).
/// Note that If the state of each door is different, they will still all end up with the same
/// state as the first valid exit by declaration.
#[cfg(feature = "sunrise")]
pub const TOGGLE_ALL_AIRCRAFT_DOORS: &str = "TOGGLE_ALL_AIRCRAFT_DOORS";
/// Toggles primary door open/close. Usually followed by (for example) KEY_SELECT_2, etc...
/// for subsequent doors.
pub const TOGGLE_AIRCRAFT_EXIT: &str = "TOGGLE_AIRCRAFT_EXIT";
/// Toggles primary door open/close at double the default speed. Usually followed by (for
/// example) KEY_SELECT_2, etc... for subsequent doors.
pub const TOGGLE_AIRCRAFT_EXIT_FAST: &str = "TOGGLE_AIRCRAFT_EXIT_FAST";

// Miscellaneous
/// Turns off (0) the annunciator switch.
pub const ANNUNCIATOR_SWITCH_OFF: &str = "ANNUNCIATOR_SWITCH_OFF";
/// Turns on (1) the annunciator switch.
pub const ANNUNCIATOR_SWITCH_ON: &str = "ANNUNCIATOR_SWITCH_ON";
/// Toggles the annunciator switch off (0) and on (1).
pub const ANNUNCIATOR_SWITCH_TOGGLE: &str = "ANNUNCIATOR_SWITCH_TOGGLE";
/// Not used by the simulation Parameters: -.
pub const BAIL_OUT: &str = "BAIL_OUT";
/// Increases the bleed air source control. Order of operation is Auto -> Off -> APU ->
/// Engines.
pub const BLEED_AIR_SOURCE_CONTROL_INC: &str = "BLEED_AIR_SOURCE_CONTROL_INC";
/// Decreases the bleed air source control. Order of operation is Engines -> APU -> Off ->
/// Auto.
pub const BLEED_AIR_SOURCE_CONTROL_DEC: &str = "BLEED_AIR_SOURCE_CONTROL_DEC";
/// Sets the bleed air system source. The input parameter \[0\] can be one of the following: 0
/// - auto 1 - off 2 - apu 3 - engines Parameters: \[0\]: source value.
pub const BLEED_AIR_SOURCE_CONTROL_SET: &str = "BLEED_AIR_SOURCE_CONTROL_SET";
/// Turn the "No smoking" alert on or off.
pub const CABIN_NO_SMOKING_ALERT_SWITCH_TOGGLE: &str = "CABIN_NO_SMOKING_ALERT_SWITCH_TOGGLE";
/// Turn the "Fasten seatbelts" alert on or off.
pub const CABIN_SEATBELTS_ALERT_SWITCH_TOGGLE: &str = "CABIN_SEATBELTS_ALERT_SWITCH_TOGGLE";
/// Sets the state of the specified "generic" cover. Requires the cover index and the state,
/// either 0 (off) or 1 (on). Generic covers are indexed from 1, however you can use 0 for the
/// index to set all generic covers if required. For more information, please see here:
/// Preflight Parameters: \[0\]: Cover Index \[1\]: Boolean.
#[cfg(feature = "sunrise")]
pub const COVER_GENERIC_SET: &str = "COVER_GENERIC_SET";
/// Set the state of any of the defined covers for the aircraft. Requires the cover ID and the
/// state, either 0 (off) or 1 (on). Cover IDs are as follows: Chock = 0 Engine = 1 Pitot = 2
/// StaticPort = 3 Rotor = 4 LandingGear = 5 Propeller = 6 windshield = 7 Note that this key
/// event will only work while the character state is in preflight. For more information,
/// please see here: Preflight Parameters: \[0\]: Cover ID \[1\]: Boolean.
#[cfg(feature = "sunrise")]
pub const COVER_SET: &str = "COVER_SET";
/// Decrements the AGL decision height reference by 1m.
pub const DECREASE_DECISION_HEIGHT: &str = "DECREASE_DECISION_HEIGHT";
/// Increments the AGL decision height reference by 1m.
pub const INCREASE_DECISION_HEIGHT: &str = "INCREASE_DECISION_HEIGHT";
/// Set the AGL decision height reference, in meters. Parameters: \[0\]: height (m).
pub const DECISION_HEIGHT_SET: &str = "DECISION_HEIGHT_SET";
/// Decrements the MSL decision height reference by the amount given, or by 10m if no amount
/// is given. Parameters: \[0\]: amount.
pub const DECREASE_DECISION_ALTITUDE_MSL: &str = "DECREASE_DECISION_ALTITUDE_MSL";
/// Increments the MSL decision height reference by the amount given, or by 10m if no amount
/// is given. Parameters: \[0\]: amount.
pub const INCREASE_DECISION_ALTITUDE_MSL: &str = "INCREASE_DECISION_ALTITUDE_MSL";
/// Set the MSL decision height reference, in meters. Parameters: \[0\]: height (m).
pub const SET_DECISION_ALTITUDE_MSL: &str = "SET_DECISION_ALTITUDE_MSL";
/// This key event requires a two digit number for parameter \[0\]. The first digit represents
/// the fire extinguisher index to use, and the second represents the engine index. For
/// example, a value of 11 would represent using bottle 1 on engine 1. 21 would represent
/// using bottle 2 on engine 1. Typical entries for a twin engine aircraft would be 11 and 22.
/// Parameters: \[0\]: combined index (see description).
pub const EXTINGUISH_ENGINE_FIRE: &str = "EXTINGUISH_ENGINE_FIRE";
/// Trigger the aircraft horn.
pub const HORN_TRIGGER: &str = "HORN_TRIGGER";
/// Exit Cockpit interaction mode.
#[cfg(feature = "sunrise")]
pub const INTERACTION_UNLOCK: &str = "INTERACTION_UNLOCK";
/// Turns the pitot heat switch off.
pub const PITOT_HEAT_OFF: &str = "PITOT_HEAT_OFF";
/// Turns the pitot heat switch on.
pub const PITOT_HEAT_ON: &str = "PITOT_HEAT_ON";
/// Sets the pitot heat switch on/off. Parameters: \[0\]: TRUE/FALSE to set or the pitot heat
/// switch on/off \[1\]: Pitot index.
pub const PITOT_HEAT_SET: &str = "PITOT_HEAT_SET";
/// Toggles the pitot heat switch.
pub const PITOT_HEAT_TOGGLE: &str = "PITOT_HEAT_TOGGLE";
/// Toggles pushback.
pub const TOGGLE_PUSHBACK: &str = "TOGGLE_PUSHBACK";
/// Release one dropable object. Multiple key events will release multiple objects.
pub const RELEASE_DROPPABLE_OBJECTS: &str = "RELEASE_DROPPABLE_OBJECTS";
/// (no description provided by the vendor docs)
pub const SCRIPT_EVENT_1: &str = "SCRIPT_EVENT_1";
/// (no description provided by the vendor docs)
pub const SCRIPT_EVENT_2: &str = "SCRIPT_EVENT_2";
/// (no description provided by the vendor docs)
pub const SEE_OWN_AC_OFF: &str = "SEE_OWN_AC_OFF";
/// (no description provided by the vendor docs)
pub const SEE_OWN_AC_ON: &str = "SEE_OWN_AC_ON";
/// (no description provided by the vendor docs) Parameters: \[0\]: Value.
pub const SEE_OWN_AC_SET: &str = "SEE_OWN_AC_SET";
/// (no description provided by the vendor docs)
pub const SEE_OWN_AC_TOGGLE: &str = "SEE_OWN_AC_TOGGLE";
/// Sets the wings into the folded position suitable for storage, typically on a carrier.
/// Takes one of the following values: 1 -fold wings 0 - unfold wings Parameters: \[0\]:
/// TRUE/FALSE to fold or unfold wings..
pub const SET_WING_FOLD: &str = "SET_WING_FOLD";
/// Turns the smoke system off.
pub const SMOKE_OFF: &str = "SMOKE_OFF";
/// Turns the smoke system on.
pub const SMOKE_ON: &str = "SMOKE_ON";
/// Sets smoke system on/off. Parameters: \[0\]: TRUE/FALSE to enable/disable the smoke
/// system.
pub const SMOKE_SET: &str = "SMOKE_SET";
/// Toggle smoke system switch.
pub const SMOKE_TOGGLE: &str = "SMOKE_TOGGLE";
/// Toggles alternate static pressure port.
pub const TOGGLE_ALTERNATE_STATIC: &str = "TOGGLE_ALTERNATE_STATIC";
/// Toggles structural deice switch.
pub const TOGGLE_STRUCTURAL_DEICE: &str = "TOGGLE_STRUCTURAL_DEICE";
/// Toggles tail wheel lock.
pub const TOGGLE_TAILWHEEL_LOCK: &str = "TOGGLE_TAILWHEEL_LOCK";
/// Turn the indexed water ballast valve on or off. Parameters: \[0\]: valve index from 1 to n
/// where n is the NumberOfReleaseValves defined in the systems.cfg file..
pub const TOGGLE_WATER_BALLAST_VALVE: &str = "TOGGLE_WATER_BALLAST_VALVE";
/// Toggles water rudders.
pub const TOGGLE_WATER_RUDDER: &str = "TOGGLE_WATER_RUDDER";
/// Toggles wing folding.
pub const TOGGLE_WING_FOLD: &str = "TOGGLE_WING_FOLD";
/// When used, this will remove all defined covers (generic and standard).
#[cfg(feature = "sunrise")]
pub const TOOLS_QUICK_PREFLIGHT: &str = "TOOLS_QUICK_PREFLIGHT";
/// Disables tug.
pub const TUG_DISABLE: &str = "TUG_DISABLE";
/// Triggers the tug and sets the desired heading. The units are a 32 bit integer (0 to
/// 4294967295) which represent 0 to 360 degrees. To set a 45 degree angle, for example, set
/// the value to 4294967295 / 8. Parameters: \[0\]: Heading (0 - 4294967295.
pub const TUG_HEADING: &str = "TUG_HEADING";
/// Triggers tug, and sets desired speed, in feet per second. The speed can be either positive
/// (forward movement) or negative (backward movement). Parameters: \[0\]: Speed (ft / s).
pub const TUG_SPEED: &str = "TUG_SPEED";
/// Toggle emergency power system.
pub const WAR_EMERGENCY_POWER: &str = "WAR_EMERGENCY_POWER";

// Sim Control
/// This will raise the aircraft off the ground and into flight, or - if already in flight -
/// it will force the aircraft to gain height.
pub const BACK_TO_FLY: &str = "BACK_TO_FLY";

// Skydiving
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SKYDIVE_DOORLIGHTS_DEC: &str = "SKYDIVE_DOORLIGHTS_DEC";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SKYDIVE_DOORLIGHTS_GETREADY: &str = "SKYDIVE_DOORLIGHTS_GETREADY";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SKYDIVE_DOORLIGHTS_INC: &str = "SKYDIVE_DOORLIGHTS_INC";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SKYDIVE_DOORLIGHTS_JUMP: &str = "SKYDIVE_DOORLIGHTS_JUMP";
/// This event is unbound in the simulation and as such can be used for a variety of things
/// depending on aircraft specific model behaviors. Parameters: User Defined.
#[cfg(feature = "sunrise")]
pub const SKYDIVE_DOORLIGHTS_OFF: &str = "SKYDIVE_DOORLIGHTS_OFF";

// Cabin Pressurization
/// Increases the altitude that the cabin is pressurized to by approximately 50ft.
pub const PRESSURIZATION_PRESSURE_ALT_INC: &str = "PRESSURIZATION_PRESSURE_ALT_INC";
/// Decreases the altitude that the cabin is pressurized to by approximately 50ft.
pub const PRESSURIZATION_PRESSURE_ALT_DEC: &str = "PRESSURIZATION_PRESSURE_ALT_DEC";
/// Increment the cabin pressurization by approximately 50ft/min step, based on the
/// initialisation value of 500ft/min.
pub const PRESSURIZATION_CLIMB_RATE_INC: &str = "PRESSURIZATION_CLIMB_RATE_INC";
/// Decrement the cabin pressurization by approximately 50ft/min steps based on the
/// initialisation value of 500ft/min.
pub const PRESSURIZATION_CLIMB_RATE_DEC: &str = "PRESSURIZATION_CLIMB_RATE_DEC";
/// Sets the cabin pressurization. Parameters: \[0\]: Value.
pub const PRESSURIZATION_CLIMB_RATE_SET: &str = "PRESSURIZATION_CLIMB_RATE_SET";
/// Toggles the pressure dump switch between on (sets the cabin pressure to the outside air
/// pressure) and off.
pub const PRESSURIZATION_PRESSURE_DUMP_SWITCH: &str = "PRESSURIZATION_PRESSURE_DUMP_SWITCH";

// Nose Wheel Steering
/// Sets the value of the nose wheel steering position. Zero is straight ahead (-16384, far
/// left +16384, far right). Parameters: \[0\]: Steering position (+/-16384).
pub const AXIS_STEERING_SET: &str = "AXIS_STEERING_SET";
/// Set the steering angle limit for the nose wheel. -180° maps to -16383 and 180° maps to
/// 16383. Parameters: \[0\]: Steering position (+/-16383).
pub const NOSE_WHEEL_STEERING_LIMIT_SET: &str = "NOSE_WHEEL_STEERING_LIMIT_SET";
/// Increments the nose wheel steering position by 5 percent.
pub const STEERING_INC: &str = "STEERING_INC";
/// Decrements the nose wheel steering position by 5 percent.
pub const STEERING_DEC: &str = "STEERING_DEC";
/// Sets the value of the nose wheel steering position. Zero is straight ahead (-16383, far
/// left +16383, far right). Parameters: \[0\]: Steering position (+/-16383).
pub const STEERING_SET: &str = "STEERING_SET";

// Windshield De-Ice
/// Switches on the windshield deicing system.
pub const WINDSHIELD_DEICE_OFF: &str = "WINDSHIELD_DEICE_OFF";
/// Switches off the windshield deicing system.
pub const WINDSHIELD_DEICE_ON: &str = "WINDSHIELD_DEICE_ON";
/// Sets the windshield deicing system on or off based on the input parameter \[0\].
/// Parameters: \[0\]: Bool.
pub const WINDSHIELD_DEICE_SET: &str = "WINDSHIELD_DEICE_SET";
/// Toggles the windshield deicing system on and off.
pub const WINDSHIELD_DEICE_TOGGLE: &str = "WINDSHIELD_DEICE_TOGGLE";

// Catapult Launches
/// Deploy or remove the launch assist arm.
pub const TAKEOFF_ASSIST_ARM_TOGGLE: &str = "TAKEOFF_ASSIST_ARM_TOGGLE";
/// Used to set or unset the launch assist arm. Parameters: \[0\]: Bool.
pub const TAKEOFF_ASSIST_ARM_SET: &str = "TAKEOFF_ASSIST_ARM_SET";
/// If everything is set up correctly. Launch from the catapult.
pub const TAKEOFF_ASSIST_FIRE: &str = "TAKEOFF_ASSIST_FIRE";
/// Toggle the request for the launch bar to be installed or removed.
pub const TOGGLE_LAUNCH_BAR_SWITCH: &str = "TOGGLE_LAUNCH_BAR_SWITCH";
/// Set the switch of the launch bar extension system to be on or off. Parameters: \[0\]:
/// Bool.
pub const SET_LAUNCH_BAR_SWITCH: &str = "SET_LAUNCH_BAR_SWITCH";

// Weapons
/// Not used in the simulation.
pub const GUNSIGHT_SEL: &str = "GUNSIGHT_SEL";
/// Not used in the simulation.
pub const GUNSIGHT_TOGGLE: &str = "GUNSIGHT_TOGGLE";
/// Not used in the simulation.
pub const FIRE_ALL_GUNS: &str = "FIRE_ALL_GUNS";
/// Not used in the simulation.
pub const FIRE_PRIMARY_GUNS: &str = "FIRE_PRIMARY_GUNS";
/// Not used in the simulation.
pub const FIRE_SECONDARY_GUNS: &str = "FIRE_SECONDARY_GUNS";
/// Not used in the simulation.
pub const STOP_PRIMARY_GUNS: &str = "STOP_PRIMARY_GUNS";
/// Not used in the simulation.
pub const STOP_SECONDARY_GUNS: &str = "STOP_SECONDARY_GUNS";
/// Not used in the simulation.
pub const STOP_ALL_GUNS: &str = "STOP_ALL_GUNS";
