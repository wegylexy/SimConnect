// ADF
/// Sequentially selects the ADF tuner digits for use with +/-. Follow by SELECT_1 for ADF 1,
/// or SELECT_2 for ADF 2.
pub const ADF: &str = "ADF";
/// Decrements the ADF card by 10° if the key is pressed more than 2 seconds, 4° if the key is
/// pressed more than 1 second, or by 1° otherwise. The resulting value is clamped between 0°
/// and 360°.
pub const ADF_CARD_DEC: &str = "ADF_CARD_DEC";
/// Increments the ADF card by 10° if the key is pressed more than 2 seconds, 4° if the key is
/// pressed more than 1 second, or by 1° otherwise. The resulting value is clamped between 0°
/// and 360°.
pub const ADF_CARD_INC: &str = "ADF_CARD_INC";
/// Sets the ADF card. The resulting value is clamped between 0° and 360°. Parameters: \[0\]:
/// Card value.
pub const ADF_CARD_SET: &str = "ADF_CARD_SET";
/// Decrements the ADF 1 / 2 frequency by 1 KHz, with wrapping.
pub const ADF_1_DEC: &str = "ADF_1_DEC";
/// Decrements the ADF 1 / 2 frequency by 1 KHz, with wrapping.
pub const ADF2_1_DEC: &str = "ADF2_1_DEC";
/// Decrements the ADF 1 / 2 frequency by 10 KHz, with wrapping.
pub const ADF_10_DEC: &str = "ADF_10_DEC";
/// Decrements the ADF 1 / 2 frequency by 10 KHz, with wrapping.
pub const ADF2_10_DEC: &str = "ADF2_10_DEC";
/// Decrements the ADF 1 / 2 frequency by 100 KHz, with wrapping.
pub const ADF_100_DEC: &str = "ADF_100_DEC";
/// Decrements the ADF 1 / 2 frequency by 100 KHz, with wrapping.
pub const ADF2_100_DEC: &str = "ADF2_100_DEC";
/// Increments the ADF 1 / 2 frequency by 1 KHz, with wrapping.
pub const ADF_1_INC: &str = "ADF_1_INC";
/// Increments the ADF 1 / 2 frequency by 1 KHz, with wrapping.
pub const ADF2_1_INC: &str = "ADF2_1_INC";
/// Increments the ADF 1 / 2 frequency by 10 KHz, with wrapping.
pub const ADF_10_INC: &str = "ADF_10_INC";
/// Increments the ADF 1 / 2 frequency by 10 KHz, with wrapping.
pub const ADF2_10_INC: &str = "ADF2_10_INC";
/// Increments the ADF 1 / 2 frequency by 100 KHz, with wrapping.
pub const ADF_100_INC: &str = "ADF_100_INC";
/// Increments the ADF 1 / 2 frequency by 100 KHz, with wrapping.
pub const ADF2_100_INC: &str = "ADF2_100_INC";
/// Sets the ADF 1 / 2 active frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]:
/// Frequency value (Frequency BCD32 encoded Hz).
pub const ADF_ACTIVE_SET: &str = "ADF_ACTIVE_SET";
/// Sets the ADF 1 / 2 active frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]:
/// Frequency value (Frequency BCD32 encoded Hz).
pub const ADF2_ACTIVE_SET: &str = "ADF2_ACTIVE_SET";
/// Sets the ADF 1 / 2 frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]: Frequency
/// value (Frequency BCD32 encoded Hz).
pub const ADF_COMPLETE_SET: &str = "ADF_COMPLETE_SET";
/// Sets the ADF 1 / 2 frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]: Frequency
/// value (Frequency BCD32 encoded Hz).
pub const ADF2_COMPLETE_SET: &str = "ADF2_COMPLETE_SET";
/// Sets the ADF 1 / 2 frequency (thousands and tenths, Frequency BCD32 encoded HZ).
/// Parameters: \[0\]: Frequency value (Frequency BCD32 encoded Hz).
pub const ADF_EXTENDED_SET: &str = "ADF_EXTENDED_SET";
/// Sets the ADF 1 / 2 frequency (thousands and tenths, Frequency BCD32 encoded HZ).
/// Parameters: \[0\]: Frequency value (Frequency BCD32 encoded Hz).
pub const ADF2_EXTENDED_SET: &str = "ADF2_EXTENDED_SET";
/// Decrements the ADF 1 / 2 frequency by 0.1 KHz, with carry.
pub const ADF_FRACT_DEC_CARRY: &str = "ADF_FRACT_DEC_CARRY";
/// Decrements the ADF 1 / 2 frequency by 0.1 KHz, with carry.
pub const ADF2_FRACT_DEC_CARRY: &str = "ADF2_FRACT_DEC_CARRY";
/// Increments the ADF 1 / 2 frequency by 0.1 KHz, with carry.
pub const ADF_FRACT_INC_CARRY: &str = "ADF_FRACT_INC_CARRY";
/// Increments the ADF 1 / 2 frequency by 0.1 KHz, with carry.
pub const ADF2_FRACT_INC_CARRY: &str = "ADF2_FRACT_INC_CARRY";
/// Sets the ADF 1 / 2 highrange frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]:
/// Frequency value (Frequency BCD32 encoded Hz).
pub const ADF_HIGHRANGE_SET: &str = "ADF_HIGHRANGE_SET";
/// Sets the ADF 1 / 2 highrange frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]:
/// Frequency value (Frequency BCD32 encoded Hz).
pub const ADF2_HIGHRANGE_SET: &str = "ADF2_HIGHRANGE_SET";
/// Sets the ADF 1 / 2 lowrange frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]:
/// Frequency value (Frequency BCD32 encoded Hz).
pub const ADF_LOWRANGE_SET: &str = "ADF_LOWRANGE_SET";
/// Sets the ADF 1 / 2 lowrange frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]:
/// Frequency value (Frequency BCD32 encoded Hz).
pub const ADF2_LOWRANGE_SET: &str = "ADF2_LOWRANGE_SET";
/// Sets the ADF 1 / 2 needle value, in radians. Note that ADF_OUTSIDE_SOURCE /
/// ADF2_OUTSIDE_SOURCE must be enabled. Parameters: \[0\]: Needle value.
pub const ADF_NEEDLE_SET: &str = "ADF_NEEDLE_SET";
/// Sets the ADF 1 / 2 needle value, in radians. Note that ADF_OUTSIDE_SOURCE /
/// ADF2_OUTSIDE_SOURCE must be enabled. Parameters: \[0\]: Needle value.
pub const ADF2_NEEDLE_SET: &str = "ADF2_NEEDLE_SET";
/// When TRUE sets ADF 1 / 2 source to be outside, when FALSE it's not. This enables you to
/// use the ADF_NEEDLE_SET / ADF2_NEEDLE_SET events to set the ADF needle instead of relying
/// on the simulation source. Parameters: \[0\]: Bool.
pub const ADF_OUTSIDE_SOURCE: &str = "ADF_OUTSIDE_SOURCE";
/// When TRUE sets ADF 1 / 2 source to be outside, when FALSE it's not. This enables you to
/// use the ADF_NEEDLE_SET / ADF2_NEEDLE_SET events to set the ADF needle instead of relying
/// on the simulation source. Parameters: \[0\]: Bool.
pub const ADF2_OUTSIDE_SOURCE: &str = "ADF2_OUTSIDE_SOURCE";
/// Swaps between the ADF 1 / 2 frequency and the standby frequency.
pub const ADF1_RADIO_SWAP: &str = "ADF1_RADIO_SWAP";
/// Swaps between the ADF 1 / 2 frequency and the standby frequency.
pub const ADF2_RADIO_SWAP: &str = "ADF2_RADIO_SWAP";
/// Decrements the ADF 1 / 2 frequency by 0.1 KHz.
pub const ADF1_RADIO_TENTHS_DEC: &str = "ADF1_RADIO_TENTHS_DEC";
/// Decrements the ADF 1 / 2 frequency by 0.1 KHz.
pub const ADF2_RADIO_TENTHS_DEC: &str = "ADF2_RADIO_TENTHS_DEC";
/// Increments the ADF 1 / 2 frequency by 0.1 KHz.
pub const ADF1_RADIO_TENTHS_INC: &str = "ADF1_RADIO_TENTHS_INC";
/// Increments the ADF 1 / 2 frequency by 0.1 KHz.
pub const ADF2_RADIO_TENTHS_INC: &str = "ADF2_RADIO_TENTHS_INC";
/// Sets ADF 1 / 2 frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]: Frequency value.
pub const ADF_SET: &str = "ADF_SET";
/// Sets ADF 1 / 2 frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]: Frequency value.
pub const ADF2_SET: &str = "ADF2_SET";
/// Sets ADF 1 / 2 standby frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]:
/// Frequency value.
pub const ADF_STBY_SET: &str = "ADF_STBY_SET";
/// Sets ADF 1 / 2 standby frequency (Frequency BCD32 encoded Hz). Parameters: \[0\]:
/// Frequency value.
pub const ADF2_STBY_SET: &str = "ADF2_STBY_SET";
/// Increase ADF 1 / 2 volume by 0.02. The resulting value will be clamped between 0 and 1.
pub const ADF_VOLUME_INC: &str = "ADF_VOLUME_INC";
/// Increase ADF 1 / 2 volume by 0.02. The resulting value will be clamped between 0 and 1.
pub const ADF2_VOLUME_INC: &str = "ADF2_VOLUME_INC";
/// Decrease ADF 1 / 2 volume by 0.02. The resulting value will be clamped between 0 and 1.
pub const ADF_VOLUME_DEC: &str = "ADF_VOLUME_DEC";
/// Decrease ADF 1 / 2 volume by 0.02. The resulting value will be clamped between 0 and 1.
pub const ADF2_VOLUME_DEC: &str = "ADF2_VOLUME_DEC";
/// Sets ADF 1 / 2 volume (from 0 to 100). Parameters: \[0\]: Volume value.
pub const ADF_VOLUME_SET: &str = "ADF_VOLUME_SET";
/// Sets ADF 1 / 2 volume (from 0 to 100). Parameters: \[0\]: Volume value.
pub const ADF2_VOLUME_SET: &str = "ADF2_VOLUME_SET";
/// Decrements the ADF 1 / 2 frequency by 1 KHz, with carry as digits wrap.
pub const ADF1_WHOLE_DEC: &str = "ADF1_WHOLE_DEC";
/// Decrements the ADF 1 / 2 frequency by 1 KHz, with carry as digits wrap.
pub const ADF2_WHOLE_DEC: &str = "ADF2_WHOLE_DEC";
/// Increments the ADF 1 / 2 frequency by 1 KHz, with carry as digits wrap.
pub const ADF1_WHOLE_INC: &str = "ADF1_WHOLE_INC";
/// Increments the ADF 1 / 2 frequency by 1 KHz, with carry as digits wrap.
pub const ADF2_WHOLE_INC: &str = "ADF2_WHOLE_INC";
/// Turns the ADF 1 / 2 ID off.
pub const RADIO_ADF_IDENT_DISABLE: &str = "RADIO_ADF_IDENT_DISABLE";
/// Turns the ADF 1 / 2 ID off.
pub const RADIO_ADF2_IDENT_DISABLE: &str = "RADIO_ADF2_IDENT_DISABLE";
/// Turns the ADF 1 / 2 ID on.
pub const RADIO_ADF_IDENT_ENABLE: &str = "RADIO_ADF_IDENT_ENABLE";
/// Turns the ADF 1 / 2 ID on.
pub const RADIO_ADF2_IDENT_ENABLE: &str = "RADIO_ADF2_IDENT_ENABLE";
/// Sets the ADF 1 / 2 ID on (1) or off (0). Parameters: \[0\]: True/False (1, 0).
pub const RADIO_ADF_IDENT_SET: &str = "RADIO_ADF_IDENT_SET";
/// Sets the ADF 1 / 2 ID on (1) or off (0). Parameters: \[0\]: True/False (1, 0).
pub const RADIO_ADF2_IDENT_SET: &str = "RADIO_ADF2_IDENT_SET";
/// Toggles the ADF 1 / 2 ID between on (1) and off (0).
pub const RADIO_ADF_IDENT_TOGGLE: &str = "RADIO_ADF_IDENT_TOGGLE";
/// Toggles the ADF 1 / 2 ID between on (1) and off (0).
pub const RADIO_ADF2_IDENT_TOGGLE: &str = "RADIO_ADF2_IDENT_TOGGLE";

// Avionics
/// Sets the avionics master switch to on or off. Parameters: \[0\]: Bool.
pub const AVIONICS_MASTER_SET: &str = "AVIONICS_MASTER_SET";
/// Toggles the avionics master switch
pub const TOGGLE_AVIONICS_MASTER: &str = "TOGGLE_AVIONICS_MASTER";
/// Sets avionics master 1 / 2 switch to on (1).
pub const AVIONICS_MASTER_1_ON: &str = "AVIONICS_MASTER_1_ON";
/// Sets avionics master 1 / 2 switch to on (1).
pub const AVIONICS_MASTER_2_ON: &str = "AVIONICS_MASTER_2_ON";
/// Sets avionics master 1 / 2 switch to off (0).
pub const AVIONICS_MASTER_1_OFF: &str = "AVIONICS_MASTER_1_OFF";
/// Sets avionics master 1 / 2 switch to off (0).
pub const AVIONICS_MASTER_2_OFF: &str = "AVIONICS_MASTER_2_OFF";
/// Sets avionics master 1 / 2 switch to on (1) or off (0). Parameters: \[0\]: Bool.
pub const AVIONICS_MASTER_1_SET: &str = "AVIONICS_MASTER_1_SET";
/// Sets avionics master 1 / 2 switch to on (1) or off (0). Parameters: \[0\]: Bool.
pub const AVIONICS_MASTER_2_SET: &str = "AVIONICS_MASTER_2_SET";

// COM
/// Sequentially selects the COM tuner digits for use with +/-. Follow by SELECT_2 for COM 2
/// or SELECT_3 for COM 3.
pub const COM_RADIO: &str = "COM_RADIO";
/// Decrements COM 1/2/3 frequency by 25 KHz, with no carry when digit wraps
pub const COM_RADIO_FRACT_DEC: &str = "COM_RADIO_FRACT_DEC";
/// Decrements COM 1/2/3 frequency by 25 KHz, with no carry when digit wraps
pub const COM2_RADIO_FRACT_DEC: &str = "COM2_RADIO_FRACT_DEC";
/// Decrements COM 1/2/3 frequency by 25 KHz, with no carry when digit wraps
pub const COM3_RADIO_FRACT_DEC: &str = "COM3_RADIO_FRACT_DEC";
/// Decrement COM 1/2/3 frequency by 25 KHz, and carry when digit wraps
pub const COM_RADIO_FRACT_DEC_CARRY: &str = "COM_RADIO_FRACT_DEC_CARRY";
/// Decrement COM 1/2/3 frequency by 25 KHz, and carry when digit wraps
pub const COM2_RADIO_FRACT_DEC_CARRY: &str = "COM2_RADIO_FRACT_DEC_CARRY";
/// Decrement COM 1/2/3 frequency by 25 KHz, and carry when digit wraps
pub const COM3_RADIO_FRACT_DEC_CARRY: &str = "COM3_RADIO_FRACT_DEC_CARRY";
/// Increment COM 1/2/3 frequency by 25 KHz, with no carry when digit wraps
pub const COM_RADIO_FRACT_INC: &str = "COM_RADIO_FRACT_INC";
/// Increment COM 1/2/3 frequency by 25 KHz, with no carry when digit wraps
pub const COM2_RADIO_FRACT_INC: &str = "COM2_RADIO_FRACT_INC";
/// Increment COM 1/2/3 frequency by 25 KHz, with no carry when digit wraps
pub const COM3_RADIO_FRACT_INC: &str = "COM3_RADIO_FRACT_INC";
/// Increment COM 1/2/3 frequency by 25 KHz, and carry when digit wraps
pub const COM_RADIO_FRACT_INC_CARRY: &str = "COM_RADIO_FRACT_INC_CARRY";
/// Increment COM 1/2/3 frequency by 25 KHz, and carry when digit wraps
pub const COM2_RADIO_FRACT_INC_CARRY: &str = "COM2_RADIO_FRACT_INC_CARRY";
/// Increment COM 1/2/3 frequency by 25 KHz, and carry when digit wraps
pub const COM3_RADIO_FRACT_INC_CARRY: &str = "COM3_RADIO_FRACT_INC_CARRY";
/// COM1, legacy 25 kHz `FrequencyBcd16`-encoded frequency. Prefer `COM_RADIO_SET_HZ` for new
/// code -- it works for both 25 kHz and 8.33 kHz-spaced radios, see `crate::bcd`'s module
/// docs for why.
pub const COM_RADIO_SET: &str = "COM_RADIO_SET";
/// COM2, legacy 25 kHz `FrequencyBcd16`-encoded frequency. Prefer `COM2_RADIO_SET_HZ` for new
/// code.
pub const COM2_RADIO_SET: &str = "COM2_RADIO_SET";
/// COM3, legacy 25 kHz `FrequencyBcd16`-encoded frequency. Prefer `COM3_RADIO_SET_HZ` for new
/// code.
pub const COM3_RADIO_SET: &str = "COM3_RADIO_SET";
/// COM1, exact Hz -- works for both 25 kHz and 8.33 kHz-spaced radios.
pub const COM_RADIO_SET_HZ: &str = "COM_RADIO_SET_HZ";
/// COM2, exact Hz.
pub const COM2_RADIO_SET_HZ: &str = "COM2_RADIO_SET_HZ";
/// COM3, exact Hz.
pub const COM3_RADIO_SET_HZ: &str = "COM3_RADIO_SET_HZ";
/// Sets COM 1/2/3 standby frequency as a BCD16 encoded value. Parameters: \[0\]: Frequency
/// value (Frequency BCD16 encoded Hz).
pub const COM_STBY_RADIO_SET: &str = "COM_STBY_RADIO_SET";
/// Sets COM 1/2/3 standby frequency as a BCD16 encoded value. Parameters: \[0\]: Frequency
/// value (Frequency BCD16 encoded Hz).
pub const COM2_STBY_RADIO_SET: &str = "COM2_STBY_RADIO_SET";
/// Sets COM 1/2/3 standby frequency as a BCD16 encoded value. Parameters: \[0\]: Frequency
/// value (Frequency BCD16 encoded Hz).
pub const COM3_STBY_RADIO_SET: &str = "COM3_STBY_RADIO_SET";
/// Sets COM 1/2/3 standby frequency in Hz. Parameters: \[0\]: Frequency value (Hz).
pub const COM_STBY_RADIO_SET_HZ: &str = "COM_STBY_RADIO_SET_HZ";
/// Sets COM 1/2/3 standby frequency in Hz. Parameters: \[0\]: Frequency value (Hz).
pub const COM2_STBY_RADIO_SET_HZ: &str = "COM2_STBY_RADIO_SET_HZ";
/// Sets COM 1/2/3 standby frequency in Hz. Parameters: \[0\]: Frequency value (Hz).
pub const COM3_STBY_RADIO_SET_HZ: &str = "COM3_STBY_RADIO_SET_HZ";
/// Swaps COM 1 frequency with standby.
pub const COM_STBY_RADIO_SWAP: &str = "COM_STBY_RADIO_SWAP";
/// Decrement COM 1/2/3 frequency by 1 MHz. Values from 118 to 137, and this will wrap if the
/// values go over 137.
pub const COM_RADIO_WHOLE_DEC: &str = "COM_RADIO_WHOLE_DEC";
/// Decrement COM 1/2/3 frequency by 1 MHz. Values from 118 to 137, and this will wrap if the
/// values go over 137.
pub const COM2_RADIO_WHOLE_DEC: &str = "COM2_RADIO_WHOLE_DEC";
/// Decrement COM 1/2/3 frequency by 1 MHz. Values from 118 to 137, and this will wrap if the
/// values go over 137.
pub const COM3_RADIO_WHOLE_DEC: &str = "COM3_RADIO_WHOLE_DEC";
/// Increment COM 1/2/3 frequency by 1 MHz. Values are from 118 to 137, and this will wrap if
/// the values go under 118.
pub const COM_RADIO_WHOLE_INC: &str = "COM_RADIO_WHOLE_INC";
/// Increment COM 1/2/3 frequency by 1 MHz. Values are from 118 to 137, and this will wrap if
/// the values go under 118.
pub const COM2_RADIO_WHOLE_INC: &str = "COM2_RADIO_WHOLE_INC";
/// Increment COM 1/2/3 frequency by 1 MHz. Values are from 118 to 137, and this will wrap if
/// the values go under 118.
pub const COM3_RADIO_WHOLE_INC: &str = "COM3_RADIO_WHOLE_INC";
/// Swaps COM 1/2/3 frequency with the standby frequency. NOTE: COM_RADIO_SWAP is simply an
/// alias for COM1_RADIO_SWAP
pub const COM_RADIO_SWAP: &str = "COM_RADIO_SWAP";
/// Swaps COM 1/2/3 frequency with the standby frequency. NOTE: COM_RADIO_SWAP is simply an
/// alias for COM1_RADIO_SWAP
pub const COM1_RADIO_SWAP: &str = "COM1_RADIO_SWAP";
/// Swaps COM 1/2/3 frequency with the standby frequency. NOTE: COM_RADIO_SWAP is simply an
/// alias for COM1_RADIO_SWAP
pub const COM2_RADIO_SWAP: &str = "COM2_RADIO_SWAP";
/// Swaps COM 1/2/3 frequency with the standby frequency. NOTE: COM_RADIO_SWAP is simply an
/// alias for COM1_RADIO_SWAP
pub const COM3_RADIO_SWAP: &str = "COM3_RADIO_SWAP";
/// Sets receive on (1) or off (0) for COM 1/2/3. Parameters: \[0\] Bool.
pub const COM1_RECEIVE_SELECT: &str = "COM1_RECEIVE_SELECT";
/// Sets receive on (1) or off (0) for COM 1/2/3. Parameters: \[0\] Bool.
pub const COM2_RECEIVE_SELECT: &str = "COM2_RECEIVE_SELECT";
/// Sets receive on (1) or off (0) for COM 1/2/3. Parameters: \[0\] Bool.
pub const COM3_RECEIVE_SELECT: &str = "COM3_RECEIVE_SELECT";
/// Toggle between the different modes for COM 1/2/3.
pub const COM_1_SPACING_MODE_SWITCH: &str = "COM_1_SPACING_MODE_SWITCH";
/// Toggle between the different modes for COM 1/2/3.
pub const COM_2_SPACING_MODE_SWITCH: &str = "COM_2_SPACING_MODE_SWITCH";
/// Toggle between the different modes for COM 1/2/3.
pub const COM_3_SPACING_MODE_SWITCH: &str = "COM_3_SPACING_MODE_SWITCH";
/// Sets the COM 1/2/3 stored frequency as a BCD encoded value. Parameters: \[0\]: Frequency
/// value (Frequency BCD16 or Frequency BCD32 encoded Hz).
pub const COM1_STORED_FREQUENCY_SET: &str = "COM1_STORED_FREQUENCY_SET";
/// Sets the COM 1/2/3 stored frequency as a BCD encoded value. Parameters: \[0\]: Frequency
/// value (Frequency BCD16 or Frequency BCD32 encoded Hz).
pub const COM2_STORED_FREQUENCY_SET: &str = "COM2_STORED_FREQUENCY_SET";
/// Sets the COM 1/2/3 stored frequency as a BCD encoded value. Parameters: \[0\]: Frequency
/// value (Frequency BCD16 or Frequency BCD32 encoded Hz).
pub const COM3_STORED_FREQUENCY_SET: &str = "COM3_STORED_FREQUENCY_SET";
/// Sets COM 1/2/3 stored frequency as Hz. Parameters: \[0\]: Frequency value (Hz).
pub const COM1_STORED_FREQUENCY_SET_HZ: &str = "COM1_STORED_FREQUENCY_SET_HZ";
/// Sets COM 1/2/3 stored frequency as Hz. Parameters: \[0\]: Frequency value (Hz).
pub const COM2_STORED_FREQUENCY_SET_HZ: &str = "COM2_STORED_FREQUENCY_SET_HZ";
/// Sets COM 1/2/3 stored frequency as Hz. Parameters: \[0\]: Frequency value (Hz).
pub const COM3_STORED_FREQUENCY_SET_HZ: &str = "COM3_STORED_FREQUENCY_SET_HZ";
/// This is used to select the index for when you want to store frequencies. This can be done
/// by including this event when you store a frequency using one of the 2 available types:
/// COM1_STORED_FREQUENCY_SET, COM1_STORED_FREQUENCY_SET_HZ. For example if you want to store
/// a Com1 frequency HZ to an index of 2 then you would do this: 2
/// (>K:COM1_STORED_FREQUENCY_INDEX_SET) 112 (>K:COM1_STORED_FREQUENCY_SET_HZ) After you save
/// it you can retrieve this value from the 3 types of variables as seen here: (A:COM1 STORED
/// FREQUENCY:2, Hz) (A:COM1 STORED FREQUENCY:2, FrequencyBCD16) (A:COM1 STORED FREQUENCY:2,
/// FrequencyBCD32)
pub const COM1_STORED_FREQUENCY_INDEX_SET: &str = "COM1_STORED_FREQUENCY_INDEX_SET";
/// This is used to select the index for when you want to store frequencies. This can be done
/// by including this event when you store a frequency using one of the 2 available types:
/// COM1_STORED_FREQUENCY_SET, COM1_STORED_FREQUENCY_SET_HZ. For example if you want to store
/// a Com1 frequency HZ to an index of 2 then you would do this: 2
/// (>K:COM1_STORED_FREQUENCY_INDEX_SET) 112 (>K:COM1_STORED_FREQUENCY_SET_HZ) After you save
/// it you can retrieve this value from the 3 types of variables as seen here: (A:COM1 STORED
/// FREQUENCY:2, Hz) (A:COM1 STORED FREQUENCY:2, FrequencyBCD16) (A:COM1 STORED FREQUENCY:2,
/// FrequencyBCD32)
pub const COM2_STORED_FREQUENCY_INDEX_SET: &str = "COM2_STORED_FREQUENCY_INDEX_SET";
/// This is used to select the index for when you want to store frequencies. This can be done
/// by including this event when you store a frequency using one of the 2 available types:
/// COM1_STORED_FREQUENCY_SET, COM1_STORED_FREQUENCY_SET_HZ. For example if you want to store
/// a Com1 frequency HZ to an index of 2 then you would do this: 2
/// (>K:COM1_STORED_FREQUENCY_INDEX_SET) 112 (>K:COM1_STORED_FREQUENCY_SET_HZ) After you save
/// it you can retrieve this value from the 3 types of variables as seen here: (A:COM1 STORED
/// FREQUENCY:2, Hz) (A:COM1 STORED FREQUENCY:2, FrequencyBCD16) (A:COM1 STORED FREQUENCY:2,
/// FrequencyBCD32)
pub const COM3_STORED_FREQUENCY_INDEX_SET: &str = "COM3_STORED_FREQUENCY_INDEX_SET";
/// Selects COM 1/2 to transmit See PILOT_TRANSMITTER_SET instead.
pub const COM1_TRANSMIT_SELECT: &str = "COM1_TRANSMIT_SELECT";
/// Selects COM 1/2 to transmit See PILOT_TRANSMITTER_SET instead.
pub const COM2_TRANSMIT_SELECT: &str = "COM2_TRANSMIT_SELECT";
/// Sets the COM 1/2/3 volume (from 0 to 100). Parameters: \[0\]: Volume (0 - 100).
pub const COM1_VOLUME_SET: &str = "COM1_VOLUME_SET";
/// Sets the COM 1/2/3 volume (from 0 to 100). Parameters: \[0\]: Volume (0 - 100).
pub const COM2_VOLUME_SET: &str = "COM2_VOLUME_SET";
/// Sets the COM 1/2/3 volume (from 0 to 100). Parameters: \[0\]: Volume (0 - 100).
pub const COM3_VOLUME_SET: &str = "COM3_VOLUME_SET";
/// Increases the COM 1/2/3 volume by 0.02, and the resulting value will be clamped between 0
/// and 1.
pub const COM1_VOLUME_INC: &str = "COM1_VOLUME_INC";
/// Increases the COM 1/2/3 volume by 0.02, and the resulting value will be clamped between 0
/// and 1.
pub const COM2_VOLUME_INC: &str = "COM2_VOLUME_INC";
/// Increases the COM 1/2/3 volume by 0.02, and the resulting value will be clamped between 0
/// and 1.
pub const COM3_VOLUME_INC: &str = "COM3_VOLUME_INC";
/// Decreases the COM 1/2/3 volume by 0.02, and the resulting value will be clamped between 0
/// and 1.
pub const COM1_VOLUME_DEC: &str = "COM1_VOLUME_DEC";
/// Decreases the COM 1/2/3 volume by 0.02, and the resulting value will be clamped between 0
/// and 1.
pub const COM2_VOLUME_DEC: &str = "COM2_VOLUME_DEC";
/// Decreases the COM 1/2/3 volume by 0.02, and the resulting value will be clamped between 0
/// and 1.
pub const COM3_VOLUME_DEC: &str = "COM3_VOLUME_DEC";
/// Sets whether to receive on all COM radios (1, 0) Parameters: \[0\] Bool.
pub const COM_RECEIVE_ALL_SET: &str = "COM_RECEIVE_ALL_SET";
/// Toggles receive on (1) or off (0) for all COM radios.
pub const COM_RECEIVE_ALL_TOGGLE: &str = "COM_RECEIVE_ALL_TOGGLE";
/// Places COM 1/2/3 in "test mode". NOTE: Currently, placing COMs in test mode will have no
/// effect other than to set the SimVar COM TEST.
pub const RADIO_COMMNAV1_TEST_TOGGLE: &str = "RADIO_COMMNAV1_TEST_TOGGLE";
/// Places COM 1/2/3 in "test mode". NOTE: Currently, placing COMs in test mode will have no
/// effect other than to set the SimVar COM TEST.
pub const RADIO_COMMNAV2_TEST_TOGGLE: &str = "RADIO_COMMNAV2_TEST_TOGGLE";
/// Places COM 1/2/3 in "test mode". NOTE: Currently, placing COMs in test mode will have no
/// effect other than to set the SimVar COM TEST.
pub const RADIO_COMMNAV3_TEST_TOGGLE: &str = "RADIO_COMMNAV3_TEST_TOGGLE";
/// Toggles the COM 1/2 autoswitch on (1) or off (0).
pub const RADIO_COMM1_AUTOSWITCH_TOGGLE: &str = "RADIO_COMM1_AUTOSWITCH_TOGGLE";
/// Toggles the COM 1/2 autoswitch on (1) or off (0).
pub const RADIO_COMM2_AUTOSWITCH_TOGGLE: &str = "RADIO_COMM2_AUTOSWITCH_TOGGLE";

// DME
/// Selects the DME for use with +/-
pub const DME: &str = "DME";
/// Selects one of the two DME systems (1, 2). Parameters: \[0\]: DME ID.
pub const DME_SELECT: &str = "DME_SELECT";
/// Toggles DME between NAV 1 and NAV 2.
pub const TOGGLE_DME: &str = "TOGGLE_DME";
/// Sets the DME 1 / 2 display to NAV 1 / 2.
pub const DME1_TOGGLE: &str = "DME1_TOGGLE";
/// Sets the DME 1 / 2 display to NAV 1 / 2.
pub const DME2_TOGGLE: &str = "DME2_TOGGLE";
/// Turns the DME 1 / 2 ID off (0).
pub const RADIO_DME1_IDENT_DISABLE: &str = "RADIO_DME1_IDENT_DISABLE";
/// Turns the DME 1 / 2 ID off (0).
pub const RADIO_DME2_IDENT_DISABLE: &str = "RADIO_DME2_IDENT_DISABLE";
/// Turns the DME 1 / 2 ID on (1).
pub const RADIO_DME1_IDENT_ENABLE: &str = "RADIO_DME1_IDENT_ENABLE";
/// Turns the DME 1 / 2 ID on (1).
pub const RADIO_DME2_IDENT_ENABLE: &str = "RADIO_DME2_IDENT_ENABLE";
/// Sets the DME 1 /2 ID to on (1) or off (0). Parameters: \[0\]: Bool.
pub const RADIO_DME1_IDENT_SET: &str = "RADIO_DME1_IDENT_SET";
/// Sets the DME 1 /2 ID to on (1) or off (0). Parameters: \[0\]: Bool.
pub const RADIO_DME2_IDENT_SET: &str = "RADIO_DME2_IDENT_SET";
/// Toggles the DME 1 / 2 ID between on (1) and off (0).
pub const RADIO_DME1_IDENT_TOGGLE: &str = "RADIO_DME1_IDENT_TOGGLE";
/// Toggles the DME 1 / 2 ID between on (1) and off (0).
pub const RADIO_DME2_IDENT_TOGGLE: &str = "RADIO_DME2_IDENT_TOGGLE";
/// Turns on the identification sound for the selected DME.
pub const RADIO_SELECTED_DME_IDENT_ENABLE: &str = "RADIO_SELECTED_DME_IDENT_ENABLE";
/// Turns off the identification sound for the selected DME.
pub const RADIO_SELECTED_DME_IDENT_DISABLE: &str = "RADIO_SELECTED_DME_IDENT_DISABLE";
/// Sets the DME identification sound to the given filename. Parameters: \[0\]: Bool.
pub const RADIO_SELECTED_DME_IDENT_SET: &str = "RADIO_SELECTED_DME_IDENT_SET";
/// Turns on or off the identification sound for the selected DME.
pub const RADIO_SELECTED_DME_IDENT_TOGGLE: &str = "RADIO_SELECTED_DME_IDENT_TOGGLE";

// ELT
/// Switches the ELT off (0).
pub const ELT_OFF: &str = "ELT_OFF";
/// Switches the ELT on (1).
pub const ELT_ON: &str = "ELT_ON";
/// Sets the ELT on (1) or off (0). Parameters: \[0\]: Bool.
pub const ELT_SET: &str = "ELT_SET";
/// Toggles the ELT between on (1) and off (0).
pub const ELT_TOGGLE: &str = "ELT_TOGGLE";

// GPS
/// Press GPS Activate button.
pub const GPS_ACTIVATE_BUTTON: &str = "GPS_ACTIVATE_BUTTON";
/// Press GPS Button 1.
pub const GPS_BUTTON1: &str = "GPS_BUTTON1";
/// Press GPS Button 2.
pub const GPS_BUTTON2: &str = "GPS_BUTTON2";
/// Press GPS Button 3.
pub const GPS_BUTTON3: &str = "GPS_BUTTON3";
/// Press GPS Button 4.
pub const GPS_BUTTON4: &str = "GPS_BUTTON4";
/// Press GPS Button 5.
pub const GPS_BUTTON5: &str = "GPS_BUTTON5";
/// Press GPS Clear button (clears entered data on a page).
pub const GPS_CLEAR_BUTTON: &str = "GPS_CLEAR_BUTTON";
/// Press GPS Clear All button (clears all data immediately).
pub const GPS_CLEAR_ALL_BUTTON: &str = "GPS_CLEAR_ALL_BUTTON";
/// Triggers the pressing of the Clear button.
pub const GPS_CLEAR_BUTTON_DOWN: &str = "GPS_CLEAR_BUTTON_DOWN";
/// Triggers the release of the Clear button.
pub const GPS_CLEAR_BUTTON_UP: &str = "GPS_CLEAR_BUTTON_UP";
/// Selects GPS cursor
pub const GPS_CURSOR_BUTTON: &str = "GPS_CURSOR_BUTTON";
/// Brings up the "Direct To" page
pub const GPS_DIRECTTO_BUTTON: &str = "GPS_DIRECTTO_BUTTON";
/// Approves entered data.
pub const GPS_ENTER_BUTTON: &str = "GPS_ENTER_BUTTON";
/// Displays the programmed flightplan.
pub const GPS_FLIGHTPLAN_BUTTON: &str = "GPS_FLIGHTPLAN_BUTTON";
/// Increase GPS Group.
pub const GPS_GROUP_KNOB_INC: &str = "GPS_GROUP_KNOB_INC";
/// Decrease GPS Group.
pub const GPS_GROUP_KNOB_DEC: &str = "GPS_GROUP_KNOB_DEC";
/// Brings up page to select active legs in a flightplan.
pub const GPS_MENU_BUTTON: &str = "GPS_MENU_BUTTON";
/// Toggles the Message Page.
pub const GPS_MSG_BUTTON: &str = "GPS_MSG_BUTTON";
/// Triggers the pressing of the message button.
pub const GPS_MSG_BUTTON_DOWN: &str = "GPS_MSG_BUTTON_DOWN";
/// Triggers the release of the message button.
pub const GPS_MSG_BUTTON_UP: &str = "GPS_MSG_BUTTON_UP";
/// Selects Nearest Airport Page.
pub const GPS_NEAREST_BUTTON: &str = "GPS_NEAREST_BUTTON";
/// Toggle GPS OBS mode active status on/off.
pub const GPS_OBS: &str = "GPS_OBS";
/// Toggles automatic sequencing of waypoints.
pub const GPS_OBS_BUTTON: &str = "GPS_OBS_BUTTON";
/// Decreases GPS OBS value by 1 degree (if the value goes below 1 it will wrap to 360).
pub const GPS_OBS_DEC: &str = "GPS_OBS_DEC";
/// Increases GPS OBS value by 1 degree (if the value goes above 360 it will wrap to 1).
pub const GPS_OBS_INC: &str = "GPS_OBS_INC";
/// Turn the GPS OBS mode to be inactive.
pub const GPS_OBS_OFF: &str = "GPS_OBS_OFF";
/// Turn on the GPS OBS mode to be active
pub const GPS_OBS_ON: &str = "GPS_OBS_ON";
/// Set the GPS OBS value to a new value, in degrees. Parameters: \[0\]: Value in degrees.
pub const GPS_OBS_SET: &str = "GPS_OBS_SET";
/// Increments through pages
pub const GPS_PAGE_KNOB_INC: &str = "GPS_PAGE_KNOB_INC";
/// Decrements through pages
pub const GPS_PAGE_KNOB_DEC: &str = "GPS_PAGE_KNOB_DEC";
/// Displays the approach procedure page.
pub const GPS_PROCEDURE_BUTTON: &str = "GPS_PROCEDURE_BUTTON";
/// Toggles power button
pub const GPS_POWER_BUTTON: &str = "GPS_POWER_BUTTON";
/// (no description provided by the vendor docs)
pub const GPS_SETUP_BUTTON: &str = "GPS_SETUP_BUTTON";
/// Displays terrain information on default display
pub const GPS_TERRAIN_BUTTON: &str = "GPS_TERRAIN_BUTTON";
/// (no description provided by the vendor docs)
pub const GPS_VNAV_BUTTON: &str = "GPS_VNAV_BUTTON";
/// Zooms in default display
pub const GPS_ZOOMIN_BUTTON: &str = "GPS_ZOOMIN_BUTTON";
/// Zooms out default display
pub const GPS_ZOOMOUT_BUTTON: &str = "GPS_ZOOMOUT_BUTTON";
/// Toggles between GPS and NAV 1 driving NAV 1 OBS display (and AP)
pub const TOGGLE_GPS_DRIVES_NAV1: &str = "TOGGLE_GPS_DRIVES_NAV1";

// Miscellaneous
/// This event can be used to select the COM channel to use for the copilot. The input is one
/// of the following values: 0: Com1 1: Com2 2: Com3 4: None The exact number of COM available
/// will depend on the Com.N parameter of the systems.cfg. Parameters: \[0\]: The Com channel
/// to select..
pub const COPILOT_TRANSMITTER_SET: &str = "COPILOT_TRANSMITTER_SET";
/// Swaps frequency with standby on whichever NAV or COM radio is selected.
pub const FREQUENCY_SWAP: &str = "FREQUENCY_SWAP";
/// This event can be used to set the intercom mode. The input is one of the following values:
/// 0: ISO 1: All 2: Crew Parameters: \[0\]: Mode.
pub const INTERCOM_MODE_SET: &str = "INTERCOM_MODE_SET";
/// Sets a boolean which you can retrieve with MARKER_BEACON_SENSITIVITY_HIGH SimVar.
/// Parameters: \[0\]: Bool.
pub const MARKER_BEACON_SENSITIVITY_HIGH: &str = "MARKER_BEACON_SENSITIVITY_HIGH";
/// Enables / Disables beacon marker sounds. Parameters: \[0\]: Bool.
pub const MARKER_BEACON_TEST_MUTE: &str = "MARKER_BEACON_TEST_MUTE";
/// Toggles marker beacon sound on/off.
pub const MARKER_SOUND_TOGGLE: &str = "MARKER_SOUND_TOGGLE";
/// Sets marker beacon sound (1, 0). Not currently used in the simulation. Parameters: \[0\]:
/// Bool.
pub const MARKER_SOUND_SET: &str = "MARKER_SOUND_SET";
/// This event can be used to select the COM channel to use for the pilot. The input is one of
/// the following values: 0: Com1 1: Com2 2: Com3 4: None The exact number of COM available
/// will depend on the Com.N parameter of the systems.cfg. Parameters: \[0\]: The Com channel
/// to select..
pub const PILOT_TRANSMITTER_SET: &str = "PILOT_TRANSMITTER_SET";
/// Not currently used in the simulation.
pub const TOGGLE_RADAR: &str = "TOGGLE_RADAR";
/// Not currently used in the simulation.
pub const TOGGLE_RADIO: &str = "TOGGLE_RADIO";
/// Not currently used in the simulation.
pub const TOGGLE_RAD_INS_SWITCH: &str = "TOGGLE_RAD_INS_SWITCH";

// NAV
/// Sequentially selects the NAV tuner digits for use with +/-. Follow by SELECT_1, SELECT_2,
/// SELECT_3, or SELECT_4 for NAV 1, 2, 3 or 4.
pub const NAV_RADIO: &str = "NAV_RADIO";
/// This event is used to enable (set to 1, TRUE) or disable (set to 0, FALSE) the following
/// SimVars: NAV_CLOSE_DME NAV_CLOSE_FREQUENCY NAV_CLOSE_IDENT NAV_CLOSE_LOCALIZER
/// NAV_CLOSE_NAME Also note that all the NAV key events are simply aliases for each other,
/// and using any of them will have the same effect. Parameters: \[0\]: Bool.
pub const NAV1_CLOSE_FREQ_SET: &str = "NAV1_CLOSE_FREQ_SET";
/// This event is used to enable (set to 1, TRUE) or disable (set to 0, FALSE) the following
/// SimVars: NAV_CLOSE_DME NAV_CLOSE_FREQUENCY NAV_CLOSE_IDENT NAV_CLOSE_LOCALIZER
/// NAV_CLOSE_NAME Also note that all the NAV key events are simply aliases for each other,
/// and using any of them will have the same effect. Parameters: \[0\]: Bool.
pub const NAV2_CLOSE_FREQ_SET: &str = "NAV2_CLOSE_FREQ_SET";
/// This event is used to enable (set to 1, TRUE) or disable (set to 0, FALSE) the following
/// SimVars: NAV_CLOSE_DME NAV_CLOSE_FREQUENCY NAV_CLOSE_IDENT NAV_CLOSE_LOCALIZER
/// NAV_CLOSE_NAME Also note that all the NAV key events are simply aliases for each other,
/// and using any of them will have the same effect. Parameters: \[0\]: Bool.
pub const NAV3_CLOSE_FREQ_SET: &str = "NAV3_CLOSE_FREQ_SET";
/// This event is used to enable (set to 1, TRUE) or disable (set to 0, FALSE) the following
/// SimVars: NAV_CLOSE_DME NAV_CLOSE_FREQUENCY NAV_CLOSE_IDENT NAV_CLOSE_LOCALIZER
/// NAV_CLOSE_NAME Also note that all the NAV key events are simply aliases for each other,
/// and using any of them will have the same effect. Parameters: \[0\]: Bool.
pub const NAV4_CLOSE_FREQ_SET: &str = "NAV4_CLOSE_FREQ_SET";
/// Decrements the chosen NAV frequency by 25 KHz.
pub const NAV1_RADIO_FRACT_DEC: &str = "NAV1_RADIO_FRACT_DEC";
/// Decrements the chosen NAV frequency by 25 KHz.
pub const NAV2_RADIO_FRACT_DEC: &str = "NAV2_RADIO_FRACT_DEC";
/// Decrements the chosen NAV frequency by 25 KHz.
pub const NAV3_RADIO_FRACT_DEC: &str = "NAV3_RADIO_FRACT_DEC";
/// Decrements the chosen NAV frequency by 25 KHz.
pub const NAV4_RADIO_FRACT_DEC: &str = "NAV4_RADIO_FRACT_DEC";
/// Decrement the chosen NAV frequency by 50 KHz, and will carry when the value wraps.
pub const NAV1_RADIO_FRACT_DEC_CARRY: &str = "NAV1_RADIO_FRACT_DEC_CARRY";
/// Decrement the chosen NAV frequency by 50 KHz, and will carry when the value wraps.
pub const NAV2_RADIO_FRACT_DEC_CARRY: &str = "NAV2_RADIO_FRACT_DEC_CARRY";
/// Decrement the chosen NAV frequency by 50 KHz, and will carry when the value wraps.
pub const NAV3_RADIO_FRACT_DEC_CARRY: &str = "NAV3_RADIO_FRACT_DEC_CARRY";
/// Decrement the chosen NAV frequency by 50 KHz, and will carry when the value wraps.
pub const NAV4_RADIO_FRACT_DEC_CARRY: &str = "NAV4_RADIO_FRACT_DEC_CARRY";
/// Increments the chosen NAV frequency by 25 KHz.
pub const NAV1_RADIO_FRACT_INC: &str = "NAV1_RADIO_FRACT_INC";
/// Increments the chosen NAV frequency by 25 KHz.
pub const NAV2_RADIO_FRACT_INC: &str = "NAV2_RADIO_FRACT_INC";
/// Increments the chosen NAV frequency by 25 KHz.
pub const NAV3_RADIO_FRACT_INC: &str = "NAV3_RADIO_FRACT_INC";
/// Increments the chosen NAV frequency by 25 KHz.
pub const NAV4_RADIO_FRACT_INC: &str = "NAV4_RADIO_FRACT_INC";
/// Increment the chosen NAV frequency by 50 KHz, and will carry when the value wraps.
pub const NAV1_RADIO_FRACT_INC_CARRY: &str = "NAV1_RADIO_FRACT_INC_CARRY";
/// Increment the chosen NAV frequency by 50 KHz, and will carry when the value wraps.
pub const NAV2_RADIO_FRACT_INC_CARRY: &str = "NAV2_RADIO_FRACT_INC_CARRY";
/// Increment the chosen NAV frequency by 50 KHz, and will carry when the value wraps.
pub const NAV3_RADIO_FRACT_INC_CARRY: &str = "NAV3_RADIO_FRACT_INC_CARRY";
/// Increment the chosen NAV frequency by 50 KHz, and will carry when the value wraps.
pub const NAV4_RADIO_FRACT_INC_CARRY: &str = "NAV4_RADIO_FRACT_INC_CARRY";
/// Sets the chosen NAV frequency (Frequency BCD16 encoded Hz). Parameters: \[0\] Frequency
/// value.
pub const NAV1_RADIO_SET: &str = "NAV1_RADIO_SET";
/// Sets the chosen NAV frequency (Frequency BCD16 encoded Hz). Parameters: \[0\] Frequency
/// value.
pub const NAV2_RADIO_SET: &str = "NAV2_RADIO_SET";
/// Sets the chosen NAV frequency (Frequency BCD16 encoded Hz). Parameters: \[0\] Frequency
/// value.
pub const NAV3_RADIO_SET: &str = "NAV3_RADIO_SET";
/// Sets the chosen NAV frequency (Frequency BCD16 encoded Hz). Parameters: \[0\] Frequency
/// value.
pub const NAV4_RADIO_SET: &str = "NAV4_RADIO_SET";
/// Sets the chosen NAV frequency (Hz). Parameters: \[0\] Frequency value.
pub const NAV1_RADIO_SET_HZ: &str = "NAV1_RADIO_SET_HZ";
/// Sets the chosen NAV frequency (Hz). Parameters: \[0\] Frequency value.
pub const NAV2_RADIO_SET_HZ: &str = "NAV2_RADIO_SET_HZ";
/// Sets the chosen NAV frequency (Hz). Parameters: \[0\] Frequency value.
pub const NAV3_RADIO_SET_HZ: &str = "NAV3_RADIO_SET_HZ";
/// Sets the chosen NAV frequency (Hz). Parameters: \[0\] Frequency value.
pub const NAV4_RADIO_SET_HZ: &str = "NAV4_RADIO_SET_HZ";
/// Swap between the chosen NAV frequency and the corresponding standby frequency.
pub const NAV1_RADIO_SWAP: &str = "NAV1_RADIO_SWAP";
/// Swap between the chosen NAV frequency and the corresponding standby frequency.
pub const NAV2_RADIO_SWAP: &str = "NAV2_RADIO_SWAP";
/// Swap between the chosen NAV frequency and the corresponding standby frequency.
pub const NAV3_RADIO_SWAP: &str = "NAV3_RADIO_SWAP";
/// Swap between the chosen NAV frequency and the corresponding standby frequency.
pub const NAV4_RADIO_SWAP: &str = "NAV4_RADIO_SWAP";
/// Decrements the chosen NAV frequency by one MHz.
pub const NAV1_RADIO_WHOLE_DEC: &str = "NAV1_RADIO_WHOLE_DEC";
/// Decrements the chosen NAV frequency by one MHz.
pub const NAV2_RADIO_WHOLE_DEC: &str = "NAV2_RADIO_WHOLE_DEC";
/// Decrements the chosen NAV frequency by one MHz.
pub const NAV3_RADIO_WHOLE_DEC: &str = "NAV3_RADIO_WHOLE_DEC";
/// Decrements the chosen NAV frequency by one MHz.
pub const NAV4_RADIO_WHOLE_DEC: &str = "NAV4_RADIO_WHOLE_DEC";
/// Increments the chosen NAV frequency by one MHz.
pub const NAV1_RADIO_WHOLE_INC: &str = "NAV1_RADIO_WHOLE_INC";
/// Increments the chosen NAV frequency by one MHz.
pub const NAV2_RADIO_WHOLE_INC: &str = "NAV2_RADIO_WHOLE_INC";
/// Increments the chosen NAV frequency by one MHz.
pub const NAV3_RADIO_WHOLE_INC: &str = "NAV3_RADIO_WHOLE_INC";
/// Increments the chosen NAV frequency by one MHz.
pub const NAV4_RADIO_WHOLE_INC: &str = "NAV4_RADIO_WHOLE_INC";
/// Sets the chosen NAV standby frequency (Frequency BCD16 encoded Hz). Parameters: \[0\]
/// Frequency value.
pub const NAV1_STBY_SET: &str = "NAV1_STBY_SET";
/// Sets the chosen NAV standby frequency (Frequency BCD16 encoded Hz). Parameters: \[0\]
/// Frequency value.
pub const NAV2_STBY_SET: &str = "NAV2_STBY_SET";
/// Sets the chosen NAV standby frequency (Frequency BCD16 encoded Hz). Parameters: \[0\]
/// Frequency value.
pub const NAV3_STBY_SET: &str = "NAV3_STBY_SET";
/// Sets the chosen NAV standby frequency (Frequency BCD16 encoded Hz). Parameters: \[0\]
/// Frequency value.
pub const NAV4_STBY_SET: &str = "NAV4_STBY_SET";
/// Sets the chosen NAV standby frequency (Hz). Parameters: \[0\] Frequency value.
pub const NAV1_STBY_SET_HZ: &str = "NAV1_STBY_SET_HZ";
/// Sets the chosen NAV standby frequency (Hz). Parameters: \[0\] Frequency value.
pub const NAV2_STBY_SET_HZ: &str = "NAV2_STBY_SET_HZ";
/// Sets the chosen NAV standby frequency (Hz). Parameters: \[0\] Frequency value.
pub const NAV3_STBY_SET_HZ: &str = "NAV3_STBY_SET_HZ";
/// Sets the chosen NAV standby frequency (Hz). Parameters: \[0\] Frequency value.
pub const NAV4_STBY_SET_HZ: &str = "NAV4_STBY_SET_HZ";
/// Decrement the volume by 0.02, down to a minimum of 0.
pub const NAV1_VOLUME_DEC: &str = "NAV1_VOLUME_DEC";
/// Decrement the volume by 0.02, down to a minimum of 0.
pub const NAV2_VOLUME_DEC: &str = "NAV2_VOLUME_DEC";
/// Decrement the volume by 0.02, down to a minimum of 0.
pub const NAV3_VOLUME_DEC: &str = "NAV3_VOLUME_DEC";
/// Decrement the volume by 0.02, down to a minimum of 0.
pub const NAV4_VOLUME_DEC: &str = "NAV4_VOLUME_DEC";
/// Increment the volume by 0.02, up to a maximum of 1.
pub const NAV1_VOLUME_INC: &str = "NAV1_VOLUME_INC";
/// Increment the volume by 0.02, up to a maximum of 1.
pub const NAV2_VOLUME_INC: &str = "NAV2_VOLUME_INC";
/// Increment the volume by 0.02, up to a maximum of 1.
pub const NAV3_VOLUME_INC: &str = "NAV3_VOLUME_INC";
/// Increment the volume by 0.02, up to a maximum of 1.
pub const NAV4_VOLUME_INC: &str = "NAV4_VOLUME_INC";
/// Sets the volume for the chosen NAV. NOTE: These events are deprecated as they no longer
/// work correctly. Instead use the _EX1 versions, listed below. Parameters: \[0\] Volume
/// value (0 -1).
pub const NAV1_VOLUME_SET: &str = "NAV1_VOLUME_SET";
/// Sets the volume for the chosen NAV. NOTE: These events are deprecated as they no longer
/// work correctly. Instead use the _EX1 versions, listed below. Parameters: \[0\] Volume
/// value (0 -1).
pub const NAV2_VOLUME_SET: &str = "NAV2_VOLUME_SET";
/// Sets the volume for the chosen NAV. NOTE: These events are deprecated as they no longer
/// work correctly. Instead use the _EX1 versions, listed below. Parameters: \[0\] Volume
/// value (0 -1).
pub const NAV3_VOLUME_SET: &str = "NAV3_VOLUME_SET";
/// Sets the volume for the chosen NAV. NOTE: These events are deprecated as they no longer
/// work correctly. Instead use the _EX1 versions, listed below. Parameters: \[0\] Volume
/// value (0 -1).
pub const NAV4_VOLUME_SET: &str = "NAV4_VOLUME_SET";
/// Sets the volume for the chosen NAV, from 0 to 100 (interpolated in the simulation to a
/// value from 0 to 1). Parameters: \[0\] Volume value (0 - 100).
pub const NAV1_VOLUME_SET_EX1: &str = "NAV1_VOLUME_SET_EX1";
/// Sets the volume for the chosen NAV, from 0 to 100 (interpolated in the simulation to a
/// value from 0 to 1). Parameters: \[0\] Volume value (0 - 100).
pub const NAV2_VOLUME_SET_EX1: &str = "NAV2_VOLUME_SET_EX1";
/// Sets the volume for the chosen NAV, from 0 to 100 (interpolated in the simulation to a
/// value from 0 to 1). Parameters: \[0\] Volume value (0 - 100).
pub const NAV3_VOLUME_SET_EX1: &str = "NAV3_VOLUME_SET_EX1";
/// Sets the volume for the chosen NAV, from 0 to 100 (interpolated in the simulation to a
/// value from 0 to 1). Parameters: \[0\] Volume value (0 - 100).
pub const NAV4_VOLUME_SET_EX1: &str = "NAV4_VOLUME_SET_EX1";
/// Toggle NAV1 / NAV2 auto-switch.
pub const RADIO_NAV1_AUTOSWITCH_TOGGLE: &str = "RADIO_NAV1_AUTOSWITCH_TOGGLE";
/// Toggle NAV1 / NAV2 auto-switch.
pub const RADIO_NAV2_AUTOSWITCH_TOGGLE: &str = "RADIO_NAV2_AUTOSWITCH_TOGGLE";

// TACAN
/// Set TACAN 1/2 active channel, from 1 to 127. Parameters: \[0\]: Channel value (1 - 127).
pub const TACAN1_ACTIVE_CHANNEL_SET: &str = "TACAN1_ACTIVE_CHANNEL_SET";
/// Set TACAN 1/2 active channel, from 1 to 127. Parameters: \[0\]: Channel value (1 - 127).
pub const TACAN2_ACTIVE_CHANNEL_SET: &str = "TACAN2_ACTIVE_CHANNEL_SET";
/// Set TACAN 1/2 standby channel, from 1 to 127 Parameters: \[0\]: Channel value (1 - 127).
pub const TACAN1_STANDBY_CHANNEL_SET: &str = "TACAN1_STANDBY_CHANNEL_SET";
/// Set TACAN 1/2 standby channel, from 1 to 127 Parameters: \[0\]: Channel value (1 - 127).
pub const TACAN2_STANDBY_CHANNEL_SET: &str = "TACAN2_STANDBY_CHANNEL_SET";
/// Set the TACAN 1/2 active mode, either 0 (X) or 1 (Y). Parameters: \[0\]: Active mode value
/// (0, 1).
pub const TACAN1_ACTIVE_MODE_SET: &str = "TACAN1_ACTIVE_MODE_SET";
/// Set the TACAN 1/2 active mode, either 0 (X) or 1 (Y). Parameters: \[0\]: Active mode value
/// (0, 1).
pub const TACAN2_ACTIVE_MODE_SET: &str = "TACAN2_ACTIVE_MODE_SET";
/// Set the TACAN 1/2 standby mode, either 0 (X) or 1 (Y). Parameters: \[0\]: Standby mode
/// value (0, 1).
pub const TACAN1_STANDBY_MODE_SET: &str = "TACAN1_STANDBY_MODE_SET";
/// Set the TACAN 1/2 standby mode, either 0 (X) or 1 (Y). Parameters: \[0\]: Standby mode
/// value (0, 1).
pub const TACAN2_STANDBY_MODE_SET: &str = "TACAN2_STANDBY_MODE_SET";
/// Swap between active and standby TACAN 1/2 frequencies.
pub const TACAN1_SWAP: &str = "TACAN1_SWAP";
/// Swap between active and standby TACAN 1/2 frequencies.
pub const TACAN2_SWAP: &str = "TACAN2_SWAP";
/// Increase TACAN 1/2 volume by 1, up to a maximum volume of 100.
pub const TACAN1_VOLUME_INC: &str = "TACAN1_VOLUME_INC";
/// Increase TACAN 1/2 volume by 1, up to a maximum volume of 100.
pub const TACAN2_VOLUME_INC: &str = "TACAN2_VOLUME_INC";
/// Decrease TACAN 1/2 volume by 1, down to a minimum volume of 0.
pub const TACAN1_VOLUME_DEC: &str = "TACAN1_VOLUME_DEC";
/// Decrease TACAN 1/2 volume by 1, down to a minimum volume of 0.
pub const TACAN2_VOLUME_DEC: &str = "TACAN2_VOLUME_DEC";
/// Set TACAN 1/2 volume to a value from 0 (no volume) to 100 (full volume). Parameters:
/// \[0\]: Volume value (0, 100).
pub const TACAN1_VOLUME_SET: &str = "TACAN1_VOLUME_SET";
/// Set TACAN 1/2 volume to a value from 0 (no volume) to 100 (full volume). Parameters:
/// \[0\]: Volume value (0, 100).
pub const TACAN2_VOLUME_SET: &str = "TACAN2_VOLUME_SET";
/// Set TACAN 1/2 Omni bearing indicator. The behavior is similar to the OBS knob on a
/// traditional VOR. Parameters: \[0\]: Bearing indicator value.
pub const TACAN1_SET: &str = "TACAN1_SET";
/// Set TACAN 1/2 Omni bearing indicator. The behavior is similar to the OBS knob on a
/// traditional VOR. Parameters: \[0\]: Bearing indicator value.
pub const TACAN2_SET: &str = "TACAN2_SET";
/// Decrease TACAN 1/2 OBI by 1 degree. OBI bearing is between 0° and 359°, and and will loop
/// back to 359º if you go below 0º.
pub const TACAN1_OBI_DEC: &str = "TACAN1_OBI_DEC";
/// Decrease TACAN 1/2 OBI by 1 degree. OBI bearing is between 0° and 359°, and and will loop
/// back to 359º if you go below 0º.
pub const TACAN2_OBI_DEC: &str = "TACAN2_OBI_DEC";
/// Increase TACAN 1/2 OBI by 1 degree. OBI bearing is between 0° and 359°, and and will loop
/// back to 0º if you go above 359º.
pub const TACAN1_OBI_INC: &str = "TACAN1_OBI_INC";
/// Increase TACAN 1/2 OBI by 1 degree. OBI bearing is between 0° and 359°, and and will loop
/// back to 0º if you go above 359º.
pub const TACAN2_OBI_INC: &str = "TACAN2_OBI_INC";
/// Decrease TACAN 1/2 OBI by 10 degrees. OBI bearing is between 0° and 359°, and and will
/// loop back to 359º if you go below 0º.
pub const TACAN1_OBI_FAST_DEC: &str = "TACAN1_OBI_FAST_DEC";
/// Decrease TACAN 1/2 OBI by 10 degrees. OBI bearing is between 0° and 359°, and and will
/// loop back to 359º if you go below 0º.
pub const TACAN2_OBI_FAST_DEC: &str = "TACAN2_OBI_FAST_DEC";
/// Increase TACAN 1/2 OBI by 10 degrees. OBI bearing is between 0° and 359°, and and will
/// loop back to 0º if you go above 359º.
pub const TACAN1_OBI_FAST_INC: &str = "TACAN1_OBI_FAST_INC";
/// Increase TACAN 1/2 OBI by 10 degrees. OBI bearing is between 0° and 359°, and and will
/// loop back to 0º if you go above 359º.
pub const TACAN2_OBI_FAST_INC: &str = "TACAN2_OBI_FAST_INC";
/// Toggles the TACAN DRIVES NAV SimVar to indicate that the NAV1 autopilot feature is driven
/// by Tacan instead of classic Nav systems (VOR/ILS).
pub const TOGGLE_TACAN_DRIVES_NAV1: &str = "TOGGLE_TACAN_DRIVES_NAV1";

// VOR
/// Turns VOR 1/2/3/4 ID off.
pub const RADIO_VOR1_IDENT_DISABLE: &str = "RADIO_VOR1_IDENT_DISABLE";
/// Turns VOR 1/2/3/4 ID off.
pub const RADIO_VOR2_IDENT_DISABLE: &str = "RADIO_VOR2_IDENT_DISABLE";
/// Turns VOR 1/2/3/4 ID off.
pub const RADIO_VOR3_IDENT_DISABLE: &str = "RADIO_VOR3_IDENT_DISABLE";
/// Turns VOR 1/2/3/4 ID off.
pub const RADIO_VOR4_IDENT_DISABLE: &str = "RADIO_VOR4_IDENT_DISABLE";
/// Turns VOR 1/2/3/4 ID on.
pub const RADIO_VOR1_IDENT_ENABLE: &str = "RADIO_VOR1_IDENT_ENABLE";
/// Turns VOR 1/2/3/4 ID on.
pub const RADIO_VOR2_IDENT_ENABLE: &str = "RADIO_VOR2_IDENT_ENABLE";
/// Turns VOR 1/2/3/4 ID on.
pub const RADIO_VOR3_IDENT_ENABLE: &str = "RADIO_VOR3_IDENT_ENABLE";
/// Turns VOR 1/2/3/4 ID on.
pub const RADIO_VOR4_IDENT_ENABLE: &str = "RADIO_VOR4_IDENT_ENABLE";
/// Sets VOR 1/2/3/4 ID (on/off). Parameters: \[0\]: Bool.
pub const RADIO_VOR1_IDENT_SET: &str = "RADIO_VOR1_IDENT_SET";
/// Sets VOR 1/2/3/4 ID (on/off). Parameters: \[0\]: Bool.
pub const RADIO_VOR2_IDENT_SET: &str = "RADIO_VOR2_IDENT_SET";
/// Sets VOR 1/2/3/4 ID (on/off). Parameters: \[0\]: Bool.
pub const RADIO_VOR3_IDENT_SET: &str = "RADIO_VOR3_IDENT_SET";
/// Sets VOR 1/2/3/4 ID (on/off). Parameters: \[0\]: Bool.
pub const RADIO_VOR4_IDENT_SET: &str = "RADIO_VOR4_IDENT_SET";
/// Toggles VOR 1/2/3/4 ID between on and off.
pub const RADIO_VOR1_IDENT_TOGGLE: &str = "RADIO_VOR1_IDENT_TOGGLE";
/// Toggles VOR 1/2/3/4 ID between on and off.
pub const RADIO_VOR2_IDENT_TOGGLE: &str = "RADIO_VOR2_IDENT_TOGGLE";
/// Toggles VOR 1/2/3/4 ID between on and off.
pub const RADIO_VOR3_IDENT_TOGGLE: &str = "RADIO_VOR3_IDENT_TOGGLE";
/// Toggles VOR 1/2/3/4 ID between on and off.
pub const RADIO_VOR4_IDENT_TOGGLE: &str = "RADIO_VOR4_IDENT_TOGGLE";
/// Sequentially selects the VOR OBS for use with +/-. Follow by SELECT_1 for VOR 1 and
/// SELECT_2 for VOR 2.
pub const VOR_OBS: &str = "VOR_OBS";
/// Decrements the VOR 1/2/3/4 OBS setting
pub const VOR1_OBI_DEC: &str = "VOR1_OBI_DEC";
/// Decrements the VOR 1/2/3/4 OBS setting
pub const VOR2_OBI_DEC: &str = "VOR2_OBI_DEC";
/// Decrements the VOR 1/2/3/4 OBS setting
pub const VOR3_OBI_DEC: &str = "VOR3_OBI_DEC";
/// Decrements the VOR 1/2/3/4 OBS setting
pub const VOR4_OBI_DEC: &str = "VOR4_OBI_DEC";
/// Decrements the VOR 1/2/3/4 OBS setting by 10 degrees. The value will stop on 0 and not
/// arap.
pub const VOR1_OBI_FAST_DEC: &str = "VOR1_OBI_FAST_DEC";
/// Decrements the VOR 1/2/3/4 OBS setting by 10 degrees. The value will stop on 0 and not
/// arap.
pub const VOR2_OBI_FAST_DEC: &str = "VOR2_OBI_FAST_DEC";
/// Decrements the VOR 1/2/3/4 OBS setting by 10 degrees. The value will stop on 0 and not
/// arap.
pub const VOR3_OBI_FAST_DEC: &str = "VOR3_OBI_FAST_DEC";
/// Decrements the VOR 1/2/3/4 OBS setting by 10 degrees. The value will stop on 0 and not
/// arap.
pub const VOR4_OBI_FAST_DEC: &str = "VOR4_OBI_FAST_DEC";
/// Increments the VOR 1/2/3/4 OBS setting by 10 degrees. The value will stop on 360 and not
/// arap.
pub const VOR1_OBI_FAST_INC: &str = "VOR1_OBI_FAST_INC";
/// Increments the VOR 1/2/3/4 OBS setting by 10 degrees. The value will stop on 360 and not
/// arap.
pub const VOR2_OBI_FAST_INC: &str = "VOR2_OBI_FAST_INC";
/// Increments the VOR 1/2/3/4 OBS setting by 10 degrees. The value will stop on 360 and not
/// arap.
pub const VOR3_OBI_FAST_INC: &str = "VOR3_OBI_FAST_INC";
/// Increments the VOR 1/2/3/4 OBS setting by 10 degrees. The value will stop on 360 and not
/// arap.
pub const VOR4_OBI_FAST_INC: &str = "VOR4_OBI_FAST_INC";
/// Increments the VOR 1/2/3/4 OBS setting
pub const VOR1_OBI_INC: &str = "VOR1_OBI_INC";
/// Increments the VOR 1/2/3/4 OBS setting
pub const VOR2_OBI_INC: &str = "VOR2_OBI_INC";
/// Increments the VOR 1/2/3/4 OBS setting
pub const VOR3_OBI_INC: &str = "VOR3_OBI_INC";
/// Increments the VOR 1/2/3/4 OBS setting
pub const VOR4_OBI_INC: &str = "VOR4_OBI_INC";
/// Sets OBS 1/2/3/4 (0 to 360) Parameters: \[0\]: Value (0 - 360).
pub const VOR1_SET: &str = "VOR1_SET";
/// Sets OBS 1/2/3/4 (0 to 360) Parameters: \[0\]: Value (0 - 360).
pub const VOR2_SET: &str = "VOR2_SET";
/// Sets OBS 1/2/3/4 (0 to 360) Parameters: \[0\]: Value (0 - 360).
pub const VOR3_SET: &str = "VOR3_SET";
/// Sets OBS 1/2/3/4 (0 to 360) Parameters: \[0\]: Value (0 - 360).
pub const VOR4_SET: &str = "VOR4_SET";

// XPNDR (Transponder)
/// Sequentially selects the transponder digits for use with +/-.
pub const XPNDR: &str = "XPNDR";
/// Decrements the first digit of the transponder.
pub const XPNDR_1000_DEC: &str = "XPNDR_1000_DEC";
/// Decrements the second digit of the transponder.
pub const XPNDR_100_DEC: &str = "XPNDR_100_DEC";
/// Decrements the third digit of the transponder.
pub const XPNDR_10_DEC: &str = "XPNDR_10_DEC";
/// Decrements the fourth digit of the transponder.
pub const XPNDR_1_DEC: &str = "XPNDR_1_DEC";
/// Increments the first digit of the transponder.
pub const XPNDR_1000_INC: &str = "XPNDR_1000_INC";
/// Increments the second digit of the transponder.
pub const XPNDR_100_INC: &str = "XPNDR_100_INC";
/// Increments the third digit of the transponder.
pub const XPNDR_10_INC: &str = "XPNDR_10_INC";
/// Increments the fourth digit of the transponder.
pub const XPNDR_1_INC: &str = "XPNDR_1_INC";
/// Decrements the fourth digit of the transponder, with carry.
pub const XPNDR_DEC_CARRY: &str = "XPNDR_DEC_CARRY";
/// Increments the fourth digit of the transponder, with carry.
pub const XPNDR_INC_CARRY: &str = "XPNDR_INC_CARRY";
/// Disable the transponder Ident (can be used along with the simvar TRANSPONDER_IDENT).
pub const XPNDR_IDENT_OFF: &str = "XPNDR_IDENT_OFF";
/// Enable the transponder Ident (can be used along with the simvar TRANSPONDER_IDENT). After
/// 18 seconds it will disable automatically.
pub const XPNDR_IDENT_ON: &str = "XPNDR_IDENT_ON";
/// Set the transponder Ident on or off (can be used along with the simvar TRANSPONDER_IDENT).
/// If set to on, it will switch off automatically after 18 seconds. Parameters: \[0\]: Bool.
pub const XPNDR_IDENT_SET: &str = "XPNDR_IDENT_SET";
/// Toggle the transponder Ident from on to off or off to on (can be used along with the
/// simvar TRANSPONDER_IDENT).
pub const XPNDR_IDENT_TOGGLE: &str = "XPNDR_IDENT_TOGGLE";
/// Sets the transponder frequency code. Parameters: \[0\]: Frequency value (Bco16 encoded).
pub const XPNDR_SET: &str = "XPNDR_SET";
