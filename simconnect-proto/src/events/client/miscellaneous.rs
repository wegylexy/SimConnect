// Demo / Replay
/// Record 1 second demo.
pub const DEMO_RECORD_1_SEC: &str = "DEMO_RECORD_1_SEC";
/// Record 5 second demo.
pub const DEMO_RECORD_5_SEC: &str = "DEMO_RECORD_5_SEC";
/// Record a message with the demo.
#[cfg(feature = "sunrise")]
pub const DEMO_RECORD_MESSAGE: &str = "DEMO_RECORD_MESSAGE";
/// Stops demo system recording.
pub const DEMO_RECORD_STOP: &str = "DEMO_RECORD_STOP";
/// Stops demo system playback.
pub const DEMO_STOP: &str = "DEMO_STOP";
/// Stops replay system playback.
pub const REPLAY_STOP: &str = "REPLAY_STOP";

// Character Actions
/// Make character look down (<0) / up (>0) in first person view. Parameters: \[0\] Value.
#[cfg(feature = "sunrise")]
pub const AXIS_PC_FPV_ROTATION_X: &str = "AXIS_PC_FPV_ROTATION_X";
/// Make character look left (<0) / right (>0) in first person view. Parameters: \[0\] Value.
#[cfg(feature = "sunrise")]
pub const AXIS_PC_FPV_ROTATION_Y: &str = "AXIS_PC_FPV_ROTATION_Y";
/// Make character move left (<0) / right (>0). Parameters: \[0\] Value.
#[cfg(feature = "sunrise")]
pub const AXIS_PC_MOVE_X: &str = "AXIS_PC_MOVE_X";
/// Make character move backwards (<0) / forwards (>0). Parameters: \[0\] Value.
#[cfg(feature = "sunrise")]
pub const AXIS_PC_MOVE_Z: &str = "AXIS_PC_MOVE_Z";
/// Toggle the character between crouching and standing.
#[cfg(feature = "sunrise")]
pub const PC_CROUCH_TOGGLE: &str = "PC_CROUCH_TOGGLE";
/// Make character look down in first person view.
#[cfg(feature = "sunrise")]
pub const PC_FPV_LOOK_DOWN: &str = "PC_FPV_LOOK_DOWN";
/// Make character look down and left in first person view.
#[cfg(feature = "sunrise")]
pub const PC_FPV_LOOK_DOWN_LEFT: &str = "PC_FPV_LOOK_DOWN_LEFT";
/// Make character look down and right in first person view.
#[cfg(feature = "sunrise")]
pub const PC_FPV_LOOK_DOWN_RIGHT: &str = "PC_FPV_LOOK_DOWN_RIGHT";
/// Make character look left in first person view.
#[cfg(feature = "sunrise")]
pub const PC_FPV_LOOK_LEFT: &str = "PC_FPV_LOOK_LEFT";
/// Make character look right in first person view.
#[cfg(feature = "sunrise")]
pub const PC_FPV_LOOK_RIGHT: &str = "PC_FPV_LOOK_RIGHT";
/// Make character look up in first person view.
#[cfg(feature = "sunrise")]
pub const PC_FPV_LOOK_UP: &str = "PC_FPV_LOOK_UP";
/// Make character look up and left in first person view.
#[cfg(feature = "sunrise")]
pub const PC_FPV_LOOK_UP_LEFT: &str = "PC_FPV_LOOK_UP_LEFT";
/// Make character look up and right in first person view.
#[cfg(feature = "sunrise")]
pub const PC_FPV_LOOK_UP_RIGHT: &str = "PC_FPV_LOOK_UP_RIGHT";
/// Make character move backward.
#[cfg(feature = "sunrise")]
pub const PC_MOVE_BACKWARD: &str = "PC_MOVE_BACKWARD";
/// Make character move backward to the left.
#[cfg(feature = "sunrise")]
pub const PC_MOVE_BACKWARD_LEFT: &str = "PC_MOVE_BACKWARD_LEFT";
/// Make character move backward to the right.
#[cfg(feature = "sunrise")]
pub const PC_MOVE_BACKWARD_RIGHT: &str = "PC_MOVE_BACKWARD_RIGHT";
/// Make character move forward.
#[cfg(feature = "sunrise")]
pub const PC_MOVE_FORWARD: &str = "PC_MOVE_FORWARD";
/// Make character move forward to the right.
#[cfg(feature = "sunrise")]
pub const PC_MOVE_FORWARD_RIGHT: &str = "PC_MOVE_FORWARD_RIGHT";
/// Make character move forward to the left.
#[cfg(feature = "sunrise")]
pub const PC_MOVE_FORWARD_LEFT: &str = "PC_MOVE_FORWARD_LEFT";
/// Make character strafe left.
#[cfg(feature = "sunrise")]
pub const PC_MOVE_LEFT: &str = "PC_MOVE_LEFT";
/// Make character strafe right.
#[cfg(feature = "sunrise")]
pub const PC_MOVE_RIGHT: &str = "PC_MOVE_RIGHT";
/// Set movement speed to run. Parameters: \[0\] Value.
#[cfg(feature = "sunrise")]
pub const PC_RUN_SET: &str = "PC_RUN_SET";

// Miscellaneous
/// Quit Microsoft Flight Simulator 2024 without a message.
pub const ABORT: &str = "ABORT";
/// Not used by the simulation. Parameters: -.
pub const ADVENTURE_ACTION: &str = "ADVENTURE_ACTION";
/// Not used by the simulation. Parameters: -.
pub const ANALYSIS_MANEUVER_STOP: &str = "ANALYSIS_MANEUVER_STOP";
/// Increases the audio panel sound volume by 0.01. The resulting value will be clamped
/// between 0 and 1.
pub const AUDIO_PANEL_VOLUME_INC: &str = "AUDIO_PANEL_VOLUME_INC";
/// Decreases the audio panel sound volume by 0.01. The resulting value will be clamped
/// between 0 and 1.
pub const AUDIO_PANEL_VOLUME_DEC: &str = "AUDIO_PANEL_VOLUME_DEC";
/// Set the audio panel volume to the given value, clamped between 0 and 1. Parameters: \[0\]:
/// Volume.
pub const AUDIO_PANEL_VOLUME_SET: &str = "AUDIO_PANEL_VOLUME_SET";
/// Not used in the simulation. Parameters: -.
pub const AUTOCOORD_OFF: &str = "AUTOCOORD_OFF";
/// Not used in the simulation. Parameters: -.
pub const AUTOCOORD_ON: &str = "AUTOCOORD_ON";
/// Not used in the simulation. Parameters: -.
pub const AUTOCOORD_SET: &str = "AUTOCOORD_SET";
/// Switch inversion of Y axis controls on or off.
pub const AUTOCOORD_TOGGLE: &str = "AUTOCOORD_TOGGLE";
/// Toggle gyro controls on or off.
#[cfg(feature = "sunrise")]
pub const AXIS_SENSOR_TOGGLE: &str = "AXIS_SENSOR_TOGGLE";
/// Capture the current view as a screenshot. Which will be saved to a bmp file in: My
/// Documents\Pictures\
pub const CAPTURE_SCREENSHOT: &str = "CAPTURE_SCREENSHOT";
/// Reloads the user aircraft data (from cache if same type loaded as an AI, otherwise from
/// disk).
pub const RELOAD_USER_AIRCRAFT: &str = "RELOAD_USER_AIRCRAFT";
/// Quit Microsoft Flight Simulator 2024 with a message.
pub const EXIT: &str = "EXIT";
/// Generic key event to set a number value. Parameters: \[0\]: Value.
pub const EXTERNAL_SYSTEM_SET: &str = "EXTERNAL_SYSTEM_SET";
/// Generic key event to toggle a value on/off (true/false).
pub const EXTERNAL_SYSTEM_TOGGLE: &str = "EXTERNAL_SYSTEM_TOGGLE";
/// Brings up flight map.
pub const FLIGHT_MAP: &str = "FLIGHT_MAP";
/// Force the termination of the current scenario or flight
pub const FORCE_END: &str = "FORCE_END";
/// Not used in the simulation.
pub const FULL_WINDOW_TOGGLE: &str = "FULL_WINDOW_TOGGLE";
/// Brings up Help system.
pub const INVOKE_HELP: &str = "INVOKE_HELP";
/// Toggles joystick calibration on/off.
pub const JOYSTICK_CALIBRATE: &str = "JOYSTICK_CALIBRATE";
/// Brings up the keyboard overlay.
pub const KEYBOARD_OVERLAY: &str = "KEYBOARD_OVERLAY";
/// Toggles kneeboard view. Not used in the simulation.
pub const KNEEBOARD_VIEW: &str = "KNEEBOARD_VIEW";
/// Not used in the simulation. Parameters: -.
pub const LABEL_COLOR_CYCLE: &str = "LABEL_COLOR_CYCLE";
/// Not used in the simulation. Parameters: -.
pub const LOD_ZOOM_IN: &str = "LOD_ZOOM_IN";
/// Not used in the simulation. Parameters: -.
pub const LOD_ZOOM_OUT: &str = "LOD_ZOOM_OUT";
/// Begin an event from a macro. Parameters: \[0\] Macro.
pub const MACRO_BEGIN: &str = "MACRO_BEGIN";
/// End an event from a macro. Parameters: \[0\] Macro.
pub const MACRO_END: &str = "MACRO_END";
/// Used in conjunction with "selected" parameters to decrease their value (e.g.,radio
/// frequency)
pub const MINUS: &str = "MINUS";
/// Used with other events
pub const MINUS_SHIFT: &str = "MINUS_SHIFT";
/// Switch Mouse Look mode on or off. Mouse Look mode enables a user to control their view
/// using the mouse, and holding down the space bar.
pub const MOUSE_LOOK_TOGGLE: &str = "MOUSE_LOOK_TOGGLE";
/// Toggle the EFB visibility.
#[cfg(feature = "sunrise")]
pub const MENU_SR_EFB_TOGGLE: &str = "MENU_SR_EFB_TOGGLE";
/// Open menu to kick player from Reno race event.
#[cfg(feature = "sunrise")]
pub const MENU_RENO_KICK_PLAYER: &str = "MENU_RENO_KICK_PLAYER";
/// Not currently used in the simulation.
pub const OVERLAYMENU: &str = "OVERLAYMENU";
/// Toggles panels 1 - 9.
pub const PANEL_1: &str = "PANEL_1";
/// Toggles panels 1 - 9.
pub const PANEL_2: &str = "PANEL_2";
/// Toggles panels 1 - 9.
pub const PANEL_3: &str = "PANEL_3";
/// Toggles panels 1 - 9.
pub const PANEL_4: &str = "PANEL_4";
/// Toggles panels 1 - 9.
pub const PANEL_5: &str = "PANEL_5";
/// Toggles panels 1 - 9.
pub const PANEL_6: &str = "PANEL_6";
/// Toggles panels 1 - 9.
pub const PANEL_7: &str = "PANEL_7";
/// Toggles panels 1 - 9.
pub const PANEL_8: &str = "PANEL_8";
/// Toggles panels 1 - 9.
pub const PANEL_9: &str = "PANEL_9";
/// Toggles indexed panel (1 to 9) Parameters: \[0\]: panel index.
pub const PANEL_ID_TOGGLE: &str = "PANEL_ID_TOGGLE";
/// Opens indexed panel (1 to 9) Parameters: \[0\]: panel index.
pub const PANEL_ID_OPEN: &str = "PANEL_ID_OPEN";
/// Closes indexed panel (1 to 9) Parameters: \[0\]: panel index.
pub const PANEL_ID_CLOSE: &str = "PANEL_ID_CLOSE";
/// Not currently used in the simulation.
pub const PANEL_SELECT_1: &str = "PANEL_SELECT_1";
/// Not currently used in the simulation.
pub const PANEL_SELECT_2: &str = "PANEL_SELECT_2";
/// Toggles the indexed panel. Parameters: \[0\]: panel index.
pub const PANEL_TOGGLE: &str = "PANEL_TOGGLE";
/// Toggles pause on/off
pub const PAUSE_TOGGLE: &str = "PAUSE_TOGGLE";
/// Turns pause on
pub const PAUSE_ON: &str = "PAUSE_ON";
/// Turns pause off
pub const PAUSE_OFF: &str = "PAUSE_OFF";
/// Sets pause on/off (1,0) Parameters: \[0\]: Bool.
pub const PAUSE_SET: &str = "PAUSE_SET";
/// Used in conjunction with "selected" parameters to increase their value (e.g.,radio
/// frequency)
pub const PLUS: &str = "PLUS";
/// Used with other events
pub const PLUS_SHIFT: &str = "PLUS_SHIFT";
/// Cycle through information readouts
pub const READOUTS_FLIGHT: &str = "READOUTS_FLIGHT";
/// Cycle through information readouts while in slew
pub const READOUTS_SLEW: &str = "READOUTS_SLEW";
/// Reloads scenery.
pub const REFRESH_SCENERY: &str = "REFRESH_SCENERY";
/// Reload panel data
pub const RELOAD_PANELS: &str = "RELOAD_PANELS";
/// Requests catering truck on the nearest airport.
pub const REQUEST_CATERING: &str = "REQUEST_CATERING";
/// Requests a baggage loader from the nearest airport.
pub const REQUEST_LUGGAGE: &str = "REQUEST_LUGGAGE";
/// Requests ground power unit from the nearest airport.
pub const REQUEST_POWER_SUPPLY: &str = "REQUEST_POWER_SUPPLY";
/// Sets "selected" index (for other events) to 1/2/3/4.
pub const SELECT_1: &str = "SELECT_1";
/// Sets "selected" index (for other events) to 1/2/3/4.
pub const SELECT_2: &str = "SELECT_2";
/// Sets "selected" index (for other events) to 1/2/3/4.
pub const SELECT_3: &str = "SELECT_3";
/// Sets "selected" index (for other events) to 1/2/3/4.
pub const SELECT_4: &str = "SELECT_4";
/// Resets aircraft state
pub const SIM_RESET: &str = "SIM_RESET";
/// Display the ATC window.
pub const SIMUI_WINDOW_HIDESHOW: &str = "SIMUI_WINDOW_HIDESHOW";
/// Saves flight situation
pub const SITUATION_SAVE: &str = "SITUATION_SAVE";
/// Resets flight situation
pub const SITUATION_RESET: &str = "SITUATION_RESET";
/// Turns sound off
pub const SOUND_OFF: &str = "SOUND_OFF";
/// Turns sound on
pub const SOUND_ON: &str = "SOUND_ON";
/// Sets sound on/off (1,0) Parameters: \[0\]: Bool.
pub const SOUND_SET: &str = "SOUND_SET";
/// Toggles sound on/off
pub const SOUND_TOGGLE: &str = "SOUND_TOGGLE";
/// Set text scrolling on/off. Parameters: \[0\] Bool.
pub const TEXT_SCROLL_SET: &str = "TEXT_SCROLL_SET";
/// Toggles aircraft labels
pub const TOGGLE_AIRCRAFT_LABELS: &str = "TOGGLE_AIRCRAFT_LABELS";
/// Turn on or off the airport name.
pub const TOGGLE_AIRPORT_NAME_DISPLAY: &str = "TOGGLE_AIRPORT_NAME_DISPLAY";
/// Not currently used in the simulation.
pub const TOGGLE_DAMAGE_TEXT: &str = "TOGGLE_DAMAGE_TEXT";
/// Not currently used in the simulation.
pub const TOGGLE_ENEMY_INDICATOR: &str = "TOGGLE_ENEMY_INDICATOR";
/// Requests a jetway, which will only be answered if the aircraft is at a parking spot, or
/// sends already requested jetway away.
pub const TOGGLE_JETWAY: &str = "TOGGLE_JETWAY";
/// Requests a boarding ramp from the nearest airport, or sends an already requested boarding
/// ramp away.
pub const TOGGLE_RAMPTRUCK: &str = "TOGGLE_RAMPTRUCK";
/// Not currently used in the simulation.
pub const TOOLTIP_UNITS_SET: &str = "TOOLTIP_UNITS_SET";
/// Not currently used in the simulation.
pub const TOOLTIP_UNITS_TOGGLE: &str = "TOOLTIP_UNITS_TOGGLE";
/// (no description provided by the vendor docs)
pub const USERINTERRUPT: &str = "USERINTERRUPT";
/// Turn on or off the video recording feature. This records uncompressed AVI format files to:
/// My Documents\Videos\
pub const VIDEO_RECORD_TOGGLE: &str = "VIDEO_RECORD_TOGGLE";
/// Not used in the simulation (Internal Debug Only).
pub const WINDOW_TITLES_SET: &str = "WINDOW_TITLES_SET";
/// Not used in the simulation (Internal Debug Only).
pub const VIEW_WINDOW_TITLES_TOGGLE: &str = "VIEW_WINDOW_TITLES_TOGGLE";
/// Switch inversion of Y axis controls on or off.
pub const YAXIS_INVERT_TOGGLE: &str = "YAXIS_INVERT_TOGGLE";

// Debug
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_0: &str = "DEBUG_0";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_1: &str = "DEBUG_1";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_2: &str = "DEBUG_2";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_3: &str = "DEBUG_3";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_4: &str = "DEBUG_4";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_5: &str = "DEBUG_5";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_6: &str = "DEBUG_6";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_7: &str = "DEBUG_7";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_8: &str = "DEBUG_8";
/// Triggers the keys 1 - 9 for debugging.
pub const DEBUG_9: &str = "DEBUG_9";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_A: &str = "DEBUG_A";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_B: &str = "DEBUG_B";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_C: &str = "DEBUG_C";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_D: &str = "DEBUG_D";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_E: &str = "DEBUG_E";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_F: &str = "DEBUG_F";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_G: &str = "DEBUG_G";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_H: &str = "DEBUG_H";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_I: &str = "DEBUG_I";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_J: &str = "DEBUG_J";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_K: &str = "DEBUG_K";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_L: &str = "DEBUG_L";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_M: &str = "DEBUG_M";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_N: &str = "DEBUG_N";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_O: &str = "DEBUG_O";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_P: &str = "DEBUG_P";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_Q: &str = "DEBUG_Q";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_R: &str = "DEBUG_R";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_S: &str = "DEBUG_S";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_T: &str = "DEBUG_T";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_U: &str = "DEBUG_U";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_V: &str = "DEBUG_V";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_W: &str = "DEBUG_W";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_X: &str = "DEBUG_X";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_Y: &str = "DEBUG_Y";
/// Triggers the keys A - Z for debugging. Parameters: KEY_DEBUG_A - Z.
pub const DEBUG_Z: &str = "DEBUG_Z";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_0: &str = "DEBUG_NUMPAD_0";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_1: &str = "DEBUG_NUMPAD_1";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_2: &str = "DEBUG_NUMPAD_2";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_3: &str = "DEBUG_NUMPAD_3";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_4: &str = "DEBUG_NUMPAD_4";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_5: &str = "DEBUG_NUMPAD_5";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_6: &str = "DEBUG_NUMPAD_6";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_7: &str = "DEBUG_NUMPAD_7";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_8: &str = "DEBUG_NUMPAD_8";
/// Triggers the numberpad keys for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_NUMPAD_9: &str = "DEBUG_NUMPAD_9";
/// Triggers the Down Arrow key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_DOWN: &str = "DEBUG_DOWN";
/// Triggers the Enter key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_ENTER: &str = "DEBUG_ENTER";
/// Triggers the Left ALT key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_LALT: &str = "DEBUG_LALT";
/// Triggers the Left Control key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_LCTRL: &str = "DEBUG_LCTRL";
/// Triggers the Left Arrow key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_LEFT: &str = "DEBUG_LEFT";
/// Triggers the Left Shift key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_LSHIFT: &str = "DEBUG_LSHIFT";
/// Triggers the Menu key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_MENU: &str = "DEBUG_MENU";
/// Triggers the Pause key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_PAUSE: &str = "DEBUG_PAUSE";
/// Triggers the Right ALT key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_RALT: &str = "DEBUG_RALT";
/// Triggers the Right Control key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_RCTRL: &str = "DEBUG_RCTRL";
/// Triggers the Right Arrow key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_RIGHT: &str = "DEBUG_RIGHT";
/// Triggers the Right Shift key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_RSHIFT: &str = "DEBUG_RSHIFT";
/// Triggers the Tab key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_TAB: &str = "DEBUG_TAB";
/// Triggers the Arrow Up key for debugging.
#[cfg(feature = "sunrise")]
pub const DEBUG_UP: &str = "DEBUG_UP";

// Drone Controls
/// Set the vertical speed to a value between 0 and 16000. Parameters: \[0\] Value between 0
/// and 16000.
pub const AXIS_VERTICAL_SPEED_SET: &str = "AXIS_VERTICAL_SPEED_SET";
/// By default this will decrement the vertical speed by 1/128, but you may supply a decrement
/// amount as a parameter. Parameters: \[0\] Value between -16000 and 16000 to decrement by
/// (maps to a value between -1 and 1).
pub const VERTICAL_SPEED_DEC: &str = "VERTICAL_SPEED_DEC";
/// By default this will increment the vertical speed by 1/128, but you may supply an
/// increment amount as a parameter. Parameters: \[0\] Value between -16000 and 16000 to
/// increment by (maps to a value between -1 and 1).
pub const VERTICAL_SPEED_INC: &str = "VERTICAL_SPEED_INC";
/// Set the vertical speed to 0.
pub const VERTICAL_SPEED_ZERO: &str = "VERTICAL_SPEED_ZERO";

// Ornithopter Controls
/// Sets the Ornithopter in boost mode.
#[cfg(feature = "sunrise")]
pub const ORNI_BOOST_SET: &str = "ORNI_BOOST_SET";
/// Disablesthe Ornithopter's dive mode.
#[cfg(feature = "sunrise")]
pub const ORNI_DIVE_MODE_OFF: &str = "ORNI_DIVE_MODE_OFF";
/// Enables the Ornithopter's dive mode.
#[cfg(feature = "sunrise")]
pub const ORNI_DIVE_MODE_ON: &str = "ORNI_DIVE_MODE_ON";
/// Enables and disables the Ornithopter's dive mode.
#[cfg(feature = "sunrise")]
pub const ORNI_DIVE_MODE_TOGGLE: &str = "ORNI_DIVE_MODE_TOGGLE";
/// Disables the Ornithopter's glide mode.
#[cfg(feature = "sunrise")]
pub const ORNI_GLIDE_MODE_OFF: &str = "ORNI_GLIDE_MODE_OFF";
/// Enables the Ornithopter's glide mode.
#[cfg(feature = "sunrise")]
pub const ORNI_GLIDE_MODE_ON: &str = "ORNI_GLIDE_MODE_ON";
/// Enables and disables the Ornithopter's glide mode.
#[cfg(feature = "sunrise")]
pub const ORNI_GLIDE_MODE_TOGGLE: &str = "ORNI_GLIDE_MODE_TOGGLE";
/// Sets the Ornithopter in wings brake mode.
#[cfg(feature = "sunrise")]
pub const ORNI_WINGS_BRAKE_SET: &str = "ORNI_WINGS_BRAKE_SET";
/// Unfolds the wings of the ornithopter.
#[cfg(feature = "sunrise")]
pub const WING_FOLD_OFF: &str = "WING_FOLD_OFF";
/// Folds the wings of the ornithopter.
#[cfg(feature = "sunrise")]
pub const WING_FOLD_ON: &str = "WING_FOLD_ON";
/// Sets the wings of the ornithopter to fold (1, TRUE) or unfold (0, FALSE). Parameters:
/// \[0\]: Boolean.
#[cfg(feature = "sunrise")]
pub const WING_FOLD_SET: &str = "WING_FOLD_SET";

// Freezing Position
/// Turns the freezing of the lat/lon position of the aircraft (either user or AI controlled)
/// on or off. If this key event is set, it means that the latitude and longitude of the
/// aircraft are not being controlled by the simulation, so enabling, for example, a
/// SimConnect client to control the position of the aircraft. This can also apply to altitude
/// and attitude. Refer to the simulation variables: IS_LATITUDE_LONGITUDE_FREEZE_ON,
/// IS_ALTITUDE_FREEZE_ON, and IS_ATTITUDE_FREEZE_ON. Refer also to the
/// SimConnect_AIReleaseControl function.
pub const FREEZE_LATITUDE_LONGITUDE_TOGGLE: &str = "FREEZE_LATITUDE_LONGITUDE_TOGGLE";
/// Freezes the lat/lon position of the aircraft.
pub const FREEZE_LATITUDE_LONGITUDE_SET: &str = "FREEZE_LATITUDE_LONGITUDE_SET";
/// Turns the freezing of the altitude of the aircraft on or off.
pub const FREEZE_ALTITUDE_TOGGLE: &str = "FREEZE_ALTITUDE_TOGGLE";
/// Freezes the altitude of the aircraft.
pub const FREEZE_ALTITUDE_SET: &str = "FREEZE_ALTITUDE_SET";
/// Turns the freezing of the attitude (pitch, bank and heading) of the aircraft on or off.
pub const FREEZE_ATTITUDE_TOGGLE: &str = "FREEZE_ATTITUDE_TOGGLE";
/// Freezes the attitude (pitch, bank and heading) of the aircraft.
pub const FREEZE_ATTITUDE_SET: &str = "FREEZE_ATTITUDE_SET";

// Mission Keys
/// Turn the point-of-interest indicator (often a light beam) on or off. Refer to the Missions
/// system documentation. Parameters: -.
pub const POINT_OF_INTEREST_TOGGLE_POINTER: &str = "POINT_OF_INTEREST_TOGGLE_POINTER";
/// Change the current point-of-interest to the previous point-of-interest. Parameters: -.
pub const POINT_OF_INTEREST_CYCLE_PREVIOUS: &str = "POINT_OF_INTEREST_CYCLE_PREVIOUS";
/// Change the current point-of-interest to the next point-of-interest. Parameters: -.
pub const POINT_OF_INTEREST_CYCLE_NEXT: &str = "POINT_OF_INTEREST_CYCLE_NEXT";

// Sim Rate
/// Selects the simulation rate. Use the PLUS and MINUS events to increment/decrement the
/// value. Parameters: -.
pub const SIM_RATE: &str = "SIM_RATE";
/// Decreases the simulation rate, which will slow down the in-simulation time. Parameters: -.
pub const SIM_RATE_DECR: &str = "SIM_RATE_DECR";
/// Increase the simulation rate, which will speed up the in-simulation time. Parameters: -.
pub const SIM_RATE_INCR: &str = "SIM_RATE_INCR";
/// Set the simulation rate. Parameters: \[0\] Value.
pub const SIM_RATE_SET: &str = "SIM_RATE_SET";

// Time / Date
/// Decrements time by hours. Parameters: -.
pub const CLOCK_HOURS_DEC: &str = "CLOCK_HOURS_DEC";
/// Increments time by hours. Parameters: -.
pub const CLOCK_HOURS_INC: &str = "CLOCK_HOURS_INC";
/// Sets hour of day. Parameters: \[0\] Value.
pub const CLOCK_HOURS_SET: &str = "CLOCK_HOURS_SET";
/// Decrements time by minutes. Parameters: -.
pub const CLOCK_MINUTES_DEC: &str = "CLOCK_MINUTES_DEC";
/// Increments time by minutes. Parameters: -.
pub const CLOCK_MINUTES_INC: &str = "CLOCK_MINUTES_INC";
/// Sets minutes of the hour. Parameters: \[0\] Value.
pub const CLOCK_MINUTES_SET: &str = "CLOCK_MINUTES_SET";
/// Zeros seconds. Parameters: -.
pub const CLOCK_SECONDS_ZERO: &str = "CLOCK_SECONDS_ZERO";
/// Sets day, in zulu time. Parameters: \[0\] Value.
pub const ZULU_DAY_SET: &str = "ZULU_DAY_SET";
/// Sets hours, zulu time. Parameters: \[0\] Value.
pub const ZULU_HOURS_SET: &str = "ZULU_HOURS_SET";
/// Sets minutes, in zulu time. Parameters: \[0\] Value.
pub const ZULU_MINUTES_SET: &str = "ZULU_MINUTES_SET";
/// Sets year, in zulu time. Parameters: \[0\] Value.
pub const ZULU_YEAR_SET: &str = "ZULU_YEAR_SET";

// 3rd Party
/// For developers, opens the debug window to access your exclusive content.
#[cfg(feature = "sunrise")]
pub const _3RD_PARTY_WINDOW_OPEN_PRIMARY: &str = "3RD_PARTY_WINDOW_OPEN_PRIMARY";
/// For developers, opens debug windows to help testing your exclusive content.
#[cfg(feature = "sunrise")]
pub const _3RD_PARTY_WINDOW_OPEN_SECONDARY: &str = "3RD_PARTY_WINDOW_OPEN_SECONDARY";
/// While using the community content manager, move selection down.
#[cfg(feature = "sunrise")]
pub const _3RD_PARTY_WINDOW_MOVE_DOWN: &str = "3RD_PARTY_WINDOW_MOVE_DOWN";
/// While using the community content manager, move selection up.
#[cfg(feature = "sunrise")]
pub const _3RD_PARTY_WINDOW_MOVE_UP: &str = "3RD_PARTY_WINDOW_MOVE_UP";
/// While using the community content manager, validate selection.
#[cfg(feature = "sunrise")]
pub const _3RD_PARTY_WINDOW_VALIDATE: &str = "3RD_PARTY_WINDOW_VALIDATE";

// Multiplayer
/// Activates chat window
pub const MP_ACTIVATE_CHAT: &str = "MP_ACTIVATE_CHAT";
/// Start capturing audio from the users computer and transmitting it to all other players in
/// the multiplayer session.
pub const MP_BROADCAST_VOICE_CAPTURE_START: &str = "MP_BROADCAST_VOICE_CAPTURE_START";
/// Stop capturing broadcast audio.
pub const MP_BROADCAST_VOICE_CAPTURE_STOP: &str = "MP_BROADCAST_VOICE_CAPTURE_STOP";
/// Toggles chat window visible/invisible
pub const MP_CHAT: &str = "MP_CHAT";
/// Pause the multiplayer session.
pub const MP_PAUSE_SESSION: &str = "MP_PAUSE_SESSION";
/// Cycle through the current user aircraft.
pub const MP_PLAYER_CYCLE: &str = "MP_PLAYER_CYCLE";
/// Set the view to follow the selected user aircraft.
pub const MP_PLAYER_FOLLOW: &str = "MP_PLAYER_FOLLOW";
/// Toggle to the next player to track
pub const MP_TRANSFER_CONTROL: &str = "MP_TRANSFER_CONTROL";
/// Start capturing audio from the users computer and transmitting it to all other players in
/// the multiplayer session who are turned to the same radio frequency.
pub const MP_VOICE_CAPTURE_START: &str = "MP_VOICE_CAPTURE_START";
/// Stop capturing radio audio.
pub const MP_VOICE_CAPTURE_STOP: &str = "MP_VOICE_CAPTURE_STOP";
/// Show or hide multi-player race results.
pub const TOGGLE_RACERESULTS_WINDOW: &str = "TOGGLE_RACERESULTS_WINDOW";
