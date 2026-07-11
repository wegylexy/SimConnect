// Axis
/// Sets the pitch of the camera axis. Requires an angle. Parameters: \[0\] Value.
pub const AXIS_PAN_PITCH: &str = "AXIS_PAN_PITCH";
/// Sets the heading of the axis. Requires an angle. Parameters: \[0\] Value.
pub const AXIS_PAN_HEADING: &str = "AXIS_PAN_HEADING";
/// Sets the tilt of the axis. Requires an angle. Parameters: \[0\] Value.
pub const AXIS_PAN_TILT: &str = "AXIS_PAN_TILT";
/// Set the view axis to one of the following enum values: 0 - AXIS_CYCL_OFF 1 -
/// AXIS_CYCL_4_DOTS 2 - AXIS_CYCL_SMALL_V 3 - AXIS_CYCL_LARGE_V Parameters: \[0\] Value.
pub const AXIS_IND_SET: &str = "AXIS_IND_SET";
/// Step through the view axis. Parameters: -.
pub const VIEW_AXIS_INDICATOR_CYCLE: &str = "VIEW_AXIS_INDICATOR_CYCLE";

// Chase Camera
/// Deprecated, do not use. Parameters: -.
pub const CHASE_VIEW: &str = "CHASE_VIEW";
/// Deprecated, do not use. Parameters: -.
pub const CHASE_VIEW_NEXT: &str = "CHASE_VIEW_NEXT";
/// Deprecated, do not use. Parameters: -.
pub const CHASE_VIEW_PREV: &str = "CHASE_VIEW_PREV";
/// Toggles chase view on/off
pub const CHASE_VIEW_TOGGLE: &str = "CHASE_VIEW_TOGGLE";
/// Increments the distance of the view camera from the chase object (such as in Spot Plane
/// view, or viewing an AI controlled aircraft).
pub const VIEW_CHASE_DISTANCE_ADD: &str = "VIEW_CHASE_DISTANCE_ADD";
/// Decrements the distance of the view camera from the chase object.
pub const VIEW_CHASE_DISTANCE_SUB: &str = "VIEW_CHASE_DISTANCE_SUB";

// Eyepoint
/// Move eyepoint backward Parameters: -.
pub const EYEPOINT_BACK: &str = "EYEPOINT_BACK";
/// Move eyepoint down Parameters: -.
pub const EYEPOINT_DOWN: &str = "EYEPOINT_DOWN";
/// Move eyepoint forward Parameters: -.
pub const EYEPOINT_FORWARD: &str = "EYEPOINT_FORWARD";
/// Move eyepoint left Parameters: -.
pub const EYEPOINT_LEFT: &str = "EYEPOINT_LEFT";
/// Move eyepoint to default position Parameters: -.
pub const EYEPOINT_RESET: &str = "EYEPOINT_RESET";
/// Move eyepoint right Parameters: -.
pub const EYEPOINT_RIGHT: &str = "EYEPOINT_RIGHT";
/// Move eyepoint up Parameters: -.
pub const EYEPOINT_UP: &str = "EYEPOINT_UP";

// Miscellaneous
/// Steps through the different HUD colours: 0 - HUD_GREEN 1 - HUD_DK_GREEN 2 - HUD_BLUE 3 -
/// HUD_DK_BLUE 4 - HUD_RED 5 - HUD_DK_RED 6 - HUD_BLACK 7 - HUD_WHITE Parameters: -.
pub const HUD_COLOR: &str = "HUD_COLOR";
/// Not currently used in the simulation. Parameters: -.
pub const HUD_UNITS: &str = "HUD_UNITS";
/// Not currently used in the simulation. Parameters: -.
pub const LETTERBOX: &str = "LETTERBOX";
/// Step through the map orientations. Parameters: -.
pub const MAP_ORIENTATION_CYCLE: &str = "MAP_ORIENTATION_CYCLE";
/// Set the map orientation to one of the following enum values: 0 - NORTH_ORIENTED 1 -
/// AC_ORIENTED 2 - NORTH_HIGH_ALT Parameters: \[0\] Value.
pub const MAP_ORIENTATION_SET: &str = "MAP_ORIENTATION_SET";
/// Fine zoom in map view. Parameters: -.
pub const MAP_ZOOM_FINE_IN: &str = "MAP_ZOOM_FINE_IN";
/// Fine zoom out in map view. Parameters: -.
pub const MAP_ZOOM_FINE_OUT: &str = "MAP_ZOOM_FINE_OUT";
/// Set the view zoom level. Parameters: \[0\] Value.
pub const MAP_ZOOM_SET: &str = "MAP_ZOOM_SET";
/// Opens new map view. Parameters: -.
pub const NEW_MAP: &str = "NEW_MAP";
/// Not currently used in the simulation. Parameters: -.
pub const OTHER_AIRCRAFT_VIEW: &str = "OTHER_AIRCRAFT_VIEW";
/// Move to the next HUD panel. Parameters: -.
pub const PANEL_HUD_NEXT: &str = "PANEL_HUD_NEXT";
/// Move to the previous HUD panel. Parameters: -.
pub const PANEL_HUD_PREVIOUS: &str = "PANEL_HUD_PREVIOUS";
/// Not currently used in the simulation. Parameters: -.
pub const SELECT_NEXT_TARGET: &str = "SELECT_NEXT_TARGET";
/// Not currently used in the simulation. Parameters: -.
pub const SELECT_PREV_TARGET: &str = "SELECT_PREV_TARGET";
/// Not currently used in the simulation. Parameters: -.
pub const SKIP_ACTION: &str = "SKIP_ACTION";
/// Not currently used in the simulation. Parameters: -.
pub const SP_MULTIPLAYER_SCORE_DISPLAY: &str = "SP_MULTIPLAYER_SCORE_DISPLAY";
/// Deprecated, do not use. Parameters: -.
pub const TOGGLE_PADLOCK: &str = "TOGGLE_PADLOCK";
/// (no description provided by the vendor docs) Parameters: -.
pub const UNLOCK_TARGET: &str = "UNLOCK_TARGET";

// Views
/// Close current view. Parameters: -.
pub const CLOSE_VIEW: &str = "CLOSE_VIEW";
/// Next view in the category. Parameters: -.
pub const NEXT_SUB_VIEW: &str = "NEXT_SUB_VIEW";
/// Select next view. Parameters: -.
pub const NEXT_VIEW: &str = "NEXT_VIEW";
/// Open new view. Parameters: -.
pub const NEW_VIEW: &str = "NEW_VIEW";
/// Select previous view. Parameters: -.
pub const PREV_VIEW: &str = "PREV_VIEW";
/// Previous view in the category. Parameters: -.
pub const PREV_SUB_VIEW: &str = "PREV_SUB_VIEW";
/// Snap the view using one of the following enum values: 22 - VIEW_DIR_FORWARD 67 -
/// VIEW_DIR_FORWARD_RIGHT 112 - VIEW_DIR_RIGHT 155 - VIEW_DIR_REAR_RIGHT 202 - VIEW_DIR_REAR
/// 247 - VIEW_DIR_REAR_LEFT 292 - VIEW_DIR_LEFT 337 - VIEW_DIR_FORWARD_LEFT Parameters: \[0\]
/// Value.
pub const SNAP_VIEW: &str = "SNAP_VIEW";
/// Select View Direction. Parameters: -.
pub const VIEW: &str = "VIEW";
/// (no description provided by the vendor docs) Parameters: -.
pub const VIEW_ALWAYS_PAN_DOWN: &str = "VIEW_ALWAYS_PAN_DOWN";
/// (no description provided by the vendor docs) Parameters: -.
pub const VIEW_ALWAYS_PAN_UP: &str = "VIEW_ALWAYS_PAN_UP";
/// Select the one of the 6 auxiliary camera views. Parameters: -.
pub const VIEW_AUX_00: &str = "VIEW_AUX_00";
/// Select the one of the 6 auxiliary camera views. Parameters: -.
pub const VIEW_AUX_01: &str = "VIEW_AUX_01";
/// Select the one of the 6 auxiliary camera views. Parameters: -.
pub const VIEW_AUX_02: &str = "VIEW_AUX_02";
/// Select the one of the 6 auxiliary camera views. Parameters: -.
pub const VIEW_AUX_03: &str = "VIEW_AUX_03";
/// Select the one of the 6 auxiliary camera views. Parameters: -.
pub const VIEW_AUX_04: &str = "VIEW_AUX_04";
/// Select the one of the 6 auxiliary camera views. Parameters: -.
pub const VIEW_AUX_05: &str = "VIEW_AUX_05";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_0: &str = "VIEW_CAMERA_SELECT_0";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_1: &str = "VIEW_CAMERA_SELECT_1";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_2: &str = "VIEW_CAMERA_SELECT_2";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_3: &str = "VIEW_CAMERA_SELECT_3";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_4: &str = "VIEW_CAMERA_SELECT_4";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_5: &str = "VIEW_CAMERA_SELECT_5";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_6: &str = "VIEW_CAMERA_SELECT_6";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_7: &str = "VIEW_CAMERA_SELECT_7";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_8: &str = "VIEW_CAMERA_SELECT_8";
/// Select the one of the 10 camera views. Parameters: -.
pub const VIEW_CAMERA_SELECT_9: &str = "VIEW_CAMERA_SELECT_9";
/// Select the starting view set for the camera. Parameters: -.
pub const VIEW_CAMERA_SELECT_START: &str = "VIEW_CAMERA_SELECT_START";
/// Switch immediately to the forward view, in 2D mode. Parameters: -.
pub const VIEW_COCKPIT_FORWARD: &str = "VIEW_COCKPIT_FORWARD";
/// Deprecated, do not use. Parameters: -.
pub const VIEW_DIRECTION_SET: &str = "VIEW_DIRECTION_SET";
/// Set the view direction using one of the following enum values: 0 - VIEW_DIR_FORWARD 1 -
/// VIEW_DIR_RIGHT 2 - VIEW_DIR_REAR 3 - VIEW_DIR_LEFT Parameters: \[0\] Value.
pub const VIEW1_DIRECTION_SET: &str = "VIEW1_DIRECTION_SET";
/// Set the view direction using one of the following enum values: 0 - VIEW_DIR_FORWARD 1 -
/// VIEW_DIR_RIGHT 2 - VIEW_DIR_REAR 3 - VIEW_DIR_LEFT Parameters: \[0\] Value.
pub const VIEW2_DIRECTION_SET: &str = "VIEW2_DIRECTION_SET";
/// Sets view direction down. Parameters: -.
pub const VIEW_DOWN: &str = "VIEW_DOWN";
/// Sets view direction forward. Parameters: -.
pub const VIEW_FORWARD: &str = "VIEW_FORWARD";
/// Sets view direction forward and left. Parameters: -.
pub const VIEW_FORWARD_LEFT: &str = "VIEW_FORWARD_LEFT";
/// Sets view direction forward and right. Parameters: -.
pub const VIEW_FORWARD_RIGHT: &str = "VIEW_FORWARD_RIGHT";
/// Sets view forward, right, and up. Parameters: -.
pub const VIEW_FORWARD_RIGHT_UP: &str = "VIEW_FORWARD_RIGHT_UP";
/// Sets view forward and up. Parameters: -.
pub const VIEW_FORWARD_UP: &str = "VIEW_FORWARD_UP";
/// Sets view forward left and up. Parameters: -.
pub const VIEW_FORWARD_LEFT_UP: &str = "VIEW_FORWARD_LEFT_UP";
/// Sets view direction to the left. Parameters: -.
pub const VIEW_LEFT: &str = "VIEW_LEFT";
/// Sets view left and up. Parameters: -.
pub const VIEW_LEFT_UP: &str = "VIEW_LEFT_UP";
/// Links all the views from one camera together, so that panning the view will change the
/// view of all the linked cameras. Parameters: \[0\] Bool.
pub const VIEW_LINKING_SET: &str = "VIEW_LINKING_SET";
/// Turns view linking on or off. Parameters: -.
pub const VIEW_LINKING_TOGGLE: &str = "VIEW_LINKING_TOGGLE";
/// Selects next view category. Parameters: -.
pub const VIEW_MODE: &str = "VIEW_MODE";
/// Reverse view cycle. Parameters: -.
pub const VIEW_MODE_REV: &str = "VIEW_MODE_REV";
/// Decrement alpha-blending for the panel. Parameters: -.
pub const VIEW_PANEL_ALPHA_DEC: &str = "VIEW_PANEL_ALPHA_DEC";
/// Increment alpha-blending for the panel. Parameters: -.
pub const VIEW_PANEL_ALPHA_INC: &str = "VIEW_PANEL_ALPHA_INC";
/// Sets the mode to change the alpha-blending, so the keys PLUS and MINUS increment and
/// decrement the value. Parameters: -.
pub const VIEW_PANEL_ALPHA_SELECT: &str = "VIEW_PANEL_ALPHA_SELECT";
/// Sets the alpha-blending value for the panel. Takes a parameter in the range 0to 255. The
/// alpha-blending can be changed from the keyboard using Ctrl-Shift-T,and the plus and minus
/// keys. Parameters: \[0\] Value.
pub const VIEW_PANEL_ALPHA_SET: &str = "VIEW_PANEL_ALPHA_SET";
/// Toggle between the current view and the previous view. Parameters: -.
pub const VIEW_PREVIOUS_TOGGLE: &str = "VIEW_PREVIOUS_TOGGLE";
/// Sets view direction to the rear. Parameters: -.
pub const VIEW_REAR: &str = "VIEW_REAR";
/// Sets view direction to the rear and left. Parameters: -.
pub const VIEW_REAR_LEFT: &str = "VIEW_REAR_LEFT";
/// Sets view rear left and up. Parameters: -.
pub const VIEW_REAR_LEFT_UP: &str = "VIEW_REAR_LEFT_UP";
/// Sets view direction to the rear and right. Parameters: -.
pub const VIEW_REAR_RIGHT: &str = "VIEW_REAR_RIGHT";
/// Sets view rear, right, and up. Parameters: -.
pub const VIEW_REAR_RIGHT_UP: &str = "VIEW_REAR_RIGHT_UP";
/// Sets view rear and up. Parameters: -.
pub const VIEW_REAR_UP: &str = "VIEW_REAR_UP";
/// Resets the view to the default. Parameters: -.
pub const VIEW_RESET: &str = "VIEW_RESET";
/// Sets view direction to the right. Parameters: -.
pub const VIEW_RIGHT: &str = "VIEW_RIGHT";
/// Sets view right and up. Parameters: -.
pub const VIEW_RIGHT_UP: &str = "VIEW_RIGHT_UP";
/// (no description provided by the vendor docs) Parameters: -.
pub const VIEW_SNAP_PANEL: &str = "VIEW_SNAP_PANEL";
/// Switch immediately to the forward view, in virtual cockpit mode (this is an alias for
/// VIEW_VIRTUAL_COCKPIT_FORWARD). Parameters: -.
pub const VIEW_SNAP_PANEL_RESET: &str = "VIEW_SNAP_PANEL_RESET";
/// (no description provided by the vendor docs) Parameters: -.
pub const VIEW_TRACK_PAN_TOGGLE: &str = "VIEW_TRACK_PAN_TOGGLE";
/// Deprecated, do not use. Parameters: -.
pub const VIEW_TYPE: &str = "VIEW_TYPE";
/// Deprecated, do not use. Parameters: -.
pub const VIEW_TYPE_REV: &str = "VIEW_TYPE_REV";
/// Sets view up. Parameters: -.
pub const VIEW_UP: &str = "VIEW_UP";
/// Switch immediately to the forward view, in virtual cockpit mode. Parameters: -.
pub const VIEW_VIRTUAL_COCKPIT_FORWARD: &str = "VIEW_VIRTUAL_COCKPIT_FORWARD";
/// Sets active window to front. Parameters: -.
pub const VIEW_WINDOW_TO_FRONT: &str = "VIEW_WINDOW_TO_FRONT";
/// Sets the active view mode, one fo the following enum values: 1 = VIEW_MODE_COCKPIT 2 =
/// VIEW_MODE_VIRTUAL_COCKPIT 3 = VIEW_MODE_TOWER 4 = VIEW_MODE_SPOT 5 = VIEW_MODE_MAP 6 =
/// VIEW_MODE_TRACK Parameters: \[0\] Value.
pub const VIEW1_MODE_SET: &str = "VIEW1_MODE_SET";
/// Sets the active view mode, one fo the following enum values: 1 = VIEW_MODE_COCKPIT 2 =
/// VIEW_MODE_VIRTUAL_COCKPIT 3 = VIEW_MODE_TOWER 4 = VIEW_MODE_SPOT 5 = VIEW_MODE_MAP 6 =
/// VIEW_MODE_TRACK Parameters: \[0\] Value.
pub const VIEW2_MODE_SET: &str = "VIEW2_MODE_SET";
/// Set the view 1/2 zoom value. Parameters: \[0\] Value.
pub const VIEW1_ZOOM_SET: &str = "VIEW1_ZOOM_SET";
/// Set the view 1/2 zoom value. Parameters: \[0\] Value.
pub const VIEW2_ZOOM_SET: &str = "VIEW2_ZOOM_SET";

// Zoom and Pan
/// Pan view down. Parameters: -.
pub const PAN_DOWN: &str = "PAN_DOWN";
/// Pans view left. Parameters: -.
pub const PAN_LEFT: &str = "PAN_LEFT";
/// Pan view left and down. Parameters: -.
pub const PAN_LEFT_DOWN: &str = "PAN_LEFT_DOWN";
/// Pan view left. Parameters: -.
pub const PAN_LEFT_UP: &str = "PAN_LEFT_UP";
/// Reset panning to forward (this is an alias for PAN_RESET_COCKPIT). Parameters: -.
pub const PAN_RESET: &str = "PAN_RESET";
/// Reset panning to forward (this is an alias for PAN_RESET). Parameters: -.
pub const PAN_RESET_COCKPIT: &str = "PAN_RESET_COCKPIT";
/// Pans view right. Parameters: -.
pub const PAN_RIGHT: &str = "PAN_RIGHT";
/// Pan view right and down. Parameters: -.
pub const PAN_RIGHT_DOWN: &str = "PAN_RIGHT_DOWN";
/// Pan view right and up. Parameters: -.
pub const PAN_RIGHT_UP: &str = "PAN_RIGHT_UP";
/// Tilt view left. Parameters: -.
pub const PAN_TILT_LEFT: &str = "PAN_TILT_LEFT";
/// Tilt view right. Parameters: -.
pub const PAN_TILT_RIGHT: &str = "PAN_TILT_RIGHT";
/// Pan view up. Parameters: -.
pub const PAN_UP: &str = "PAN_UP";
/// Pan the view in a specific way using the following enum: -1 - HEADING_RESET 0 - HEADING_UP
/// 45 - HEADING_RIGHT_UP 90 - HEADING_RIGHT 135 - HEADING_RIGHT_DOWN 180 - HEADING_DOWN 225 -
/// HEADING_LEFT_DOWN 270 - HEADING_LEFT 315 - HEADING_LEFT_UP Parameters: \[0\] Value.
pub const PAN_VIEW: &str = "PAN_VIEW";
/// Internal debug only. Parameters: ZOOM_1X -.
pub const ZOOM_1X: &str = "ZOOM_1X";
/// Zooms view OUT (is an alias for ZOOM_MINUS). Parameters: -.
pub const ZOOM_IN: &str = "ZOOM_IN";
/// Zoom in fine (is an alias for MAP_ZOOM_FINE_IN). Parameters: -.
pub const ZOOM_IN_FINE: &str = "ZOOM_IN_FINE";
/// Decreases zoom. Parameters: -.
pub const ZOOM_MINUS: &str = "ZOOM_MINUS";
/// Zooms view IN (is an alias for ZOOM_PLUS) Parameters: -.
pub const ZOOM_OUT: &str = "ZOOM_OUT";
/// Zoom out fine (is an alias for MAP_ZOOM_FINE_OUT). Parameters: -.
pub const ZOOM_OUT_FINE: &str = "ZOOM_OUT_FINE";
/// Increase zoom. Parameters: -.
pub const ZOOM_PLUS: &str = "ZOOM_PLUS";
