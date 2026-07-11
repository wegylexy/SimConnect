// Burner System
/// Set the pitch of the given burner. Not currently used in the simulation. Parameters:
/// \[0\]: Burner name (or index) \[1\]: pitch value.
#[cfg(feature = "sunrise")]
pub const AXIS_BURNER_PITCH_SET: &str = "AXIS_BURNER_PITCH_SET";
/// Set the roll of the given burner. Not currently used in the simulation. Parameters: \[0\]:
/// Burner name (or index) \[1\]: roll value.
#[cfg(feature = "sunrise")]
pub const AXIS_BURNER_ROLL_SET: &str = "AXIS_BURNER_ROLL_SET";
/// Decrease the given burner pitch. Not currently used in the simulation. Parameters: \[0\]:
/// Burner name (or index).
#[cfg(feature = "sunrise")]
pub const BURNER_PITCH_DEC: &str = "BURNER_PITCH_DEC";
/// Increase the given burner pitch. Not currently used in the simulation. Parameters: \[0\]:
/// Burner name (or index).
#[cfg(feature = "sunrise")]
pub const BURNER_PITCH_INC: &str = "BURNER_PITCH_INC";
/// Decrease the given burner roll. Not currently used in the simulation. Parameters: \[0\]:
/// Burner name (or index).
#[cfg(feature = "sunrise")]
pub const BURNER_ROLL_DEC: &str = "BURNER_ROLL_DEC";
/// Increase the given burner roll. Not currently used in the simulation. Parameters: \[0\]:
/// Burner name (or index).
#[cfg(feature = "sunrise")]
pub const BURNER_ROLL_INC: &str = "BURNER_ROLL_INC";
/// Close the given burner valve. Parameters: \[0\]: Valve name (or index).
#[cfg(feature = "sunrise")]
pub const BURNER_VALVE_CLOSE: &str = "BURNER_VALVE_CLOSE";
/// Open the given burner valve. Parameters: \[0\]: Valve name (or index).
#[cfg(feature = "sunrise")]
pub const BURNER_VALVE_OPEN: &str = "BURNER_VALVE_OPEN";
/// Set the given burner valve to the given position, where 0 is fully closed and 16384 is
/// fully open. Parameters: \[0\]: Valve name (or index) \[1\]: Valve target position
/// (Position 16k).
#[cfg(feature = "sunrise")]
pub const BURNER_VALVE_SET: &str = "BURNER_VALVE_SET";
/// This will toggle the given burner valve open/closed. Parameters: \[0\]: Valve name (or
/// index).
#[cfg(feature = "sunrise")]
pub const BURNER_VALVE_TOGGLE: &str = "BURNER_VALVE_TOGGLE";

// Envelope
/// This will close the balloon envelope vent.
#[cfg(feature = "sunrise")]
pub const BALLOON_VENT_CLOSE: &str = "BALLOON_VENT_CLOSE";
/// This will open the balloon envelope vent.
#[cfg(feature = "sunrise")]
pub const BALLOON_VENT_OPEN: &str = "BALLOON_VENT_OPEN";
/// This will set the balloon envelope vent to the given position, where 0 is fully closed and
/// 16384 is fully open. Parameters: \[0\]: Vent target position (Position 16k).
#[cfg(feature = "sunrise")]
pub const BALLOON_VENT_SET: &str = "BALLOON_VENT_SET";
/// This will toggle the balloon envelope vent between open and closed.
#[cfg(feature = "sunrise")]
pub const BALLOON_VENT_TOGGLE: &str = "BALLOON_VENT_TOGGLE";

// Airship
/// Sets valve 1/2/3/4 to the "Locked" state.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_1_CLOSE: &str = "AIRSHIP_VALVE_1_CLOSE";
/// Sets valve 1/2/3/4 to the "Locked" state.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_2_CLOSE: &str = "AIRSHIP_VALVE_2_CLOSE";
/// Sets valve 1/2/3/4 to the "Locked" state.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_3_CLOSE: &str = "AIRSHIP_VALVE_3_CLOSE";
/// Sets valve 1/2/3/4 to the "Locked" state.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_4_CLOSE: &str = "AIRSHIP_VALVE_4_CLOSE";
/// Sets valve 1/2/3/4 to the "Unlocked" state. Parameters: AIRSHIP_VALVE_1_OPEN
/// AIRSHIP_VALVE_2_OPEN AIRSHIP_VALVE_3_OPEN AIRSHIP_VALVE_4_OPEN.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_1_OPEN: &str = "AIRSHIP_VALVE_1_OPEN";
/// Sets valve 1/2/3/4 to the "Unlocked" state. Parameters: AIRSHIP_VALVE_1_OPEN
/// AIRSHIP_VALVE_2_OPEN AIRSHIP_VALVE_3_OPEN AIRSHIP_VALVE_4_OPEN.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_2_OPEN: &str = "AIRSHIP_VALVE_2_OPEN";
/// Sets valve 1/2/3/4 to the "Unlocked" state. Parameters: AIRSHIP_VALVE_1_OPEN
/// AIRSHIP_VALVE_2_OPEN AIRSHIP_VALVE_3_OPEN AIRSHIP_VALVE_4_OPEN.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_3_OPEN: &str = "AIRSHIP_VALVE_3_OPEN";
/// Sets valve 1/2/3/4 to the "Unlocked" state. Parameters: AIRSHIP_VALVE_1_OPEN
/// AIRSHIP_VALVE_2_OPEN AIRSHIP_VALVE_3_OPEN AIRSHIP_VALVE_4_OPEN.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_4_OPEN: &str = "AIRSHIP_VALVE_4_OPEN";
/// Set valve 1/2/3/4 to one of the available states: "Unlocked" (this is "Auto" in the UI and
/// means the valve works as a Relief valve) "Locked" (this is simply closed) "Force open"
/// (this is is just open without pressure settings being taken in account) Parameters: \[0\]
/// setting.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_1_SET: &str = "AIRSHIP_VALVE_1_SET";
/// Set valve 1/2/3/4 to one of the available states: "Unlocked" (this is "Auto" in the UI and
/// means the valve works as a Relief valve) "Locked" (this is simply closed) "Force open"
/// (this is is just open without pressure settings being taken in account) Parameters: \[0\]
/// setting.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_2_SET: &str = "AIRSHIP_VALVE_2_SET";
/// Set valve 1/2/3/4 to one of the available states: "Unlocked" (this is "Auto" in the UI and
/// means the valve works as a Relief valve) "Locked" (this is simply closed) "Force open"
/// (this is is just open without pressure settings being taken in account) Parameters: \[0\]
/// setting.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_3_SET: &str = "AIRSHIP_VALVE_3_SET";
/// Set valve 1/2/3/4 to one of the available states: "Unlocked" (this is "Auto" in the UI and
/// means the valve works as a Relief valve) "Locked" (this is simply closed) "Force open"
/// (this is is just open without pressure settings being taken in account) Parameters: \[0\]
/// setting.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_4_SET: &str = "AIRSHIP_VALVE_4_SET";
/// Toggles valve 1/2/3/4 between the "Locked" and "Unlocked" states. Parameters:
/// KEY_AIRSHIP_VALVE_1_TOGGLE KEY_AIRSHIP_VALVE_2_TOGGLE AIRSHIP_VALVE_3_TOGGKEY_LE
/// KEY_AIRSHIP_VALVE_4_TOGGLE.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_1_TOGGLE: &str = "AIRSHIP_VALVE_1_TOGGLE";
/// Toggles valve 1/2/3/4 between the "Locked" and "Unlocked" states. Parameters:
/// KEY_AIRSHIP_VALVE_1_TOGGLE KEY_AIRSHIP_VALVE_2_TOGGLE AIRSHIP_VALVE_3_TOGGKEY_LE
/// KEY_AIRSHIP_VALVE_4_TOGGLE.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_2_TOGGLE: &str = "AIRSHIP_VALVE_2_TOGGLE";
/// Toggles valve 1/2/3/4 between the "Locked" and "Unlocked" states. Parameters:
/// KEY_AIRSHIP_VALVE_1_TOGGLE KEY_AIRSHIP_VALVE_2_TOGGLE AIRSHIP_VALVE_3_TOGGKEY_LE
/// KEY_AIRSHIP_VALVE_4_TOGGLE.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_3_TOGGLE: &str = "AIRSHIP_VALVE_3_TOGGLE";
/// Toggles valve 1/2/3/4 between the "Locked" and "Unlocked" states. Parameters:
/// KEY_AIRSHIP_VALVE_1_TOGGLE KEY_AIRSHIP_VALVE_2_TOGGLE AIRSHIP_VALVE_3_TOGGKEY_LE
/// KEY_AIRSHIP_VALVE_4_TOGGLE.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_4_TOGGLE: &str = "AIRSHIP_VALVE_4_TOGGLE";
/// Sets the named (or indexed) valve to the "Locked" state. Parameters: \[0\] Valve name (or
/// index).
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_CLOSE: &str = "AIRSHIP_VALVE_CLOSE";
/// Sets the named (or indexed) valve to the "Unlocked" state. Parameters: \[0\] Valve name
/// (or index).
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_OPEN: &str = "AIRSHIP_VALVE_OPEN";
/// Set the named (or indexed) valve to one of the available states: "Unlocked" (this is
/// "Auto" in the UI and means the valve works as a relief valve) "Locked" (this is simply
/// closed) "Force open" (this is is just open without pressure settings being taken in
/// account) Parameters: \[0\] Valve name (or index) \[1\] setting.
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_SET: &str = "AIRSHIP_VALVE_SET";
/// Toggles the named (or indexed) valve between the "Locked" and "Unlocked" states.
/// Parameters: \[0\] Valve name (or index).
#[cfg(feature = "sunrise")]
pub const AIRSHIP_VALVE_TOGGLE: &str = "AIRSHIP_VALVE_TOGGLE";
