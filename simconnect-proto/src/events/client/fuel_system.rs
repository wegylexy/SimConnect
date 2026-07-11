// Cross Feed
/// Opens cross feed valve (when used in conjunction with "isolate" tank)
pub const CROSS_FEED_OPEN: &str = "CROSS_FEED_OPEN";
/// Toggles crossfeed valve (when used in conjunction with "isolate" tank)
pub const CROSS_FEED_TOGGLE: &str = "CROSS_FEED_TOGGLE";
/// Closes crossfeed valve (when used in conjunction with "isolate" tank)
pub const CROSS_FEED_OFF: &str = "CROSS_FEED_OFF";
/// Sets the fuel crossfeed to be from left to right.
pub const CROSS_FEED_LEFT_TO_RIGHT: &str = "CROSS_FEED_LEFT_TO_RIGHT";
/// Sets the fuel crossfeed to be from right to left.
pub const CROSS_FEED_RIGHT_TO_LEFT: &str = "CROSS_FEED_RIGHT_TO_LEFT";

// Fuel Selector
/// Turn fuel selector 1 to the ALL position.
pub const FUEL_SELECTOR_ALL: &str = "FUEL_SELECTOR_ALL";
/// Turns selector 1 to CENTER position.
pub const FUEL_SELECTOR_CENTER: &str = "FUEL_SELECTOR_CENTER";
/// Turns selector 1 to LEFT position (fuel will be retrieved from Left Tip then Left Aux then
/// Left Main).
pub const FUEL_SELECTOR_LEFT: &str = "FUEL_SELECTOR_LEFT";
/// Turns selector 1 to LEFT AUX position.
pub const FUEL_SELECTOR_LEFT_AUX: &str = "FUEL_SELECTOR_LEFT_AUX";
/// Sets the fuel selector for engine 1 to the left Main tank.
pub const FUEL_SELECTOR_LEFT_MAIN: &str = "FUEL_SELECTOR_LEFT_MAIN";
/// Turn fuel selector 1 to the OFF position.
pub const FUEL_SELECTOR_OFF: &str = "FUEL_SELECTOR_OFF";
/// Turns selector 1 to RIGHT position (fuel will be retrieved from Right Tip then Right Aux
/// then Right Main).
pub const FUEL_SELECTOR_RIGHT: &str = "FUEL_SELECTOR_RIGHT";
/// Turns selector 1 to RIGHT AUX position.
pub const FUEL_SELECTOR_RIGHT_AUX: &str = "FUEL_SELECTOR_RIGHT_AUX";
/// Sets the fuel selector for engine 1 to the right Main tank.
pub const FUEL_SELECTOR_RIGHT_MAIN: &str = "FUEL_SELECTOR_RIGHT_MAIN";
/// Sets selector 1 position (see the Fuel Selector Codes list for the correct code to use).
/// Parameters: \[0\]: Number.
pub const FUEL_SELECTOR_SET: &str = "FUEL_SELECTOR_SET";
/// Sets fuel selector 1 to "Isolate".
pub const FUEL_SELECTOR_1_ISOLATE: &str = "FUEL_SELECTOR_1_ISOLATE";
/// Sets fuel selector 1 to "Crossfeed".
pub const FUEL_SELECTOR_1_CROSSFEED: &str = "FUEL_SELECTOR_1_CROSSFEED";
/// Turns selector 2 to ALL position.
pub const FUEL_SELECTOR_2_ALL: &str = "FUEL_SELECTOR_2_ALL";
/// Turns selector 2 to CENTER position.
pub const FUEL_SELECTOR_2_CENTER: &str = "FUEL_SELECTOR_2_CENTER";
/// Sets fuel selector 2 to "Crossfeed".
pub const FUEL_SELECTOR_2_CROSSFEED: &str = "FUEL_SELECTOR_2_CROSSFEED";
/// Sets fuel selector 2 to "Isolate".
pub const FUEL_SELECTOR_2_ISOLATE: &str = "FUEL_SELECTOR_2_ISOLATE";
/// Turns selector 2 to LEFT position (fuel will be retrieved from Left Tip then Left Aux then
/// Left Main).
pub const FUEL_SELECTOR_2_LEFT: &str = "FUEL_SELECTOR_2_LEFT";
/// Turns selector 2 to LEFT AUX position.
pub const FUEL_SELECTOR_2_LEFT_AUX: &str = "FUEL_SELECTOR_2_LEFT_AUX";
/// Sets the fuel selector for engine 2 to the left Main tank.
pub const FUEL_SELECTOR_2_LEFT_MAIN: &str = "FUEL_SELECTOR_2_LEFT_MAIN";
/// Turns selector 2 to OFF position.
pub const FUEL_SELECTOR_2_OFF: &str = "FUEL_SELECTOR_2_OFF";
/// Turns selector 2 to RIGHT position (fuel will be retrieved from Right Tip then Right Aux
/// then Right Main)
pub const FUEL_SELECTOR_2_RIGHT: &str = "FUEL_SELECTOR_2_RIGHT";
/// Turns selector 2 to RIGHT AUX position.
pub const FUEL_SELECTOR_2_RIGHT_AUX: &str = "FUEL_SELECTOR_2_RIGHT_AUX";
/// Sets the fuel selector for engine 2 to the right Main tank.
pub const FUEL_SELECTOR_2_RIGHT_MAIN: &str = "FUEL_SELECTOR_2_RIGHT_MAIN";
/// Sets selector 2 position (see the Fuel Selector Codes list for the correct code to use).
/// Parameters: \[0\]: Number.
pub const FUEL_SELECTOR_2_SET: &str = "FUEL_SELECTOR_2_SET";
/// Turns selector 3 to ALL position.
pub const FUEL_SELECTOR_3_ALL: &str = "FUEL_SELECTOR_3_ALL";
/// Turns selector 3 to CENTER position.
pub const FUEL_SELECTOR_3_CENTER: &str = "FUEL_SELECTOR_3_CENTER";
/// Sets fuel selector 3 to "Crossfeed".
pub const FUEL_SELECTOR_3_CROSSFEED: &str = "FUEL_SELECTOR_3_CROSSFEED";
/// Sets fuel selector 3 to "Isolate".
pub const FUEL_SELECTOR_3_ISOLATE: &str = "FUEL_SELECTOR_3_ISOLATE";
/// Turns selector 3 to LEFT position (fuel will be retrieved from Left Tip then Left Aux then
/// Left Main).
pub const FUEL_SELECTOR_3_LEFT: &str = "FUEL_SELECTOR_3_LEFT";
/// Turns selector 3 to LEFT AUX position.
pub const FUEL_SELECTOR_3_LEFT_AUX: &str = "FUEL_SELECTOR_3_LEFT_AUX";
/// Sets the fuel selector for engine 3 to the left Main tank.
pub const FUEL_SELECTOR_3_LEFT_MAIN: &str = "FUEL_SELECTOR_3_LEFT_MAIN";
/// Turns selector 3 to OFF position.
pub const FUEL_SELECTOR_3_OFF: &str = "FUEL_SELECTOR_3_OFF";
/// Turns selector 3 to RIGHT position (fuel will be retrieved from Right Tip then Right Aux
/// then Right Main).
pub const FUEL_SELECTOR_3_RIGHT: &str = "FUEL_SELECTOR_3_RIGHT";
/// Turns selector 3 to RIGHT AUX position.
pub const FUEL_SELECTOR_3_RIGHT_AUX: &str = "FUEL_SELECTOR_3_RIGHT_AUX";
/// Sets the fuel selector for engine 3 to the right Main tank.
pub const FUEL_SELECTOR_3_RIGHT_MAIN: &str = "FUEL_SELECTOR_3_RIGHT_MAIN";
/// Sets selector 3 position (see the Fuel Selector Codes list for the correct code to use).
/// Parameters: \[0\]: Number.
pub const FUEL_SELECTOR_3_SET: &str = "FUEL_SELECTOR_3_SET";
/// Turns selector 4 to ALL position.
pub const FUEL_SELECTOR_4_ALL: &str = "FUEL_SELECTOR_4_ALL";
/// Turns selector 4 to CENTER position.
pub const FUEL_SELECTOR_4_CENTER: &str = "FUEL_SELECTOR_4_CENTER";
/// Sets fuel selector 4 to "Crossfeed".
pub const FUEL_SELECTOR_4_CROSSFEED: &str = "FUEL_SELECTOR_4_CROSSFEED";
/// Sets fuel selector 4 to "Isolate".
pub const FUEL_SELECTOR_4_ISOLATE: &str = "FUEL_SELECTOR_4_ISOLATE";
/// Turns selector 4 to OFF position.
pub const FUEL_SELECTOR_4_OFF: &str = "FUEL_SELECTOR_4_OFF";
/// Turns selector 4 to LEFT position (fuel will be retrieved from Left Tip then Left Aux then
/// Left Main).
pub const FUEL_SELECTOR_4_LEFT: &str = "FUEL_SELECTOR_4_LEFT";
/// Turns selector 4 to LEFT AUX position.
pub const FUEL_SELECTOR_4_LEFT_AUX: &str = "FUEL_SELECTOR_4_LEFT_AUX";
/// Sets the fuel selector for engine 4 to the left Main tank.
pub const FUEL_SELECTOR_4_LEFT_MAIN: &str = "FUEL_SELECTOR_4_LEFT_MAIN";
/// Turns selector 4 to RIGHT position (fuel will be retrieved from Right Tip then Right Aux
/// then Right Main).
pub const FUEL_SELECTOR_4_RIGHT: &str = "FUEL_SELECTOR_4_RIGHT";
/// Turns selector 4 to RIGHT AUX position.
pub const FUEL_SELECTOR_4_RIGHT_AUX: &str = "FUEL_SELECTOR_4_RIGHT_AUX";
/// Sets the fuel selector for engine 4 to the right Main tank.
pub const FUEL_SELECTOR_4_RIGHT_MAIN: &str = "FUEL_SELECTOR_4_RIGHT_MAIN";
/// Sets selector 4 position (see the Fuel Selector Codes list for the correct code to use).
/// Parameters: \[0\]: Number.
pub const FUEL_SELECTOR_4_SET: &str = "FUEL_SELECTOR_4_SET";

// Fuel System
/// Set the current junction options for which lines are open or closed at any given time.
/// This event requires two parameters: the first is the index of the junction (as defined by
/// the N index of the Junction.N parameter), and the second is the Option index, which is
/// what sets the lines to open/close. Parameters: \[0\]: Junction Index \[1\]: Option index.
pub const FUELSYSTEM_JUNCTION_SET: &str = "FUELSYSTEM_JUNCTION_SET";
/// Turn a fuel pump off. The event requires the N index of the Pump.N parameter to define the
/// pump to use. Parameters: \[0\]: Pump Index.
pub const FUELSYSTEM_PUMP_OFF: &str = "FUELSYSTEM_PUMP_OFF";
/// Turn a fuel pump on. The event requires the N index of the Pump.N parameter to define the
/// pump to use. Parameters: \[0\]: Pump Index.
pub const FUELSYSTEM_PUMP_ON: &str = "FUELSYSTEM_PUMP_ON";
/// Set a fuel pump to be either on or off or auto. The event requires the N index of the
/// Pump.N parameter to define the pump to use. Parameters: \[0\]: Pump Index \[1\]: Status 0
/// = Off 1 = On 2 = Auto.
pub const FUELSYSTEM_PUMP_SET: &str = "FUELSYSTEM_PUMP_SET";
/// Toggle a fuel pump on/off. The event requires the N index of the Pump.N parameter to
/// define the pump to use. Parameters: \[0\]: Pump Index.
pub const FUELSYSTEM_PUMP_TOGGLE: &str = "FUELSYSTEM_PUMP_TOGGLE";
/// Turn a trigger event off. The event requires the N index of the Trigger.N parameter to
/// define the trigger to switch off. Parameters: \[0\]: Trigger Index.
pub const FUELSYSTEM_TRIGGER_OFF: &str = "FUELSYSTEM_TRIGGER_OFF";
/// Turn a trigger event on. The event requires the N index of the Trigger.N parameter to
/// define the trigger to switch off. Parameters: \[0\]: Trigger Index.
pub const FUELSYSTEM_TRIGGER_ON: &str = "FUELSYSTEM_TRIGGER_ON";
/// Set a trigger event to be either on or off. The event requires the N index of the
/// Trigger.N parameter to define the trigger to switch off. Parameters: \[0\]: Trigger Index
/// \[1\]: Status, either on (1) or off (0).
pub const FUELSYSTEM_TRIGGER_SET: &str = "FUELSYSTEM_TRIGGER_SET";
/// Toggle a trigger event on/off. The event requires the N index of the Trigger.N parameter
/// to define the trigger to switch off. Parameters: \[0\]: Trigger Index.
pub const FUELSYSTEM_TRIGGER_TOGGLE: &str = "FUELSYSTEM_TRIGGER_TOGGLE";
/// Close a specific valve in the fuel system. The event requires the N index of the Valve.N
/// parameter to define the valve to target. Parameters: \[0\]: Valve Index.
pub const FUELSYSTEM_VALVE_CLOSE: &str = "FUELSYSTEM_VALVE_CLOSE";
/// Open a specific valve in the fuel system. The event requires the N index of the Valve.N
/// parameter to define the valve to target. Parameters: \[0\]: Valve Index.
pub const FUELSYSTEM_VALVE_OPEN: &str = "FUELSYSTEM_VALVE_OPEN";
/// Set a valve to be either open or closed. The event requires the N index of the Valve.N
/// parameter to define the valve to target. Parameters: \[0\]: Valve Index \[1\]: Status,
/// either open (1) or closed (0).
pub const FUELSYSTEM_VALVE_SET: &str = "FUELSYSTEM_VALVE_SET";
/// Toggle a valve open/closed. The event requires the N index of the Valve.N parameter to
/// define the valve to target. Parameters: \[0\]: Valve Index.
pub const FUELSYSTEM_VALVE_TOGGLE: &str = "FUELSYSTEM_VALVE_TOGGLE";

// Fuel Transfer Keys
/// Set the fuel transfer mode to the "custom" setting. Requires that at least 1 transfer pump
/// has been defined in the flight_model.cfg file using the fuel_transfer_pump.N parameter.
pub const SET_FUEL_TRANSFER_CUSTOM: &str = "SET_FUEL_TRANSFER_CUSTOM";
/// Toggle a custom fuel transfer pump on/off. The index is the Pump ID value supplied as part
/// of the fuel pump definition for the fuel_transfer_pump.N parameter in the flight_model.cfg
/// file. Parameters: \[0\]: pump Index.
pub const FUEL_TRANSFER_CUSTOM_INDEX_TOGGLE: &str = "FUEL_TRANSFER_CUSTOM_INDEX_TOGGLE";
/// Set the fuel transfer system to use the "forward" setting, which pumps from tank 1 to tank
/// 2.
pub const SET_FUEL_TRANSFER_FORWARD: &str = "SET_FUEL_TRANSFER_FORWARD";
/// Set the fuel transfer system to use the "aft" setting, which pumps from tank 2 to tank 1.
pub const SET_FUEL_TRANSFER_AFT: &str = "SET_FUEL_TRANSFER_AFT";
/// Set the fuel transfer pump to automatically balance the fuel in tanks 1 and 2 to maintain
/// the CG.
pub const SET_FUEL_TRANSFER_AUTO: &str = "SET_FUEL_TRANSFER_AUTO";
/// Set the fuel transfer pump to off.
pub const SET_FUEL_TRANSFER_OFF: &str = "SET_FUEL_TRANSFER_OFF";

// Miscellaneous
/// Adds fuel to the aircraft, 25% of capacity by default. 0 to 65535 (max fuel) can be
/// passed. Parameters: \[0\]: The fuel quantity.
pub const ADD_FUEL_QUANTITY: &str = "ADD_FUEL_QUANTITY";
/// Set the electrical pump status for all engines. Values are as follows: 0 = Off 1 = On 2 =
/// Auto These keys are only useful when using the legacy \[FUEL\] system. Parameters: \[0\]:
/// Value.
#[cfg(feature = "sunrise")]
pub const ELECT_FUEL_PUMP_SET5: &str = "ELECT_FUEL_PUMP_SET5";
/// Set the electrical pump status for engines 1-4. Values are as follows: 0 = Off 1 = On 2 =
/// Auto These keys are only useful when using the legacy \[FUEL\] system. Parameters: \[0\]:
/// Value.
pub const ELECT_FUEL_PUMP1_SET: &str = "ELECT_FUEL_PUMP1_SET";
/// Set the electrical pump status for engines 1-4. Values are as follows: 0 = Off 1 = On 2 =
/// Auto These keys are only useful when using the legacy \[FUEL\] system. Parameters: \[0\]:
/// Value.
pub const ELECT_FUEL_PUMP2_SET: &str = "ELECT_FUEL_PUMP2_SET";
/// Set the electrical pump status for engines 1-4. Values are as follows: 0 = Off 1 = On 2 =
/// Auto These keys are only useful when using the legacy \[FUEL\] system. Parameters: \[0\]:
/// Value.
pub const ELECT_FUEL_PUMP3_SET: &str = "ELECT_FUEL_PUMP3_SET";
/// Set the electrical pump status for engines 1-4. Values are as follows: 0 = Off 1 = On 2 =
/// Auto These keys are only useful when using the legacy \[FUEL\] system. Parameters: \[0\]:
/// Value.
pub const ELECT_FUEL_PUMP4_SET: &str = "ELECT_FUEL_PUMP4_SET";
/// Not currently used in the simulation.
pub const ENGINE_FUELFLOW_BUG_POSITION1: &str = "ENGINE_FUELFLOW_BUG_POSITION1";
/// Not currently used in the simulation.
pub const ENGINE_FUELFLOW_BUG_POSITION2: &str = "ENGINE_FUELFLOW_BUG_POSITION2";
/// Not currently used in the simulation.
pub const ENGINE_FUELFLOW_BUG_POSITION3: &str = "ENGINE_FUELFLOW_BUG_POSITION3";
/// Not currently used in the simulation.
pub const ENGINE_FUELFLOW_BUG_POSITION4: &str = "ENGINE_FUELFLOW_BUG_POSITION4";
/// Set to 1 (TRUE) or 0 (FALSE). The switch can only be set to TRUE if fuel_dump_rate is
/// specified in the aircraft configuration file, indicating that a fuel dump system exists.
/// This key is only useful when using the legacy \[FUEL\] system. Parameters: \[0\]: Bool.
pub const FUEL_DUMP_SWITCH_SET: &str = "FUEL_DUMP_SWITCH_SET";
/// Used to turn on (1, TRUE) or off (0, FALSE) the fuel dump switch. This key is only useful
/// when using the legacy \[FUEL\] system.
pub const FUEL_DUMP_TOGGLE: &str = "FUEL_DUMP_TOGGLE";
/// Activate the manual fuel pressure pump. Used for both modern \[FUEL_SYSTEM\] and legacy
/// \[FUEL\] systems.
pub const MANUAL_FUEL_PRESSURE_PUMP: &str = "MANUAL_FUEL_PRESSURE_PUMP";
/// Set the position of the fuel manual pump handle, as a percentage. This key is only useful
/// when using the modern \[FUEL_SYSTEM\]. Parameters: \[0\]: The pump index \[1\]: A value
/// between 0 and 16384.
pub const MANUAL_FUEL_PRESSURE_PUMP_SET: &str = "MANUAL_FUEL_PRESSURE_PUMP_SET";
/// When set to 1 (TRUE) it sets the fuel transfer mode to manual. Parameters: \[0\]: Bool.
pub const MANUAL_FUEL_TRANSFER: &str = "MANUAL_FUEL_TRANSFER";
/// Release all external tanks that can be jettisoned.
pub const RELEASE_DROP_TANK_ALL: &str = "RELEASE_DROP_TANK_ALL";
/// Release the first external tank that can be jettisoned.
pub const RELEASE_DROP_TANK_1: &str = "RELEASE_DROP_TANK_1";
/// Release the second external tank that can be jettisoned.
pub const RELEASE_DROP_TANK_2: &str = "RELEASE_DROP_TANK_2";
/// Fully repair and refuel the user aircraft. Ignored if flight realism is enforced.
pub const REPAIR_AND_REFUEL: &str = "REPAIR_AND_REFUEL";
/// Request a fuel truck. The aircraft must be in a parking spot for this to be successful.
pub const REQUEST_FUEL_KEY: &str = "REQUEST_FUEL_KEY";
/// Toggle the anti-detonation valve. Pass a value to determine which tank to use if there are
/// multiple tanks. See the Fuel Selector Codes list for the correct tank code to use. Note
/// that this key requires the \[ANTIDETONATION_SYSTEM.N\] system to have been set up in the
/// engine configuration file. Parameters: \[0\]: Tank index (optional).
pub const ANTIDETONATION_TANK_VALVE_TOGGLE: &str = "ANTIDETONATION_TANK_VALVE_TOGGLE";
/// Toggle the nitrous valve. Pass a value to determine which tank to use if there are
/// multiple tanks. See the Fuel Selector Codes list for the correct tank code to use. Note
/// that this key requires the \[NITROUS SYSTEM.N\] system to have been set up in the engine
/// configuration file. Parameters: \[0\]: Tank index (optional).
pub const NITROUS_TANK_VALVE_TOGGLE: &str = "NITROUS_TANK_VALVE_TOGGLE";
