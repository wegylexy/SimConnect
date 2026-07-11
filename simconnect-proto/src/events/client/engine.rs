// Anti-Ice
/// Sets engine anti-ice switch. Controlled engines are set through the SimVar ENGINE CONTROL
/// SELECT. Parameters: \[0\]: Position (0 - 16383).
pub const ANTI_ICE_GRADUAL_SET: &str = "ANTI_ICE_GRADUAL_SET";
/// Sets the engine 1/2/3/4 anti-ice switch to a value. Parameters: \[0\]: Position (0 -
/// 16383).
pub const ANTI_ICE_GRADUAL_SET_ENG1: &str = "ANTI_ICE_GRADUAL_SET_ENG1";
/// Sets the engine 1/2/3/4 anti-ice switch to a value. Parameters: \[0\]: Position (0 -
/// 16383).
pub const ANTI_ICE_GRADUAL_SET_ENG2: &str = "ANTI_ICE_GRADUAL_SET_ENG2";
/// Sets the engine 1/2/3/4 anti-ice switch to a value. Parameters: \[0\]: Position (0 -
/// 16383).
pub const ANTI_ICE_GRADUAL_SET_ENG3: &str = "ANTI_ICE_GRADUAL_SET_ENG3";
/// Sets the engine 1/2/3/4 anti-ice switch to a value. Parameters: \[0\]: Position (0 -
/// 16383).
pub const ANTI_ICE_GRADUAL_SET_ENG4: &str = "ANTI_ICE_GRADUAL_SET_ENG4";
/// Sets anti-ice switches on. Controlled engines are set through the SimVar ENGINE CONTROL
/// SELECT.
pub const ANTI_ICE_ON: &str = "ANTI_ICE_ON";
/// Sets anti-ice switches off. Controlled engines are set through the SimVar ENGINE CONTROL
/// SELECT.
pub const ANTI_ICE_OFF: &str = "ANTI_ICE_OFF";
/// Sets anti-ice switches on (1) or off (0). Controlled engines are set through the SimVar
/// ENGINE CONTROL SELECT. Parameters: \[0\]: Bool.
pub const ANTI_ICE_SET: &str = "ANTI_ICE_SET";
/// Sets engine 1/2/3/4 anti-ice switch (0,1) Parameters: \[0\]: Bool.
pub const ANTI_ICE_SET_ENG1: &str = "ANTI_ICE_SET_ENG1";
/// Sets engine 1/2/3/4 anti-ice switch (0,1) Parameters: \[0\]: Bool.
pub const ANTI_ICE_SET_ENG2: &str = "ANTI_ICE_SET_ENG2";
/// Sets engine 1/2/3/4 anti-ice switch (0,1) Parameters: \[0\]: Bool.
pub const ANTI_ICE_SET_ENG3: &str = "ANTI_ICE_SET_ENG3";
/// Sets engine 1/2/3/4 anti-ice switch (0,1) Parameters: \[0\]: Bool.
pub const ANTI_ICE_SET_ENG4: &str = "ANTI_ICE_SET_ENG4";
/// Toggle anti-ice switches. Controlled engines are set through the SimVar ENGINE CONTROL
/// SELECT.
pub const ANTI_ICE_TOGGLE: &str = "ANTI_ICE_TOGGLE";
/// Toggle engine 1/2/3/4 anti-ice switch on (1) or off (0).
pub const ANTI_ICE_TOGGLE_ENG1: &str = "ANTI_ICE_TOGGLE_ENG1";
/// Toggle engine 1/2/3/4 anti-ice switch on (1) or off (0).
pub const ANTI_ICE_TOGGLE_ENG2: &str = "ANTI_ICE_TOGGLE_ENG2";
/// Toggle engine 1/2/3/4 anti-ice switch on (1) or off (0).
pub const ANTI_ICE_TOGGLE_ENG3: &str = "ANTI_ICE_TOGGLE_ENG3";
/// Toggle engine 1/2/3/4 anti-ice switch on (1) or off (0).
pub const ANTI_ICE_TOGGLE_ENG4: &str = "ANTI_ICE_TOGGLE_ENG4";

// Condition Lever
/// Control the mixture axis (-100 to 100%). This is simply an alias for the AXIS_MIXTURE_SET
/// key event. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all
/// engines. \[1\]: Value.
#[cfg(feature = "sunrise")]
pub const AXIS_CONDITION_LEVER_SET: &str = "AXIS_CONDITION_LEVER_SET";
/// Sets the condition lever position based on the percentage final value of axis the input
/// where: 0% - 33.3% = cutoff 33.3% - 66.6% = low idle 66.6% - 100% = high Parameters: \[0\]:
/// Axis value.
pub const AXIS_CONDITION_LEVER_1_SET: &str = "AXIS_CONDITION_LEVER_1_SET";
/// Sets the condition lever position based on the percentage final value of axis the input
/// where: 0% - 33.3% = cutoff 33.3% - 66.6% = low idle 66.6% - 100% = high Parameters: \[0\]:
/// Axis value.
pub const AXIS_CONDITION_LEVER_2_SET: &str = "AXIS_CONDITION_LEVER_2_SET";
/// Sets the condition lever position based on the percentage final value of axis the input
/// where: 0% - 33.3% = cutoff 33.3% - 66.6% = low idle 66.6% - 100% = high Parameters: \[0\]:
/// Axis value.
pub const AXIS_CONDITION_LEVER_3_SET: &str = "AXIS_CONDITION_LEVER_3_SET";
/// Sets the condition lever position based on the percentage final value of axis the input
/// where: 0% - 33.3% = cutoff 33.3% - 66.6% = low idle 66.6% - 100% = high Parameters: \[0\]:
/// Axis value.
pub const AXIS_CONDITION_LEVER_4_SET: &str = "AXIS_CONDITION_LEVER_4_SET";
/// Decrements the condition lever position by one for the indexed engine. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high Parameters: \[0\]: Engine
/// index.
pub const CONDITION_LEVER_DEC: &str = "CONDITION_LEVER_DEC";
/// Increments the condition lever position by one for the indexed engine. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high Parameters: \[0\]: Engine
/// index.
pub const CONDITION_LEVER_INC: &str = "CONDITION_LEVER_INC";
/// Sets the condition lever for the all engines to the given position, which is one of the
/// following: 0 for cutoff 1 for low idle 2 for high Parameters: \[0\]: Position.
pub const CONDITION_LEVER_SET: &str = "CONDITION_LEVER_SET";
/// Decreases the condition lever position by one for engine 1/2/3/4. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high
pub const CONDITION_LEVER_1_DEC: &str = "CONDITION_LEVER_1_DEC";
/// Decreases the condition lever position by one for engine 1/2/3/4. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high
pub const CONDITION_LEVER_2_DEC: &str = "CONDITION_LEVER_2_DEC";
/// Decreases the condition lever position by one for engine 1/2/3/4. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high
pub const CONDITION_LEVER_3_DEC: &str = "CONDITION_LEVER_3_DEC";
/// Decreases the condition lever position by one for engine 1/2/3/4. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high
pub const CONDITION_LEVER_4_DEC: &str = "CONDITION_LEVER_4_DEC";
/// Increments the condition lever position by one for engine 1/2/3/4. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high
pub const CONDITION_LEVER_1_INC: &str = "CONDITION_LEVER_1_INC";
/// Increments the condition lever position by one for engine 1/2/3/4. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high
pub const CONDITION_LEVER_2_INC: &str = "CONDITION_LEVER_2_INC";
/// Increments the condition lever position by one for engine 1/2/3/4. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high
pub const CONDITION_LEVER_3_INC: &str = "CONDITION_LEVER_3_INC";
/// Increments the condition lever position by one for engine 1/2/3/4. The possible lever
/// positions are as follows: 0 for cutoff 1 for low idle 2 for high
pub const CONDITION_LEVER_4_INC: &str = "CONDITION_LEVER_4_INC";
/// Sets the condition lever for engine 1/2/3/4 to the given position, which is one of the
/// following: 0 for cutoff 1 for low idle 2 for high Parameters: \[0\]: Position.
pub const CONDITION_LEVER_1_SET: &str = "CONDITION_LEVER_1_SET";
/// Sets the condition lever for engine 1/2/3/4 to the given position, which is one of the
/// following: 0 for cutoff 1 for low idle 2 for high Parameters: \[0\]: Position.
pub const CONDITION_LEVER_2_SET: &str = "CONDITION_LEVER_2_SET";
/// Sets the condition lever for engine 1/2/3/4 to the given position, which is one of the
/// following: 0 for cutoff 1 for low idle 2 for high Parameters: \[0\]: Position.
pub const CONDITION_LEVER_3_SET: &str = "CONDITION_LEVER_3_SET";
/// Sets the condition lever for engine 1/2/3/4 to the given position, which is one of the
/// following: 0 for cutoff 1 for low idle 2 for high Parameters: \[0\]: Position.
pub const CONDITION_LEVER_4_SET: &str = "CONDITION_LEVER_4_SET";

// Fuel
/// Control the mixture axis (-100 to 100%). Parameters: \[0\]: Index of the engine (starting
/// at 1) or 0 to target all engines. \[1\]: Value.
pub const AXIS_MIXTURE_SET: &str = "AXIS_MIXTURE_SET";
/// Control the mixture axis (-100 to 100%). Parameters: \[0\]: Value.
pub const AXIS_MIXTURE1_SET: &str = "AXIS_MIXTURE1_SET";
/// Control the mixture axis (-100 to 100%). Parameters: \[0\]: Value.
pub const AXIS_MIXTURE2_SET: &str = "AXIS_MIXTURE2_SET";
/// Control the mixture axis (-100 to 100%). Parameters: \[0\]: Value.
pub const AXIS_MIXTURE3_SET: &str = "AXIS_MIXTURE3_SET";
/// Control the mixture axis (-100 to 100%). Parameters: \[0\]: Value.
pub const AXIS_MIXTURE4_SET: &str = "AXIS_MIXTURE4_SET";
/// Toggle electric fuel pumps Parameters: \[0\]: Index of the engine (starting at 1) or 0 to
/// target all engines..
pub const FUEL_PUMP: &str = "FUEL_PUMP";
/// Decrement mixture levers Parameters: \[0\]: Index of the engine (starting at 1) or 0 to
/// target all engines..
pub const MIXTURE_DECR: &str = "MIXTURE_DECR";
/// Decrement mixture lever 1/2/3/4
pub const MIXTURE1_DECR: &str = "MIXTURE1_DECR";
/// Decrement mixture lever 1/2/3/4
pub const MIXTURE2_DECR: &str = "MIXTURE2_DECR";
/// Decrement mixture lever 1/2/3/4
pub const MIXTURE3_DECR: &str = "MIXTURE3_DECR";
/// Decrement mixture lever 1/2/3/4
pub const MIXTURE4_DECR: &str = "MIXTURE4_DECR";
/// Decrement mixture levers small Parameters: \[0\]: Index of the engine (starting at 1) or 0
/// to target all engines..
pub const MIXTURE_DECR_SMALL: &str = "MIXTURE_DECR_SMALL";
/// Decrement mixture 1/2/3/4 lever small
pub const MIXTURE1_DECR_SMALL: &str = "MIXTURE1_DECR_SMALL";
/// Decrement mixture 1/2/3/4 lever small
pub const MIXTURE2_DECR_SMALL: &str = "MIXTURE2_DECR_SMALL";
/// Decrement mixture 1/2/3/4 lever small
pub const MIXTURE3_DECR_SMALL: &str = "MIXTURE3_DECR_SMALL";
/// Decrement mixture 1/2/3/4 lever small
pub const MIXTURE4_DECR_SMALL: &str = "MIXTURE4_DECR_SMALL";
/// Increment mixture levers Parameters: \[0\]: Index of the engine (starting at 1) or 0 to
/// target all engines..
pub const MIXTURE_INCR: &str = "MIXTURE_INCR";
/// Increment mixture lever 1/2/3/4
pub const MIXTURE1_INCR: &str = "MIXTURE1_INCR";
/// Increment mixture lever 1/2/3/4
pub const MIXTURE2_INCR: &str = "MIXTURE2_INCR";
/// Increment mixture lever 1/2/3/4
pub const MIXTURE3_INCR: &str = "MIXTURE3_INCR";
/// Increment mixture lever 1/2/3/4
pub const MIXTURE4_INCR: &str = "MIXTURE4_INCR";
/// Increment mixture levers small Parameters: \[0\]: Index of the engine (starting at 1) or 0
/// to target all engines..
pub const MIXTURE_INCR_SMALL: &str = "MIXTURE_INCR_SMALL";
/// Increment mixture lever 1/2/3/4 small
pub const MIXTURE1_INCR_SMALL: &str = "MIXTURE1_INCR_SMALL";
/// Increment mixture lever 1/2/3/4 small
pub const MIXTURE2_INCR_SMALL: &str = "MIXTURE2_INCR_SMALL";
/// Increment mixture lever 1/2/3/4 small
pub const MIXTURE3_INCR_SMALL: &str = "MIXTURE3_INCR_SMALL";
/// Increment mixture lever 1/2/3/4 small
pub const MIXTURE4_INCR_SMALL: &str = "MIXTURE4_INCR_SMALL";
/// Set mixture levers to max lean Parameters: \[0\]: Index of the engine (starting at 1) or 0
/// to target all engines..
pub const MIXTURE_LEAN: &str = "MIXTURE_LEAN";
/// Set mixture lever 1/2/3/4 to max lean
pub const MIXTURE1_LEAN: &str = "MIXTURE1_LEAN";
/// Set mixture lever 1/2/3/4 to max lean
pub const MIXTURE2_LEAN: &str = "MIXTURE2_LEAN";
/// Set mixture lever 1/2/3/4 to max lean
pub const MIXTURE3_LEAN: &str = "MIXTURE3_LEAN";
/// Set mixture lever 1/2/3/4 to max lean
pub const MIXTURE4_LEAN: &str = "MIXTURE4_LEAN";
/// Increment or decrement the fuel mixture based on the speed and the distance of the
/// interaction from the user device (values are clamped between 0 to 16384). NOTE: This is
/// primarily for the PS5 controller touch-pad, but should work with any touch-input.
#[cfg(feature = "sunrise")]
pub const MIXTURE_RELATIVE_AXIS: &str = "MIXTURE_RELATIVE_AXIS";
/// Set mixture levers to max rich Parameters: \[0\]: Index of the engine (starting at 1) or 0
/// to target all engines..
pub const MIXTURE_RICH: &str = "MIXTURE_RICH";
/// Set mixture lever 1/2/3/4 to max rich
pub const MIXTURE1_RICH: &str = "MIXTURE1_RICH";
/// Set mixture lever 1/2/3/4 to max rich
pub const MIXTURE2_RICH: &str = "MIXTURE2_RICH";
/// Set mixture lever 1/2/3/4 to max rich
pub const MIXTURE3_RICH: &str = "MIXTURE3_RICH";
/// Set mixture lever 1/2/3/4 to max rich
pub const MIXTURE4_RICH: &str = "MIXTURE4_RICH";
/// Engine mixture set. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target
/// all engines. \[1\]: Value.
pub const MIXTURE_SET: &str = "MIXTURE_SET";
/// Set mixture levers to current best power setting. Parameters: \[0\]: Index of the engine
/// (starting at 1) or 0 to target all engines..
pub const MIXTURE_SET_BEST: &str = "MIXTURE_SET_BEST";
/// Set engine 1/2/3/4 mixture. Parameters: \[0\]: Value.
pub const MIXTURE1_SET: &str = "MIXTURE1_SET";
/// Set engine 1/2/3/4 mixture. Parameters: \[0\]: Value.
pub const MIXTURE2_SET: &str = "MIXTURE2_SET";
/// Set engine 1/2/3/4 mixture. Parameters: \[0\]: Value.
pub const MIXTURE3_SET: &str = "MIXTURE3_SET";
/// Set engine 1/2/3/4 mixture. Parameters: \[0\]: Value.
pub const MIXTURE4_SET: &str = "MIXTURE4_SET";
/// Set engine 1/2/3/4 fuel valve. Parameters: \[0\]: Value.
pub const SET_FUEL_VALVE_ENG1: &str = "SET_FUEL_VALVE_ENG1";
/// Set engine 1/2/3/4 fuel valve. Parameters: \[0\]: Value.
pub const SET_FUEL_VALVE_ENG2: &str = "SET_FUEL_VALVE_ENG2";
/// Set engine 1/2/3/4 fuel valve. Parameters: \[0\]: Value.
pub const SET_FUEL_VALVE_ENG3: &str = "SET_FUEL_VALVE_ENG3";
/// Set engine 1/2/3/4 fuel valve. Parameters: \[0\]: Value.
pub const SET_FUEL_VALVE_ENG4: &str = "SET_FUEL_VALVE_ENG4";
/// Toggle the status of the fuel shutoff valve (used on piston engine to enable/disable fuel
/// arrival). Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all
/// engines..
pub const SHUTOFF_VALVE_TOGGLE: &str = "SHUTOFF_VALVE_TOGGLE";
/// Turns on the fuel shutoff valve (used on piston engines to enable fuel arrival).
/// Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all engines..
pub const SHUTOFF_VALVE_ON: &str = "SHUTOFF_VALVE_ON";
/// Turns off the fuel shutoff valve (used on piston engines to disable fuel arrival).
/// Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all engines..
pub const SHUTOFF_VALVE_OFF: &str = "SHUTOFF_VALVE_OFF";
/// Toggle electric fuel pumps Parameters: \[0\]: Index of the engine (starting at 1) or 0 to
/// target all engines..
pub const TOGGLE_ELECT_FUEL_PUMP: &str = "TOGGLE_ELECT_FUEL_PUMP";
/// Toggle engine 1/2/3/4 electric fuel pump
pub const TOGGLE_ELECT_FUEL_PUMP1: &str = "TOGGLE_ELECT_FUEL_PUMP1";
/// Toggle engine 1/2/3/4 electric fuel pump
pub const TOGGLE_ELECT_FUEL_PUMP2: &str = "TOGGLE_ELECT_FUEL_PUMP2";
/// Toggle engine 1/2/3/4 electric fuel pump
pub const TOGGLE_ELECT_FUEL_PUMP3: &str = "TOGGLE_ELECT_FUEL_PUMP3";
/// Toggle engine 1/2/3/4 electric fuel pump
pub const TOGGLE_ELECT_FUEL_PUMP4: &str = "TOGGLE_ELECT_FUEL_PUMP4";
/// Toggle engine fuel valves
pub const TOGGLE_FUEL_VALVE_ALL: &str = "TOGGLE_FUEL_VALVE_ALL";
/// Toggle engine 1/2/3/4 fuel valve
pub const TOGGLE_FUEL_VALVE_ENG1: &str = "TOGGLE_FUEL_VALVE_ENG1";
/// Toggle engine 1/2/3/4 fuel valve
pub const TOGGLE_FUEL_VALVE_ENG2: &str = "TOGGLE_FUEL_VALVE_ENG2";
/// Toggle engine 1/2/3/4 fuel valve
pub const TOGGLE_FUEL_VALVE_ENG3: &str = "TOGGLE_FUEL_VALVE_ENG3";
/// Toggle engine 1/2/3/4 fuel valve
pub const TOGGLE_FUEL_VALVE_ENG4: &str = "TOGGLE_FUEL_VALVE_ENG4";

// Magneto
/// Selects magnetos (for +/- sequence) Parameters: \[0\]: Magneto index.
pub const MAGNETO: &str = "MAGNETO";
/// Set indexed engine magnetos on Parameters: \[0\]: Magneto index.
pub const MAGNETO_BOTH: &str = "MAGNETO_BOTH";
/// Set engine 1/2/3/4 magnetos on
pub const MAGNETO1_BOTH: &str = "MAGNETO1_BOTH";
/// Set engine 1/2/3/4 magnetos on
pub const MAGNETO2_BOTH: &str = "MAGNETO2_BOTH";
/// Set engine 1/2/3/4 magnetos on
pub const MAGNETO3_BOTH: &str = "MAGNETO3_BOTH";
/// Set engine 1/2/3/4 magnetos on
pub const MAGNETO4_BOTH: &str = "MAGNETO4_BOTH";
/// Decrease all magneto switches positions
pub const MAGNETO_DECR: &str = "MAGNETO_DECR";
/// Decrease engine 1/2/3/4 magneto switch position
pub const MAGNETO1_DECR: &str = "MAGNETO1_DECR";
/// Decrease engine 1/2/3/4 magneto switch position
pub const MAGNETO2_DECR: &str = "MAGNETO2_DECR";
/// Decrease engine 1/2/3/4 magneto switch position
pub const MAGNETO3_DECR: &str = "MAGNETO3_DECR";
/// Decrease engine 1/2/3/4 magneto switch position
pub const MAGNETO4_DECR: &str = "MAGNETO4_DECR";
/// Increase all magneto switches positions
pub const MAGNETO_INCR: &str = "MAGNETO_INCR";
/// Increase engine 1/2/3/4 magneto switch position
pub const MAGNETO1_INCR: &str = "MAGNETO1_INCR";
/// Increase engine 1/2/3/4 magneto switch position
pub const MAGNETO2_INCR: &str = "MAGNETO2_INCR";
/// Increase engine 1/2/3/4 magneto switch position
pub const MAGNETO3_INCR: &str = "MAGNETO3_INCR";
/// Increase engine 1/2/3/4 magneto switch position
pub const MAGNETO4_INCR: &str = "MAGNETO4_INCR";
/// Toggle all engine left magnetos
pub const MAGNETO_LEFT: &str = "MAGNETO_LEFT";
/// Toggle engine 1/2/3/4 left magneto
pub const MAGNETO1_LEFT: &str = "MAGNETO1_LEFT";
/// Toggle engine 1/2/3/4 left magneto
pub const MAGNETO2_LEFT: &str = "MAGNETO2_LEFT";
/// Toggle engine 1/2/3/4 left magneto
pub const MAGNETO3_LEFT: &str = "MAGNETO3_LEFT";
/// Toggle engine 1/2/3/4 left magneto
pub const MAGNETO4_LEFT: &str = "MAGNETO4_LEFT";
/// Set all engine magnetos off
pub const MAGNETO_OFF: &str = "MAGNETO_OFF";
/// Set engine 1/2/3/4 magnetos off
pub const MAGNETO1_OFF: &str = "MAGNETO1_OFF";
/// Set engine 1/2/3/4 magnetos off
pub const MAGNETO2_OFF: &str = "MAGNETO2_OFF";
/// Set engine 1/2/3/4 magnetos off
pub const MAGNETO3_OFF: &str = "MAGNETO3_OFF";
/// Set engine 1/2/3/4 magnetos off
pub const MAGNETO4_OFF: &str = "MAGNETO4_OFF";
/// Toggle all engine right magnetos
pub const MAGNETO_RIGHT: &str = "MAGNETO_RIGHT";
/// Toggle engine 1/2/3/4 right magneto
pub const MAGNETO1_RIGHT: &str = "MAGNETO1_RIGHT";
/// Toggle engine 1/2/3/4 right magneto
pub const MAGNETO2_RIGHT: &str = "MAGNETO2_RIGHT";
/// Toggle engine 1/2/3/4 right magneto
pub const MAGNETO3_RIGHT: &str = "MAGNETO3_RIGHT";
/// Toggle engine 1/2/3/4 right magneto
pub const MAGNETO4_RIGHT: &str = "MAGNETO4_RIGHT";
/// Set the status of the current controlled engine starters to On/Off. Controlled engines are
/// set through the SimVar ENGINE CONTROL SELECT. Parameters: \[0\]: Bool.
pub const MAGNETO_SET: &str = "MAGNETO_SET";
/// Set all engine magnetos on and toggle starters
pub const MAGNETO_START: &str = "MAGNETO_START";
/// Set engine 1/2/3/4 magnetos on and toggle starter
pub const MAGNETO1_START: &str = "MAGNETO1_START";
/// Set engine 1/2/3/4 magnetos on and toggle starter
pub const MAGNETO2_START: &str = "MAGNETO2_START";
/// Set engine 1/2/3/4 magnetos on and toggle starter
pub const MAGNETO3_START: &str = "MAGNETO3_START";
/// Set engine 1/2/3/4 magnetos on and toggle starter
pub const MAGNETO4_START: &str = "MAGNETO4_START";

// General Engine
/// This can be used to set the horizontal (pitch) thrust vector for the indexed engine (or
/// all engines). The value given should be between 0 and 16383, where 0 corresponds to the
/// value given for the TiltMinAngles.N paramater and 16383 corresponds to the TiltMaxAngles.N
/// parameter, with the final value being a mapping between the two. Any value above or below
/// the min or max values will be clamped to within the appropriate range. Parameters: \[0\]:
/// Index of the engine (starting at 1) or 0 to target all engines. \[1\]: Value to set
/// between 0-16K.
#[cfg(feature = "sunrise")]
pub const AXIS_THRUST_VECTOR_HORIZONTAL_SET: &str = "AXIS_THRUST_VECTOR_HORIZONTAL_SET";
/// This can be used to set the vertical (yaw) thrust vector for the indexed engine (or all
/// engines). The value given should be between 0 and 16383, where 0 corresponds to the value
/// given for the TiltMinAngles.N paramater and 16383 corresponds to the TiltMaxAngles.N
/// parameter, with the final value being a mapping between the two. Any value above or below
/// the min or max values will be clamped to within the appropriate range. Parameters: \[0\]:
/// Index of the engine (starting at 1) or 0 to target all engines. \[1\]: Value to set
/// between 0-16K.
#[cfg(feature = "sunrise")]
pub const AXIS_THRUST_VECTOR_VERTICAL_SET: &str = "AXIS_THRUST_VECTOR_VERTICAL_SET";
/// Sets engines for 1,2,3,4 selection (to be followed by SELECT_n)
pub const ENGINE: &str = "ENGINE";
/// Triggers auto-start
pub const ENGINE_AUTO_START: &str = "ENGINE_AUTO_START";
/// Triggers auto-shutdown
pub const ENGINE_AUTO_SHUTDOWN: &str = "ENGINE_AUTO_SHUTDOWN";
/// This is used to set the given engine bleed air to active (1) or not (0). Parameters:
/// \[0\]: Engine Index \[1\]: Bool.
pub const ENGINE_BLEED_AIR_SOURCE_SET: &str = "ENGINE_BLEED_AIR_SOURCE_SET";
/// Toggles the indexed engine bleed air between active (1) and inactive (0). Note that if you
/// supply 0 instead of a single engine index, then the event will target all engines.
/// Parameters: \[0\]: Engine Index.
pub const ENGINE_BLEED_AIR_SOURCE_TOGGLE: &str = "ENGINE_BLEED_AIR_SOURCE_TOGGLE";
/// Set the indexed engine as the master engine (1), or not (0). Parameters: \[0\]: Engine
/// Index.
pub const ENGINE_MASTER_SET: &str = "ENGINE_MASTER_SET";
/// Set the numbered engine as the master engine (1), or not (0).
pub const ENGINE_MASTER_1_SET: &str = "ENGINE_MASTER_1_SET";
/// Set the numbered engine as the master engine (1), or not (0).
pub const ENGINE_MASTER_2_SET: &str = "ENGINE_MASTER_2_SET";
/// Set the numbered engine as the master engine (1), or not (0).
pub const ENGINE_MASTER_3_SET: &str = "ENGINE_MASTER_3_SET";
/// Set the numbered engine as the master engine (1), or not (0).
pub const ENGINE_MASTER_4_SET: &str = "ENGINE_MASTER_4_SET";
/// Toogle the indexed engine between being the master engine (1), or not (0). Parameters:
/// \[0\]: Engine Index.
pub const ENGINE_MASTER_TOGGLE: &str = "ENGINE_MASTER_TOGGLE";
/// Toogle the numbered engine between being the master engine (1), or not (0).
pub const ENGINE_MASTER_1_TOGGLE: &str = "ENGINE_MASTER_1_TOGGLE";
/// Toogle the numbered engine between being the master engine (1), or not (0).
pub const ENGINE_MASTER_2_TOGGLE: &str = "ENGINE_MASTER_2_TOGGLE";
/// Toogle the numbered engine between being the master engine (1), or not (0).
pub const ENGINE_MASTER_3_TOGGLE: &str = "ENGINE_MASTER_3_TOGGLE";
/// Toogle the numbered engine between being the master engine (1), or not (0).
pub const ENGINE_MASTER_4_TOGGLE: &str = "ENGINE_MASTER_4_TOGGLE";
/// Set the engine mode on CRANK.
pub const ENGINE_MODE_CRANK_SET: &str = "ENGINE_MODE_CRANK_SET";
/// Set the engine mode on NORM.
pub const ENGINE_MODE_NORM_SET: &str = "ENGINE_MODE_NORM_SET";
/// Set the engine mode on IGN/ START.
pub const ENGINE_MODE_IGN_START: &str = "ENGINE_MODE_IGN_START";
/// Trigger engine primers
pub const ENGINE_PRIMER: &str = "ENGINE_PRIMER";
/// This can be used to decrease the indexed engine (or all engines) horizontal (pitch) thrust
/// vector by 1% of the total range as defined by the TiltMinAngles.N and TiltMaxAngles.N
/// parameters. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all
/// engines..
#[cfg(feature = "sunrise")]
pub const THRUST_VECTOR_HORIZONTAL_DECREASE: &str = "THRUST_VECTOR_HORIZONTAL_DECREASE";
/// This can be used to increase the indexed engine (or all engines) horizontal (pitch) thrust
/// vector by 1% of the total range as defined by the TiltMinAngles.N and TiltMaxAngles.N
/// parameters. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all
/// engines..
#[cfg(feature = "sunrise")]
pub const THRUST_VECTOR_HORIZONTAL_INCREASE: &str = "THRUST_VECTOR_HORIZONTAL_INCREASE";
/// This can be used to decrease the indexed engine (or all engines) vertical (yaw) thrust
/// vector by 1% of the total range as defined by the TiltMinAngles.N and TiltMaxAngles.N
/// parameters. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all
/// engines..
#[cfg(feature = "sunrise")]
pub const THRUST_VECTOR_VERTICAL_DECREASE: &str = "THRUST_VECTOR_VERTICAL_DECREASE";
/// This can be used to increase the indexed engine (or all engines) vertical (yaw) thrust
/// vector by 1% of the total range as defined by the TiltMinAngles.N and TiltMaxAngles.N
/// parameters. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all
/// engines..
#[cfg(feature = "sunrise")]
pub const THRUST_VECTOR_VERTICAL_INCREASE: &str = "THRUST_VECTOR_VERTICAL_INCREASE";

// Miscellaneous
/// Sets engine 1/2/3/4 cowl flap lever position (0 to 16383) Parameters: \[0\]: position from
/// 0 to 16983.
pub const COWLFLAP1_SET: &str = "COWLFLAP1_SET";
/// Sets engine 1/2/3/4 cowl flap lever position (0 to 16383) Parameters: \[0\]: position from
/// 0 to 16983.
pub const COWLFLAP2_SET: &str = "COWLFLAP2_SET";
/// Sets engine 1/2/3/4 cowl flap lever position (0 to 16383) Parameters: \[0\]: position from
/// 0 to 16983.
pub const COWLFLAP3_SET: &str = "COWLFLAP3_SET";
/// Sets engine 1/2/3/4 cowl flap lever position (0 to 16383) Parameters: \[0\]: position from
/// 0 to 16983.
pub const COWLFLAP4_SET: &str = "COWLFLAP4_SET";
/// Decrement all cowl flap levers by 0.078.
pub const DEC_COWL_FLAPS: &str = "DEC_COWL_FLAPS";
/// Decrement engine 1/2/3/4 cowl flap lever by 0.078.
pub const DEC_COWL_FLAPS1: &str = "DEC_COWL_FLAPS1";
/// Decrement engine 1/2/3/4 cowl flap lever by 0.078.
pub const DEC_COWL_FLAPS2: &str = "DEC_COWL_FLAPS2";
/// Decrement engine 1/2/3/4 cowl flap lever by 0.078.
pub const DEC_COWL_FLAPS3: &str = "DEC_COWL_FLAPS3";
/// Decrement engine 1/2/3/4 cowl flap lever by 0.078.
pub const DEC_COWL_FLAPS4: &str = "DEC_COWL_FLAPS4";
/// Increment cowl flap levers by 0.078.
pub const INC_COWL_FLAPS: &str = "INC_COWL_FLAPS";
/// Increment engine 1/2/3/4 cowl flap lever
pub const INC_COWL_FLAPS1: &str = "INC_COWL_FLAPS1";
/// Increment engine 1/2/3/4 cowl flap lever
pub const INC_COWL_FLAPS2: &str = "INC_COWL_FLAPS2";
/// Increment engine 1/2/3/4 cowl flap lever
pub const INC_COWL_FLAPS3: &str = "INC_COWL_FLAPS3";
/// Increment engine 1/2/3/4 cowl flap lever
pub const INC_COWL_FLAPS4: &str = "INC_COWL_FLAPS4";
/// Set the oil cooling flaps to the down position (1).
pub const OIL_COOLING_FLAPS_DOWN: &str = "OIL_COOLING_FLAPS_DOWN";
/// Set the oil cooling flaps to either the up position (0) or the down position (1).
/// Parameters: \[0\]: position.
pub const OIL_COOLING_FLAPS_SET: &str = "OIL_COOLING_FLAPS_SET";
/// Toggle the oil cooling flaps between the up position (0) and the down position (1).
pub const OIL_COOLING_FLAPS_TOGGLE: &str = "OIL_COOLING_FLAPS_TOGGLE";
/// Set the oil cooling flaps to the up position (0).
pub const OIL_COOLING_FLAPS_UP: &str = "OIL_COOLING_FLAPS_UP";
/// Disables the plasma effect for Engine 1.
#[cfg(feature = "sunrise")]
pub const PLASMA_OFF: &str = "PLASMA_OFF";
/// Enables the plasma effect for Engine 1.
#[cfg(feature = "sunrise")]
pub const PLASMA_ON: &str = "PLASMA_ON";
/// Toggles the plasma effect on / off for Engine 1.
#[cfg(feature = "sunrise")]
pub const PLASMA_TOGGLE: &str = "PLASMA_TOGGLE";
/// Sets the plasma effect for Engine 1 to either on (1) or off (0). Parameters: \[0\]:
/// boolean.
#[cfg(feature = "sunrise")]
pub const PLASMA_SET: &str = "PLASMA_SET";
/// Set the radiator cooling flaps to the down position (1).
pub const RADIATOR_COOLING_FLAPS_DOWN: &str = "RADIATOR_COOLING_FLAPS_DOWN";
/// Set the radiator cooling flaps to either the up position (0) or the down position (1).
pub const RADIATOR_COOLING_FLAPS_SET: &str = "RADIATOR_COOLING_FLAPS_SET";
/// Toggle the radiator cooling flaps between the up position (0) and the down position (1).
pub const RADIATOR_COOLING_FLAPS_TOGGLE: &str = "RADIATOR_COOLING_FLAPS_TOGGLE";
/// Set the radiator cooling flaps to the up position (0).
pub const RADIATOR_COOLING_FLAPS_UP: &str = "RADIATOR_COOLING_FLAPS_UP";
/// Toggles master ignition switch
pub const TOGGLE_MASTER_IGNITION_SWITCH: &str = "TOGGLE_MASTER_IGNITION_SWITCH";
/// Toogle the engine primers on (1) or off (0) Parameters: \[0\]: Index of the engine
/// (starting at 1) or 0 to target all engines..
pub const TOGGLE_PRIMER: &str = "TOGGLE_PRIMER";
/// Trigger the engine 1/2/3/4 primer on (1) or off (0).
pub const TOGGLE_PRIMER1: &str = "TOGGLE_PRIMER1";
/// Trigger the engine 1/2/3/4 primer on (1) or off (0).
pub const TOGGLE_PRIMER2: &str = "TOGGLE_PRIMER2";
/// Trigger the engine 1/2/3/4 primer on (1) or off (0).
pub const TOGGLE_PRIMER3: &str = "TOGGLE_PRIMER3";
/// Trigger the engine 1/2/3/4 primer on (1) or off (0).
pub const TOGGLE_PRIMER4: &str = "TOGGLE_PRIMER4";
/// Toggles afterburners on (1) or off (0). Parameters: \[0\]: Index of the engine (starting
/// at 1) or 0 to target all engines..
pub const TOGGLE_AFTERBURNER: &str = "TOGGLE_AFTERBURNER";
/// Toggles engine 1/2/3/4 afterburner on (1) or off (0).
pub const TOGGLE_AFTERBURNER1: &str = "TOGGLE_AFTERBURNER1";
/// Toggles engine 1/2/3/4 afterburner on (1) or off (0).
pub const TOGGLE_AFTERBURNER2: &str = "TOGGLE_AFTERBURNER2";
/// Toggles engine 1/2/3/4 afterburner on (1) or off (0).
pub const TOGGLE_AFTERBURNER3: &str = "TOGGLE_AFTERBURNER3";
/// Toggles engine 1/2/3/4 afterburner on (1) or off (0).
pub const TOGGLE_AFTERBURNER4: &str = "TOGGLE_AFTERBURNER4";

// Propeller
/// Set propeller pitch for indexed engine to the exact value (-16383 to +16383). Parameters:
/// \[0\]: Index of the engine (starting at 1) or 0 to target all engines. \[1\]: pitch from 0
/// (-100%) to 16983 (+100%).
pub const AXIS_PROPELLER_SET: &str = "AXIS_PROPELLER_SET";
/// Set propeller pitch for engine 1/2/3/4 exact value (-16383 to +16383) Parameters: \[0\]:
/// pitch from 0 (-100%) to 16983 (+100%).
pub const AXIS_PROPELLER1_SET: &str = "AXIS_PROPELLER1_SET";
/// Set propeller pitch for engine 1/2/3/4 exact value (-16383 to +16383) Parameters: \[0\]:
/// pitch from 0 (-100%) to 16983 (+100%).
pub const AXIS_PROPELLER2_SET: &str = "AXIS_PROPELLER2_SET";
/// Set propeller pitch for engine 1/2/3/4 exact value (-16383 to +16383) Parameters: \[0\]:
/// pitch from 0 (-100%) to 16983 (+100%).
pub const AXIS_PROPELLER3_SET: &str = "AXIS_PROPELLER3_SET";
/// Set propeller pitch for engine 1/2/3/4 exact value (-16383 to +16383) Parameters: \[0\]:
/// pitch from 0 (-100%) to 16983 (+100%).
pub const AXIS_PROPELLER4_SET: &str = "AXIS_PROPELLER4_SET";
/// This key allows you to disable the propeller Force Beta mode, in which case the internal
/// coded simulation logic to drive the beta is used instead of the value from PROP BETA
/// FORCED POSITION. Parameters: \[0\]: The engine index to target (from 1 to 16, or 0 for all
/// engines).
pub const PROP_FORCE_BETA_OFF: &str = "PROP_FORCE_BETA_OFF";
/// This keys allows you to enable the propeller Force Beta mode, in which case the sim logic
/// to drive the beta is ignored and instead the value from PROP BETA FORCED POSITION is used.
/// Parameters: \[0\]: The engine index to target (from 1 to 16, or 0 for all engines).
pub const PROP_FORCE_BETA_ON: &str = "PROP_FORCE_BETA_ON";
/// This key allows you to set the propeller to be in Force Beta mode, in which case the
/// internal coded simulation logic that normally drives the beta is ignored and instead the
/// value from PROP BETA FORCED POSITION is used. Parameters: \[0\]: The engine index to
/// target (from 1 to 16, or 0 for all engines) \[1\]: Whether or not to force the prop beta
/// (Boolean)..
pub const PROP_FORCE_BETA_SET: &str = "PROP_FORCE_BETA_SET";
/// This key allows you to toggle between the normal and Force Beta mode. If enabled, the
/// Force Beta mode will prevent the internal coded simulation logic from driving the beta and
/// instead allow you to control it with the value from PROP BETA FORCED POSITION. Parameters:
/// \[0\]: The engine index to target (from 1 to 16, or 0 for all engines).
pub const PROP_FORCE_BETA_TOGGLE: &str = "PROP_FORCE_BETA_TOGGLE";
/// This key allows you to set the value that the prop will attempt to reach when in Forced
/// Beta mode (this will have the same effect as setting the PROP BETA FORCED POSITION
/// SimVar). Parameters: \[0\]: The engine index to target (from 1 to 16, or 0 for all
/// engines) \[1\]: The angle that the prop should be forced to. This is stored as the 16k
/// representation of an angle between -180 degrees and + 180 degrees.
pub const PROP_FORCE_BETA_VALUE_SET: &str = "PROP_FORCE_BETA_VALUE_SET";
/// Unlocks the propeller lock. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to
/// target all engines..
pub const PROP_LOCK_OFF: &str = "PROP_LOCK_OFF";
/// Locks the propeller, but only if the propeller is already running below 5% RPM.
/// Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all engines..
pub const PROP_LOCK_ON: &str = "PROP_LOCK_ON";
/// Sets the propeller to be either locked (1) or unlocked (0). Note that you can only lock
/// the propeller if it is already running below 5% RPM. Parameters: \[0\]: Index of the
/// engine (starting at 1) or 0 to target all engines. \[0\]: True/False (1, 0).
pub const PROP_LOCK_SET: &str = "PROP_LOCK_SET";
/// Toggles the propeller pitch lever between locked (1) and unlocked (0). Note that you can
/// only lock the propeller if it is already running below 5% RPM. Parameters: \[0\]: Index of
/// the engine (starting at 1) or 0 to target all engines..
pub const PROP_LOCK_TOGGLE: &str = "PROP_LOCK_TOGGLE";
/// Set propeller pitch for indexed engine, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the lever position will be sent back toward minimum
/// position of -100%. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target
/// all engines. \[1\]: pitch from 0 (-100%) to 16983 (+100%).
pub const PROP_PITCH_AXIS_SET_EX1: &str = "PROP_PITCH_AXIS_SET_EX1";
/// Set propeller pitch for engine 1/2/3/4, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the lever position will be sent back toward minimum
/// position of -100%. Parameters: \[0\]: pitch from 0 (-100%) to 16983 (+100%).
pub const PROP_PITCH1_AXIS_SET_EX1: &str = "PROP_PITCH1_AXIS_SET_EX1";
/// Set propeller pitch for engine 1/2/3/4, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the lever position will be sent back toward minimum
/// position of -100%. Parameters: \[0\]: pitch from 0 (-100%) to 16983 (+100%).
pub const PROP_PITCH2_AXIS_SET_EX1: &str = "PROP_PITCH2_AXIS_SET_EX1";
/// Set propeller pitch for engine 1/2/3/4, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the lever position will be sent back toward minimum
/// position of -100%. Parameters: \[0\]: pitch from 0 (-100%) to 16983 (+100%).
pub const PROP_PITCH3_AXIS_SET_EX1: &str = "PROP_PITCH3_AXIS_SET_EX1";
/// Set propeller pitch for engine 1/2/3/4, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the lever position will be sent back toward minimum
/// position of -100%. Parameters: \[0\]: pitch from 0 (-100%) to 16983 (+100%).
pub const PROP_PITCH4_AXIS_SET_EX1: &str = "PROP_PITCH4_AXIS_SET_EX1";
/// Decrement prop pitch levers Parameters: \[0\]: The engine index to target (from 1 to 16,
/// or 0 for all engines).
pub const PROP_PITCH_DECR: &str = "PROP_PITCH_DECR";
/// Decrement prop pitch lever 1/2/3/4
pub const PROP_PITCH1_DECR: &str = "PROP_PITCH1_DECR";
/// Decrement prop pitch lever 1/2/3/4
pub const PROP_PITCH2_DECR: &str = "PROP_PITCH2_DECR";
/// Decrement prop pitch lever 1/2/3/4
pub const PROP_PITCH3_DECR: &str = "PROP_PITCH3_DECR";
/// Decrement prop pitch lever 1/2/3/4
pub const PROP_PITCH4_DECR: &str = "PROP_PITCH4_DECR";
/// Decrease prop levers small
pub const PROP_PITCH_DECR_SMALL: &str = "PROP_PITCH_DECR_SMALL";
/// Decrease prop lever 1/2/3/4 small
pub const PROP_PITCH1_DECR_SMALL: &str = "PROP_PITCH1_DECR_SMALL";
/// Decrease prop lever 1/2/3/4 small
pub const PROP_PITCH2_DECR_SMALL: &str = "PROP_PITCH2_DECR_SMALL";
/// Decrease prop lever 1/2/3/4 small
pub const PROP_PITCH3_DECR_SMALL: &str = "PROP_PITCH3_DECR_SMALL";
/// Decrease prop lever 1/2/3/4 small
pub const PROP_PITCH4_DECR_SMALL: &str = "PROP_PITCH4_DECR_SMALL";
/// Decrease the propeller pitch for the indexed engine, taking into account the reverser
/// status, ie: When in reverse a decrease event will actually increase the lever position
/// bringing it toward the minimum reverse position. Parameters: \[0\]: The engine index to
/// target (from 1 to 16, or 0 for all engines).
pub const PROP_PITCH_DECREASE_EX1: &str = "PROP_PITCH_DECREASE_EX1";
/// Decrease the propeller pitch for engine 1/2/3/4, taking into account the reverser status,
/// ie: When in reverse a decrease event will actually increase the lever position bringing it
/// toward the minimum reverse position.
pub const PROP_PITCH1_DECREASE_EX1: &str = "PROP_PITCH1_DECREASE_EX1";
/// Decrease the propeller pitch for engine 1/2/3/4, taking into account the reverser status,
/// ie: When in reverse a decrease event will actually increase the lever position bringing it
/// toward the minimum reverse position.
pub const PROP_PITCH2_DECREASE_EX1: &str = "PROP_PITCH2_DECREASE_EX1";
/// Decrease the propeller pitch for engine 1/2/3/4, taking into account the reverser status,
/// ie: When in reverse a decrease event will actually increase the lever position bringing it
/// toward the minimum reverse position.
pub const PROP_PITCH3_DECREASE_EX1: &str = "PROP_PITCH3_DECREASE_EX1";
/// Decrease the propeller pitch for engine 1/2/3/4, taking into account the reverser status,
/// ie: When in reverse a decrease event will actually increase the lever position bringing it
/// toward the minimum reverse position.
pub const PROP_PITCH4_DECREASE_EX1: &str = "PROP_PITCH4_DECREASE_EX1";
/// Decrease the propeller pitch for the indexed engine by half the amount of the normal
/// decrease event, taking into account the reverser status, ie: When in reverse a decrease
/// event will actually increase the lever position bringing it toward the minimum reverse
/// position. Parameters: \[0\]: The engine index to target (from 1 to 16, or 0 for all
/// engines).
pub const PROP_PITCH_DECREASE_SMALL_EX1: &str = "PROP_PITCH_DECREASE_SMALL_EX1";
/// Decrease the propeller pitch for engine 1/2/3/4 by half the amount of the normal decrease
/// event, taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the lever position bringing it toward the minimum reverse position.
pub const PROP_PITCH1_DECREASE_SMALL_EX1: &str = "PROP_PITCH1_DECREASE_SMALL_EX1";
/// Decrease the propeller pitch for engine 1/2/3/4 by half the amount of the normal decrease
/// event, taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the lever position bringing it toward the minimum reverse position.
pub const PROP_PITCH2_DECREASE_SMALL_EX1: &str = "PROP_PITCH2_DECREASE_SMALL_EX1";
/// Decrease the propeller pitch for engine 1/2/3/4 by half the amount of the normal decrease
/// event, taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the lever position bringing it toward the minimum reverse position.
pub const PROP_PITCH3_DECREASE_SMALL_EX1: &str = "PROP_PITCH3_DECREASE_SMALL_EX1";
/// Decrease the propeller pitch for engine 1/2/3/4 by half the amount of the normal decrease
/// event, taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the lever position bringing it toward the minimum reverse position.
pub const PROP_PITCH4_DECREASE_SMALL_EX1: &str = "PROP_PITCH4_DECREASE_SMALL_EX1";
/// Set prop pitch levers min (hi pitch) Parameters: \[0\]: The engine index to target (from 1
/// to 16, or 0 for all engines).
pub const PROP_PITCH_HI: &str = "PROP_PITCH_HI";
/// Set prop pitch lever 1/2/3/4 min (hi pitch)
pub const PROP_PITCH1_HI: &str = "PROP_PITCH1_HI";
/// Set prop pitch lever 1/2/3/4 min (hi pitch)
pub const PROP_PITCH2_HI: &str = "PROP_PITCH2_HI";
/// Set prop pitch lever 1/2/3/4 min (hi pitch)
pub const PROP_PITCH3_HI: &str = "PROP_PITCH3_HI";
/// Set prop pitch lever 1/2/3/4 min (hi pitch)
pub const PROP_PITCH4_HI: &str = "PROP_PITCH4_HI";
/// Set prop pitch levers min (hi pitch) Parameters: \[0\]: The engine index to target (from 1
/// to 16, or 0 for all engines).
pub const PROP_PITCH_HI_EX1: &str = "PROP_PITCH_HI_EX1";
/// Set prop pitch lever 1/2/3/4 min (hi pitch)
pub const PROP_PITCH1_HI_EX1: &str = "PROP_PITCH1_HI_EX1";
/// Set prop pitch lever 1/2/3/4 min (hi pitch)
pub const PROP_PITCH2_HI_EX1: &str = "PROP_PITCH2_HI_EX1";
/// Set prop pitch lever 1/2/3/4 min (hi pitch)
pub const PROP_PITCH3_HI_EX1: &str = "PROP_PITCH3_HI_EX1";
/// Set prop pitch lever 1/2/3/4 min (hi pitch)
pub const PROP_PITCH4_HI_EX1: &str = "PROP_PITCH4_HI_EX1";
/// Increment prop pitch levers Parameters: \[0\]: The engine index to target (from 1 to 16,
/// or 0 for all engines).
pub const PROP_PITCH_INCR: &str = "PROP_PITCH_INCR";
/// Increment prop pitch lever 1/2/3/4
pub const PROP_PITCH1_INCR: &str = "PROP_PITCH1_INCR";
/// Increment prop pitch lever 1/2/3/4
pub const PROP_PITCH2_INCR: &str = "PROP_PITCH2_INCR";
/// Increment prop pitch lever 1/2/3/4
pub const PROP_PITCH3_INCR: &str = "PROP_PITCH3_INCR";
/// Increment prop pitch lever 1/2/3/4
pub const PROP_PITCH4_INCR: &str = "PROP_PITCH4_INCR";
/// Increment prop pitch levers small Parameters: \[0\]: The engine index to target (from 1 to
/// 16, or 0 for all engines).
pub const PROP_PITCH_INCR_SMALL: &str = "PROP_PITCH_INCR_SMALL";
/// Increment prop pitch lever 1/2/3/4 small
pub const PROP_PITCH1_INCR_SMALL: &str = "PROP_PITCH1_INCR_SMALL";
/// Increment prop pitch lever 1/2/3/4 small
pub const PROP_PITCH2_INCR_SMALL: &str = "PROP_PITCH2_INCR_SMALL";
/// Increment prop pitch lever 1/2/3/4 small
pub const PROP_PITCH3_INCR_SMALL: &str = "PROP_PITCH3_INCR_SMALL";
/// Increment prop pitch lever 1/2/3/4 small
pub const PROP_PITCH4_INCR_SMALL: &str = "PROP_PITCH4_INCR_SMALL";
/// Increase the propeller pitch for the indexed engine, taking into account the reverser
/// status, ie: When in reverse an increase event will actually decrease the lever position
/// bringing it toward the maximum reverse position. Parameters: \[0\]: The engine index to
/// target (from 1 to 16, or 0 for all engines).
pub const PROP_PITCH_INCREASE_EX1: &str = "PROP_PITCH_INCREASE_EX1";
/// Increase the propeller pitch for engine 1/2/3/4, taking into account the reverser status,
/// ie: When in reverse an increase event will actually decrease the lever position bringing
/// it toward the maximum reverse position.
pub const PROP_PITCH1_INCREASE_EX1: &str = "PROP_PITCH1_INCREASE_EX1";
/// Increase the propeller pitch for engine 1/2/3/4, taking into account the reverser status,
/// ie: When in reverse an increase event will actually decrease the lever position bringing
/// it toward the maximum reverse position.
pub const PROP_PITCH2_INCREASE_EX1: &str = "PROP_PITCH2_INCREASE_EX1";
/// Increase the propeller pitch for engine 1/2/3/4, taking into account the reverser status,
/// ie: When in reverse an increase event will actually decrease the lever position bringing
/// it toward the maximum reverse position.
pub const PROP_PITCH3_INCREASE_EX1: &str = "PROP_PITCH3_INCREASE_EX1";
/// Increase the propeller pitch for engine 1/2/3/4, taking into account the reverser status,
/// ie: When in reverse an increase event will actually decrease the lever position bringing
/// it toward the maximum reverse position.
pub const PROP_PITCH4_INCREASE_EX1: &str = "PROP_PITCH4_INCREASE_EX1";
/// Increase the propeller pitch for the indexed engine by half the amount of the normal
/// increase event, taking into account the reverser status, ie: When in reverse an increase
/// event will actually decrease the lever position bringing it toward the maximum reverse
/// position. Parameters: \[0\]: The engine index to target (from 1 to 16, or 0 for all
/// engines).
pub const PROP_PITCH_INCREASE_SMALL_EX1: &str = "PROP_PITCH_INCREASE_SMALL_EX1";
/// Increase the propeller pitch for engine 1/2/3/4 by half the amount of the normal increase
/// event, taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the lever position bringing it toward the maximum reverse position.
pub const PROP_PITCH1_INCREASE_SMALL_EX1: &str = "PROP_PITCH1_INCREASE_SMALL_EX1";
/// Increase the propeller pitch for engine 1/2/3/4 by half the amount of the normal increase
/// event, taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the lever position bringing it toward the maximum reverse position.
pub const PROP_PITCH2_INCREASE_SMALL_EX1: &str = "PROP_PITCH2_INCREASE_SMALL_EX1";
/// Increase the propeller pitch for engine 1/2/3/4 by half the amount of the normal increase
/// event, taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the lever position bringing it toward the maximum reverse position.
pub const PROP_PITCH3_INCREASE_SMALL_EX1: &str = "PROP_PITCH3_INCREASE_SMALL_EX1";
/// Increase the propeller pitch for engine 1/2/3/4 by half the amount of the normal increase
/// event, taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the lever position bringing it toward the maximum reverse position.
pub const PROP_PITCH4_INCREASE_SMALL_EX1: &str = "PROP_PITCH4_INCREASE_SMALL_EX1";
/// Set prop pitch levers max (lo pitch) Parameters: \[0\]: The engine index to target (from 1
/// to 16, or 0 for all engines).
pub const PROP_PITCH_LO: &str = "PROP_PITCH_LO";
/// Set prop pitch lever 1/2/3/4 max (lo pitch)
pub const PROP_PITCH1_LO: &str = "PROP_PITCH1_LO";
/// Set prop pitch lever 1/2/3/4 max (lo pitch)
pub const PROP_PITCH2_LO: &str = "PROP_PITCH2_LO";
/// Set prop pitch lever 1/2/3/4 max (lo pitch)
pub const PROP_PITCH3_LO: &str = "PROP_PITCH3_LO";
/// Set prop pitch lever 1/2/3/4 max (lo pitch)
pub const PROP_PITCH4_LO: &str = "PROP_PITCH4_LO";
/// Set prop pitch levers max (lo pitch) Parameters: \[0\]: The engine index to target (from 1
/// to 16, or 0 for all engines).
pub const PROP_PITCH_LO_EX1: &str = "PROP_PITCH_LO_EX1";
/// Set prop pitch lever 1/2/3/4 max (lo pitch)
pub const PROP_PITCH1_LO_EX1: &str = "PROP_PITCH1_LO_EX1";
/// Set prop pitch lever 1/2/3/4 max (lo pitch)
pub const PROP_PITCH2_LO_EX1: &str = "PROP_PITCH2_LO_EX1";
/// Set prop pitch lever 1/2/3/4 max (lo pitch)
pub const PROP_PITCH3_LO_EX1: &str = "PROP_PITCH3_LO_EX1";
/// Set prop pitch lever 1/2/3/4 max (lo pitch)
pub const PROP_PITCH4_LO_EX1: &str = "PROP_PITCH4_LO_EX1";
/// Set prop pitch levers (0 to 16383) Parameters: \[0\]: The engine index to target (from 1
/// to 16, or 0 for all engines).
pub const PROP_PITCH_SET: &str = "PROP_PITCH_SET";
/// Set prop pitch lever 1/2/3/4 exact value (0 to 16383)
pub const PROP_PITCH1_SET: &str = "PROP_PITCH1_SET";
/// Set prop pitch lever 1/2/3/4 exact value (0 to 16383)
pub const PROP_PITCH2_SET: &str = "PROP_PITCH2_SET";
/// Set prop pitch lever 1/2/3/4 exact value (0 to 16383)
pub const PROP_PITCH3_SET: &str = "PROP_PITCH3_SET";
/// Set prop pitch lever 1/2/3/4 exact value (0 to 16383)
pub const PROP_PITCH4_SET: &str = "PROP_PITCH4_SET";
/// Turns propeller synchronization switch on
pub const TOGGLE_PROPELLER_SYNC: &str = "TOGGLE_PROPELLER_SYNC";
/// Increment or decrement the propeller pitch based on the speed and the distance of the
/// interaction from the user device (values are clamped between 0 to 16384). NOTE: This is
/// primarily for the PS5 controller touch-pad, but should work with any touch-input.
#[cfg(feature = "sunrise")]
pub const PROPELLER_RELATIVE_AXIS: &str = "PROPELLER_RELATIVE_AXIS";
/// Toggle the propeller reverser thrust between on (1) and off (0).
pub const PROPELLER_REVERSE_THRUST_TOGGLE: &str = "PROPELLER_REVERSE_THRUST_TOGGLE";
/// Activate propeller reverse thrust.
pub const PROPELLER_REVERSE_THRUST_HOLD: &str = "PROPELLER_REVERSE_THRUST_HOLD";
/// Turns auto-feather arming switch on.
pub const TOGGLE_AUTOFEATHER_ARM: &str = "TOGGLE_AUTOFEATHER_ARM";
/// Trigger propeller feather switches for the indexed engine. Parameters: \[0\]: The engine
/// index to target (from 1 to 16, or 0 for all engines).
pub const TOGGLE_FEATHER_SWITCHES: &str = "TOGGLE_FEATHER_SWITCHES";
/// Trigger propeller 1/2/3/4 switch.
pub const TOGGLE_FEATHER_SWITCH_1: &str = "TOGGLE_FEATHER_SWITCH_1";
/// Trigger propeller 1/2/3/4 switch.
pub const TOGGLE_FEATHER_SWITCH_2: &str = "TOGGLE_FEATHER_SWITCH_2";
/// Trigger propeller 1/2/3/4 switch.
pub const TOGGLE_FEATHER_SWITCH_3: &str = "TOGGLE_FEATHER_SWITCH_3";
/// Trigger propeller 1/2/3/4 switch.
pub const TOGGLE_FEATHER_SWITCH_4: &str = "TOGGLE_FEATHER_SWITCH_4";
/// Toggles propeller deice switch for all engines.
pub const TOGGLE_PROPELLER_DEICE: &str = "TOGGLE_PROPELLER_DEICE";

// Throttle
/// Subtracts the given value from the throttle of all engines (the final position will depend
/// on the min_throttle_limit value). Parameters: \[0\]: the value between 0 - 16383.
pub const AXIS_THROTTLE_MINUS: &str = "AXIS_THROTTLE_MINUS";
/// Adds the given value to the throttle of all engines. Parameters: \[0\]: the value between
/// 0 - 16383.
pub const AXIS_THROTTLE_PLUS: &str = "AXIS_THROTTLE_PLUS";
/// Set the throttle on the indexed engine. Parameters: \[0\]: The engine index to target
/// (from 1 to 16, or 0 for all engines) \[1\]: the value between 0 - 16383.
pub const AXIS_THROTTLE_SET: &str = "AXIS_THROTTLE_SET";
/// Set throttle 1/2/3/4 exactly (0 - 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const AXIS_THROTTLE1_SET: &str = "AXIS_THROTTLE1_SET";
/// Set throttle 1/2/3/4 exactly (0 - 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const AXIS_THROTTLE2_SET: &str = "AXIS_THROTTLE2_SET";
/// Set throttle 1/2/3/4 exactly (0 - 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const AXIS_THROTTLE3_SET: &str = "AXIS_THROTTLE3_SET";
/// Set throttle 1/2/3/4 exactly (0 - 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const AXIS_THROTTLE4_SET: &str = "AXIS_THROTTLE4_SET";
/// Decrement throttles
pub const DECREASE_THROTTLE: &str = "DECREASE_THROTTLE";
/// Increment throttles
pub const INCREASE_THROTTLE: &str = "INCREASE_THROTTLE";
/// Turn off throttle reverse thrust for the indexed engine. Parameters: \[0\]: The engine
/// index to target (from 1 to 16, or 0 for all engines).
pub const SET_REVERSE_THRUST_OFF: &str = "SET_REVERSE_THRUST_OFF";
/// Turn on throttle reverse thrust for the indexed engine. Parameters: \[0\]: The engine
/// index to target (from 1 to 16, or 0 for all engines).
pub const SET_REVERSE_THRUST_ON: &str = "SET_REVERSE_THRUST_ON";
/// Turn off the throttle reverse thrust for engine 1/2/3/4.
pub const SET_THROTTLE1_REVERSE_THRUST_OFF: &str = "SET_THROTTLE1_REVERSE_THRUST_OFF";
/// Turn off the throttle reverse thrust for engine 1/2/3/4.
pub const SET_THROTTLE2_REVERSE_THRUST_OFF: &str = "SET_THROTTLE2_REVERSE_THRUST_OFF";
/// Turn off the throttle reverse thrust for engine 1/2/3/4.
pub const SET_THROTTLE3_REVERSE_THRUST_OFF: &str = "SET_THROTTLE3_REVERSE_THRUST_OFF";
/// Turn off the throttle reverse thrust for engine 1/2/3/4.
pub const SET_THROTTLE4_REVERSE_THRUST_OFF: &str = "SET_THROTTLE4_REVERSE_THRUST_OFF";
/// Turn on the throttle reverse thrust for engine 1/2/3/4.
pub const SET_THROTTLE1_REVERSE_THRUST_ON: &str = "SET_THROTTLE1_REVERSE_THRUST_ON";
/// Turn on the throttle reverse thrust for engine 1/2/3/4.
pub const SET_THROTTLE2_REVERSE_THRUST_ON: &str = "SET_THROTTLE2_REVERSE_THRUST_ON";
/// Turn on the throttle reverse thrust for engine 1/2/3/4.
pub const SET_THROTTLE3_REVERSE_THRUST_ON: &str = "SET_THROTTLE3_REVERSE_THRUST_ON";
/// Turn on the throttle reverse thrust for engine 1/2/3/4.
pub const SET_THROTTLE4_REVERSE_THRUST_ON: &str = "SET_THROTTLE4_REVERSE_THRUST_ON";
/// Set throttles to 10%
pub const THROTTLE_10: &str = "THROTTLE_10";
/// Set throttles to 20%
pub const THROTTLE_20: &str = "THROTTLE_20";
/// Set throttles to 30%
pub const THROTTLE_30: &str = "THROTTLE_30";
/// Set throttles to 40%
pub const THROTTLE_40: &str = "THROTTLE_40";
/// Set throttles to 50%
pub const THROTTLE_50: &str = "THROTTLE_50";
/// Set throttles to 60%
pub const THROTTLE_60: &str = "THROTTLE_60";
/// Set throttles to 70%
pub const THROTTLE_70: &str = "THROTTLE_70";
/// Set throttles to 80%
pub const THROTTLE_80: &str = "THROTTLE_80";
/// Set throttles to 90%
pub const THROTTLE_90: &str = "THROTTLE_90";
/// Set throttle value for the indexed engine, taking into account the reverser status, ie:
/// When in reverse and setting to 100% the throttle position will be sent back toward minimum
/// position of 0%. Parameters: \[0\]: Index of the engine (starting at 1) or 0 to target all
/// engines. \[1\]: throttle from 0 to 16983.
pub const THROTTLE_AXIS_SET_EX1: &str = "THROTTLE_AXIS_SET_EX1";
/// Set throttle value for engine 1/2/3/4, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the throttle position will be sent back toward minimum
/// position of 0%. Parameters: \[0\]: throttle from 0 to 16983.
pub const THROTTLE1_AXIS_SET_EX1: &str = "THROTTLE1_AXIS_SET_EX1";
/// Set throttle value for engine 1/2/3/4, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the throttle position will be sent back toward minimum
/// position of 0%. Parameters: \[0\]: throttle from 0 to 16983.
pub const THROTTLE2_AXIS_SET_EX1: &str = "THROTTLE2_AXIS_SET_EX1";
/// Set throttle value for engine 1/2/3/4, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the throttle position will be sent back toward minimum
/// position of 0%. Parameters: \[0\]: throttle from 0 to 16983.
pub const THROTTLE3_AXIS_SET_EX1: &str = "THROTTLE3_AXIS_SET_EX1";
/// Set throttle value for engine 1/2/3/4, taking into account the reverser status, ie: When
/// in reverse and setting to 100% the throttle position will be sent back toward minimum
/// position of 0%. Parameters: \[0\]: throttle from 0 to 16983.
pub const THROTTLE4_AXIS_SET_EX1: &str = "THROTTLE4_AXIS_SET_EX1";
/// Set throttles to idle
pub const THROTTLE_CUT: &str = "THROTTLE_CUT";
/// Set throttle 1/2/3/4 to idle
pub const THROTTLE1_CUT: &str = "THROTTLE1_CUT";
/// Set throttle 1/2/3/4 to idle
pub const THROTTLE2_CUT: &str = "THROTTLE2_CUT";
/// Set throttle 1/2/3/4 to idle
pub const THROTTLE3_CUT: &str = "THROTTLE3_CUT";
/// Set throttle 1/2/3/4 to idle
pub const THROTTLE4_CUT: &str = "THROTTLE4_CUT";
/// Set throttles to idle
pub const THROTTLE_CUT_EX1: &str = "THROTTLE_CUT_EX1";
/// Set throttle 1/2/3/4 to idle
pub const THROTTLE1_CUT_EX1: &str = "THROTTLE1_CUT_EX1";
/// Set throttle 1/2/3/4 to idle
pub const THROTTLE2_CUT_EX1: &str = "THROTTLE2_CUT_EX1";
/// Set throttle 1/2/3/4 to idle
pub const THROTTLE3_CUT_EX1: &str = "THROTTLE3_CUT_EX1";
/// Set throttle 1/2/3/4 to idle
pub const THROTTLE4_CUT_EX1: &str = "THROTTLE4_CUT_EX1";
/// Decrement throttles by 10%.
pub const THROTTLE_DECR: &str = "THROTTLE_DECR";
/// Decrement throttle 1/2/3/4 by 10%
pub const THROTTLE1_DECR: &str = "THROTTLE1_DECR";
/// Decrement throttle 1/2/3/4 by 10%
pub const THROTTLE2_DECR: &str = "THROTTLE2_DECR";
/// Decrement throttle 1/2/3/4 by 10%
pub const THROTTLE3_DECR: &str = "THROTTLE3_DECR";
/// Decrement throttle 1/2/3/4 by 10%
pub const THROTTLE4_DECR: &str = "THROTTLE4_DECR";
/// Decrease throttles by 5%.
pub const THROTTLE_DECR_SMALL: &str = "THROTTLE_DECR_SMALL";
/// Decrease throttle 1/2/3/4 by 5%.
pub const THROTTLE1_DECR_SMALL: &str = "THROTTLE1_DECR_SMALL";
/// Decrease throttle 1/2/3/4 by 5%.
pub const THROTTLE2_DECR_SMALL: &str = "THROTTLE2_DECR_SMALL";
/// Decrease throttle 1/2/3/4 by 5%.
pub const THROTTLE3_DECR_SMALL: &str = "THROTTLE3_DECR_SMALL";
/// Decrease throttle 1/2/3/4 by 5%.
pub const THROTTLE4_DECR_SMALL: &str = "THROTTLE4_DECR_SMALL";
/// Decrease the throttle for the indexed engine, taking into account the reverser status, ie:
/// When in reverse a decrease event will actually increase the throttle position bringing it
/// toward the minimum reverse position. Parameters: \[0\]: The engine index to target (from 1
/// to 16, or 0 for all engines).
pub const THROTTLE_DECREASE_EX1: &str = "THROTTLE_DECREASE_EX1";
/// Decrease the throttle for engine 1/2/3/4, taking into account the reverser status, ie:
/// When in reverse a decrease event will actually increase the throttle position bringing it
/// toward the minimum reverse position.
pub const THROTTLE1_DECREASE_EX1: &str = "THROTTLE1_DECREASE_EX1";
/// Decrease the throttle for engine 1/2/3/4, taking into account the reverser status, ie:
/// When in reverse a decrease event will actually increase the throttle position bringing it
/// toward the minimum reverse position.
pub const THROTTLE2_DECREASE_EX1: &str = "THROTTLE2_DECREASE_EX1";
/// Decrease the throttle for engine 1/2/3/4, taking into account the reverser status, ie:
/// When in reverse a decrease event will actually increase the throttle position bringing it
/// toward the minimum reverse position.
pub const THROTTLE3_DECREASE_EX1: &str = "THROTTLE3_DECREASE_EX1";
/// Decrease the throttle for engine 1/2/3/4, taking into account the reverser status, ie:
/// When in reverse a decrease event will actually increase the throttle position bringing it
/// toward the minimum reverse position.
pub const THROTTLE4_DECREASE_EX1: &str = "THROTTLE4_DECREASE_EX1";
/// Decrease the throttle for the indexed engine by half the amount of the normal decrease
/// event, taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the throttle position bringing it toward the minimum reverse position.
/// Parameters: \[0\]: The engine index to target (from 1 to 16, or 0 for all engines).
pub const THROTTLE_DECREASE_SMALL_EX1: &str = "THROTTLE_DECREASE_SMALL_EX1";
/// Decrease the throttle for engine 1/2/3/4 by half the amount of the normal decrease event,
/// taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the throttle position bringing it toward the minimum reverse position.
pub const THROTTLE1_DECREASE_SMALL_EX1: &str = "THROTTLE1_DECREASE_SMALL_EX1";
/// Decrease the throttle for engine 1/2/3/4 by half the amount of the normal decrease event,
/// taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the throttle position bringing it toward the minimum reverse position.
pub const THROTTLE2_DECREASE_SMALL_EX1: &str = "THROTTLE2_DECREASE_SMALL_EX1";
/// Decrease the throttle for engine 1/2/3/4 by half the amount of the normal decrease event,
/// taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the throttle position bringing it toward the minimum reverse position.
pub const THROTTLE3_DECREASE_SMALL_EX1: &str = "THROTTLE3_DECREASE_SMALL_EX1";
/// Decrease the throttle for engine 1/2/3/4 by half the amount of the normal decrease event,
/// taking into account the reverser status, ie: When in reverse a decrease event will
/// actually increase the throttle position bringing it toward the minimum reverse position.
pub const THROTTLE4_DECREASE_SMALL_EX1: &str = "THROTTLE4_DECREASE_SMALL_EX1";
/// By default this is equivalent to THROTTLE_FULL. However it can be intercepted by XML/JS to
/// implement custom behaviors.
#[cfg(feature = "sunrise")]
pub const THROTTLE_DETENT_NEXT: &str = "THROTTLE_DETENT_NEXT";
/// By default this is equivalent to THROTTLE_CUT. However it can be intercepted by XML/JS to
/// implement custom behaviors.
#[cfg(feature = "sunrise")]
pub const THROTTLE_DETENT_PREV: &str = "THROTTLE_DETENT_PREV";
/// Set throttles max
pub const THROTTLE_FULL: &str = "THROTTLE_FULL";
/// Set throttle 1/2/3/4 max
pub const THROTTLE1_FULL: &str = "THROTTLE1_FULL";
/// Set throttle 1/2/3/4 max
pub const THROTTLE2_FULL: &str = "THROTTLE2_FULL";
/// Set throttle 1/2/3/4 max
pub const THROTTLE3_FULL: &str = "THROTTLE3_FULL";
/// Set throttle 1/2/3/4 max
pub const THROTTLE4_FULL: &str = "THROTTLE4_FULL";
/// Set the indexed throttle to full power, taking into consideration the reverser status.
/// Parameters: \[0\]: The engine index to target (from 1 to 16, or 0 for all engines).
pub const THROTTLE_FULL_EX1: &str = "THROTTLE_FULL_EX1";
/// Set the throttle 1/2/3/4 to full power, taking into consideration the reverser status.
pub const THROTTLE1_FULL_EX1: &str = "THROTTLE1_FULL_EX1";
/// Set the throttle 1/2/3/4 to full power, taking into consideration the reverser status.
pub const THROTTLE2_FULL_EX1: &str = "THROTTLE2_FULL_EX1";
/// Set the throttle 1/2/3/4 to full power, taking into consideration the reverser status.
pub const THROTTLE3_FULL_EX1: &str = "THROTTLE3_FULL_EX1";
/// Set the throttle 1/2/3/4 to full power, taking into consideration the reverser status.
pub const THROTTLE4_FULL_EX1: &str = "THROTTLE4_FULL_EX1";
/// Increase all throttles by 10%.
pub const THROTTLE_INCR: &str = "THROTTLE_INCR";
/// Increase throttles 1/2/3/4 by 10%.
pub const THROTTLE1_INCR: &str = "THROTTLE1_INCR";
/// Increase throttles 1/2/3/4 by 10%.
pub const THROTTLE2_INCR: &str = "THROTTLE2_INCR";
/// Increase throttles 1/2/3/4 by 10%.
pub const THROTTLE3_INCR: &str = "THROTTLE3_INCR";
/// Increase throttles 1/2/3/4 by 10%.
pub const THROTTLE4_INCR: &str = "THROTTLE4_INCR";
/// Increase all throttles by 5%.
#[cfg(feature = "sunrise")]
pub const THROTTLE_INCR_SMALL: &str = "THROTTLE_INCR_SMALL";
/// Increase throttles 1/2/3/4 by 5%.
pub const THROTTLE1_INCR_SMALL: &str = "THROTTLE1_INCR_SMALL";
/// Increase throttles 1/2/3/4 by 5%.
pub const THROTTLE2_INCR_SMALL: &str = "THROTTLE2_INCR_SMALL";
/// Increase throttles 1/2/3/4 by 5%.
pub const THROTTLE3_INCR_SMALL: &str = "THROTTLE3_INCR_SMALL";
/// Increase throttles 1/2/3/4 by 5%.
pub const THROTTLE4_INCR_SMALL: &str = "THROTTLE4_INCR_SMALL";
/// Increase the throttle for the indexed engine, taking into account the reverser status, ie:
/// When in reverse an increase event will actually decrease the throttle position bringing it
/// toward the maximum reverse position. Parameters: \[0\]: The engine index to target (from 1
/// to 16, or 0 for all engines).
pub const THROTTLE_INCREASE_EX1: &str = "THROTTLE_INCREASE_EX1";
/// Increase the throttle for engine 1/2/3/4, taking into account the reverser status, ie:
/// When in reverse an increase event will actually decrease the throttle position bringing it
/// toward the maximum reverse position.
pub const THROTTLE1_INCREASE_EX1: &str = "THROTTLE1_INCREASE_EX1";
/// Increase the throttle for engine 1/2/3/4, taking into account the reverser status, ie:
/// When in reverse an increase event will actually decrease the throttle position bringing it
/// toward the maximum reverse position.
pub const THROTTLE2_INCREASE_EX1: &str = "THROTTLE2_INCREASE_EX1";
/// Increase the throttle for engine 1/2/3/4, taking into account the reverser status, ie:
/// When in reverse an increase event will actually decrease the throttle position bringing it
/// toward the maximum reverse position.
pub const THROTTLE3_INCREASE_EX1: &str = "THROTTLE3_INCREASE_EX1";
/// Increase the throttle for engine 1/2/3/4, taking into account the reverser status, ie:
/// When in reverse an increase event will actually decrease the throttle position bringing it
/// toward the maximum reverse position.
pub const THROTTLE4_INCREASE_EX1: &str = "THROTTLE4_INCREASE_EX1";
/// Increase the throttle for the indexed engine by half the amount of the normal increase
/// event, taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the throttle position bringing it toward the maximum reverse position.
/// Parameters: \[0\]: The engine index to target (from 1 to 16, or 0 for all engines).
pub const THROTTLE_INCREASE_SMALL_EX1: &str = "THROTTLE_INCREASE_SMALL_EX1";
/// Increase the throttle for engine 1/2/3/4 by half the amount of the normal increase event,
/// taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the throttle position bringing it toward the maximum reverse position.
pub const THROTTLE1_INCREASE_SMALL_EX1: &str = "THROTTLE1_INCREASE_SMALL_EX1";
/// Increase the throttle for engine 1/2/3/4 by half the amount of the normal increase event,
/// taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the throttle position bringing it toward the maximum reverse position.
pub const THROTTLE2_INCREASE_SMALL_EX2: &str = "THROTTLE2_INCREASE_SMALL_EX2";
/// Increase the throttle for engine 1/2/3/4 by half the amount of the normal increase event,
/// taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the throttle position bringing it toward the maximum reverse position.
pub const THROTTLE3_INCREASE_SMALL_EX1: &str = "THROTTLE3_INCREASE_SMALL_EX1";
/// Increase the throttle for engine 1/2/3/4 by half the amount of the normal increase event,
/// taking into account the reverser status, ie: When in reverse an increase event will
/// actually decrease the throttle position bringing it toward the maximum reverse position.
pub const THROTTLE4_INCREASE_SMALL_EX1: &str = "THROTTLE4_INCREASE_SMALL_EX1";
/// By default this is equivalent to THROTTLE_DECR. However it can be intercepted by XML/JS to
/// implement custom behaviors.
#[cfg(feature = "sunrise")]
pub const THROTTLE_RANGE_DECR: &str = "THROTTLE_RANGE_DECR";
/// By default this is equivalent to THROTTLE_INCR. However it can be intercepted by XML/JS to
/// implement custom behaviors.
#[cfg(feature = "sunrise")]
pub const THROTTLE_RANGE_INCR: &str = "THROTTLE_RANGE_INCR";
/// Increment or decrement the throttle based on the speed and the distance of the interaction
/// from the user device (values are clamped between 0 to 16384). NOTE: This is primarily for
/// the PS5 controller touch-pad, but should work with any touch-input.
#[cfg(feature = "sunrise")]
pub const THROTTLE_RELATIVE_AXIS: &str = "THROTTLE_RELATIVE_AXIS";
/// Toggles the reverser on (1) or off (0).
pub const THROTTLE_REVERSE_THRUST_TOGGLE: &str = "THROTTLE_REVERSE_THRUST_TOGGLE";
/// Activates the reverser for all engines.
pub const THROTTLE_REVERSE_THRUST_HOLD: &str = "THROTTLE_REVERSE_THRUST_HOLD";
/// Activates the reverser for engine 1/2/3/4.
pub const THROTTLE1_REVERSE_THRUST_HOLD: &str = "THROTTLE1_REVERSE_THRUST_HOLD";
/// Activates the reverser for engine 1/2/3/4.
pub const THROTTLE2_REVERSE_THRUST_HOLD: &str = "THROTTLE2_REVERSE_THRUST_HOLD";
/// Activates the reverser for engine 1/2/3/4.
pub const THROTTLE3_REVERSE_THRUST_HOLD: &str = "THROTTLE3_REVERSE_THRUST_HOLD";
/// Activates the reverser for engine 1/2/3/4.
pub const THROTTLE4_REVERSE_THRUST_HOLD: &str = "THROTTLE4_REVERSE_THRUST_HOLD";
/// Set throttles exactly (0- 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const THROTTLE_SET: &str = "THROTTLE_SET";
/// Set throttle 1/2/3/4 exactly (0 to 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const THROTTLE1_SET: &str = "THROTTLE1_SET";
/// Set throttle 1/2/3/4 exactly (0 to 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const THROTTLE2_SET: &str = "THROTTLE2_SET";
/// Set throttle 1/2/3/4 exactly (0 to 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const THROTTLE3_SET: &str = "THROTTLE3_SET";
/// Set throttle 1/2/3/4 exactly (0 to 16383) Parameters: \[0\]: the value between 0 - 16383.
pub const THROTTLE4_SET: &str = "THROTTLE4_SET";
/// Toggle on or off the reverse thruster for engine 1/2/3/4.
pub const TOGGLE_THROTTLE1_REVERSE_THRUST: &str = "TOGGLE_THROTTLE1_REVERSE_THRUST";
/// Toggle on or off the reverse thruster for engine 1/2/3/4.
pub const TOGGLE_THROTTLE2_REVERSE_THRUST: &str = "TOGGLE_THROTTLE2_REVERSE_THRUST";
/// Toggle on or off the reverse thruster for engine 1/2/3/4.
pub const TOGGLE_THROTTLE3_REVERSE_THRUST: &str = "TOGGLE_THROTTLE3_REVERSE_THRUST";
/// Toggle on or off the reverse thruster for engine 1/2/3/4.
pub const TOGGLE_THROTTLE4_REVERSE_THRUST: &str = "TOGGLE_THROTTLE4_REVERSE_THRUST";

// Turbine
/// Setting this to TRUE will "isolate" the engine, effectively nullyfing the engine drag and
/// thrust. This key takes two parameters: an engine number (from 1 to 4 to flag a specific
/// engine, or 0 to affect all engines), and a TRUE / FALSE second parameter to set the engine
/// isolation. IMPORTANT: This event is only applicable to the DarkStar aircraft and should
/// not be used for your own aircraft. Parameters: \[0\]: Engine index (1 to 16) \[1\]: State
/// (TRUE / FALSE).
pub const ISOLATE_TURBINE_SET: &str = "ISOLATE_TURBINE_SET";
/// Using this key will "isolate" the given engine, effectively nullyfing the engine drag and
/// thrust. This key takes an engine number as a parameter (from 1 to 4 to flag a specific
/// engine, or 0 to affect all engines). IMPORTANT: This event is only applicable to the
/// DarkStar aircraft and should not be used for your own aircraft. Parameters: \[0\]: Engine
/// index (1 to 16).
pub const ISOLATE_TURBINE_ON: &str = "ISOLATE_TURBINE_ON";
/// Using this key will end the "isolation" for the engine, effectively enabling the engine
/// drag and thrust again. This key takes an engine number as a parameter (from 1 to 4 to flag
/// a specific engine, or 0 to affect all engines). IMPORTANT: This event is only applicable
/// to the DarkStar aircraft and should not be used for your own aircraft. Parameters: \[0\]:
/// Engine index (1 to 16).
pub const ISOLATE_TURBINE_OFF: &str = "ISOLATE_TURBINE_OFF";
/// This key can be used to toggle an engines "isolated" state, where an isolated engine has
/// its drag and thrust effectively nullified. This key takes an engine number as a parameter
/// (from 1 to 4 to flag a specific engine, or 0 to affect all engines). IMPORTANT: This event
/// is only applicable to the DarkStar aircraft and should not be used for your own aircraft.
/// Parameters: \[0\]: Engine index (1 to 16).
pub const ISOLATE_TURBINE_TOGGLE: &str = "ISOLATE_TURBINE_TOGGLE";
/// Set the indexed engine ignition either on (1) or off (0). Parameters: \[0\]: The engine
/// index to target (from 1 to 16, or 0 for all engines).
pub const TURBINE_IGNITION_SWITCH_SET: &str = "TURBINE_IGNITION_SWITCH_SET";
/// Set engine 1/2/3/4 ignition either on (1) or off (0).
pub const TURBINE_IGNITION_SWITCH_SET1: &str = "TURBINE_IGNITION_SWITCH_SET1";
/// Set engine 1/2/3/4 ignition either on (1) or off (0).
pub const TURBINE_IGNITION_SWITCH_SET2: &str = "TURBINE_IGNITION_SWITCH_SET2";
/// Set engine 1/2/3/4 ignition either on (1) or off (0).
pub const TURBINE_IGNITION_SWITCH_SET3: &str = "TURBINE_IGNITION_SWITCH_SET3";
/// Set engine 1/2/3/4 ignition either on (1) or off (0).
pub const TURBINE_IGNITION_SWITCH_SET4: &str = "TURBINE_IGNITION_SWITCH_SET4";
/// Turn the turbine ignition switch on or off.
pub const TURBINE_IGNITION_SWITCH_TOGGLE: &str = "TURBINE_IGNITION_SWITCH_TOGGLE";

// Starter
/// Selects jet engine starter (for +/- sequence) Parameters: JET_STARTER \[0\]: The engine
/// index to target (from 1 to 16, or 0 for all engines).
pub const JET_STARTER: &str = "JET_STARTER";
/// Set the Starter for engine 1/2/3/4 to on or off. If set to on (TRUE) the starter will stay
/// on, and setting the event to off (FALSE) will disable the starter, but only after the
/// engine RPM is above the 50% threshold. To disable the starter immediately you should use
/// STARTER1_SET, and note that turbine engines will need both these events triggered to off
/// (FALSE). Parameters: \[0\]: Bool.
pub const SET_STARTER1_HELD: &str = "SET_STARTER1_HELD";
/// Set the Starter for engine 1/2/3/4 to on or off. If set to on (TRUE) the starter will stay
/// on, and setting the event to off (FALSE) will disable the starter, but only after the
/// engine RPM is above the 50% threshold. To disable the starter immediately you should use
/// STARTER1_SET, and note that turbine engines will need both these events triggered to off
/// (FALSE). Parameters: \[0\]: Bool.
pub const SET_STARTER2_HELD: &str = "SET_STARTER2_HELD";
/// Set the Starter for engine 1/2/3/4 to on or off. If set to on (TRUE) the starter will stay
/// on, and setting the event to off (FALSE) will disable the starter, but only after the
/// engine RPM is above the 50% threshold. To disable the starter immediately you should use
/// STARTER1_SET, and note that turbine engines will need both these events triggered to off
/// (FALSE). Parameters: \[0\]: Bool.
pub const SET_STARTER3_HELD: &str = "SET_STARTER3_HELD";
/// Set the Starter for engine 1/2/3/4 to on or off. If set to on (TRUE) the starter will stay
/// on, and setting the event to off (FALSE) will disable the starter, but only after the
/// engine RPM is above the 50% threshold. To disable the starter immediately you should use
/// STARTER1_SET, and note that turbine engines will need both these events triggered to off
/// (FALSE). Parameters: \[0\]: Bool.
pub const SET_STARTER4_HELD: &str = "SET_STARTER4_HELD";
/// Set the Starter for all engines to on or off. If set to on (TRUE) the starter will stay on
/// until set to off (FALSE) with another call to the event. Parameters: \[0\]: Bool.
pub const SET_STARTER_ALL_HELD: &str = "SET_STARTER_ALL_HELD";
/// Set the Starter for engine 1/2/3/4 to on or off. Note that the starter will only stay on
/// for a short time before switching itself off again on piston engines. If you wish the
/// starter to stay on, use SET_STARTER1_HELD. Parameters: \[0\]: Bool.
pub const STARTER1_SET: &str = "STARTER1_SET";
/// Set the Starter for engine 1/2/3/4 to on or off. Note that the starter will only stay on
/// for a short time before switching itself off again on piston engines. If you wish the
/// starter to stay on, use SET_STARTER1_HELD. Parameters: \[0\]: Bool.
pub const STARTER2_SET: &str = "STARTER2_SET";
/// Set the Starter for engine 1/2/3/4 to on or off. Note that the starter will only stay on
/// for a short time before switching itself off again on piston engines. If you wish the
/// starter to stay on, use SET_STARTER1_HELD. Parameters: \[0\]: Bool.
pub const STARTER3_SET: &str = "STARTER3_SET";
/// Set the Starter for engine 1/2/3/4 to on or off. Note that the starter will only stay on
/// for a short time before switching itself off again on piston engines. If you wish the
/// starter to stay on, use SET_STARTER1_HELD. Parameters: \[0\]: Bool.
pub const STARTER4_SET: &str = "STARTER4_SET";
/// Toggle starters.
pub const TOGGLE_ALL_STARTERS: &str = "TOGGLE_ALL_STARTERS";
/// Toggle starter for master engine.
pub const TOGGLE_MASTER_STARTER_SWITCH: &str = "TOGGLE_MASTER_STARTER_SWITCH";
/// Toggle starter 1/2/3/4.
pub const TOGGLE_STARTER1: &str = "TOGGLE_STARTER1";
/// Toggle starter 1/2/3/4.
pub const TOGGLE_STARTER2: &str = "TOGGLE_STARTER2";
/// Toggle starter 1/2/3/4.
pub const TOGGLE_STARTER3: &str = "TOGGLE_STARTER3";
/// Toggle starter 1/2/3/4.
pub const TOGGLE_STARTER4: &str = "TOGGLE_STARTER4";
