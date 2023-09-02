//! Constants related to input events.
#![allow(missing_docs)]
// the definitive values for most generic mouse buttons
pub const MOUSE_LEFT_PRESS: u8 = 32;
pub const MOUSE_MIDDLE_PRESS: u8 = 33;
pub const MOUSE_RIGHT_PRESS: u8 = 34;
pub const MOUSE_RELEASE: u8 = 35;
// mouse drag events are just a repeat of the press events with 32 added to them
pub const MOUSE_LEFT_DRAG: u8 = 64;
pub const MOUSE_MIDDLE_DRAG: u8 = 65;
pub const MOUSE_RIGHT_DRAG: u8 = 66;
pub const MOUSE_MOVE: u8 = 67;
// designator for the start of mouse events
pub const MOUSE_EVENT_START: u8 = 77;
// designator for the end of mouse events currently unused
pub const MOUSE_EVENT_END: u8 = 91;
// Additional mouse buttons should still follow the above click to drag relation,
// however the mentioned buttons can have wierd values: 160 and 161 observed as the 4th and 5th
// button.

// My suspicions are that additional mouse buttons take values from 160 - 191 with their
// respective drag values being from 192 - 224. Considering the entire code range goes from
// 0 to 255 with it being relayed as a byte. I cannot confirm the hypothesis as mice with 31
// additional buttons arent available as of writing this.

/// event code for ctrl + c
pub const KILL: u8 = 3;
/// event code for ctrl + l
pub const RELOAD: u8 = 12;
// action keys
pub const ENTER: u8 = 13;
// uppercase letter keycoeds on a german qwertz keyboard
pub const UPPERCASE_A: u8 = 65;
pub const UPPERCASE_B: u8 = 66;
pub const UPPERCASE_C: u8 = 67;
pub const UPPERCASE_D: u8 = 68;
pub const UPPERCASE_E: u8 = 69;
pub const UPPERCASE_F: u8 = 70;
pub const UPPERCASE_G: u8 = 71;
pub const UPPERCASE_H: u8 = 72;
pub const UPPERCASE_I: u8 = 73;
pub const UPPERCASE_J: u8 = 74;
pub const UPPERCASE_K: u8 = 75;
pub const UPPERCASE_L: u8 = 76;
pub const UPPERCASE_M: u8 = 77;
pub const UPPERCASE_N: u8 = 78;
pub const UPPERCASE_O: u8 = 79;
pub const UPPERCASE_P: u8 = 80;
pub const UPPERCASE_Q: u8 = 81;
pub const UPPERCASE_R: u8 = 82;
pub const UPPERCASE_S: u8 = 83;
pub const UPPERCASE_T: u8 = 84;
pub const UPPERCASE_U: u8 = 85;
pub const UPPERCASE_V: u8 = 86;
pub const UPPERCASE_W: u8 = 87;
pub const UPPERCASE_X: u8 = 88;
pub const UPPERCASE_Y: u8 = 89;
pub const UPPERCASE_Z: u8 = 90;
// lowercase letter keycodes on a german quertz keyboard
pub const LOWERCASE_A: u8 = 97;
pub const LOWERCASE_B: u8 = 98;
pub const LOWERCASE_C: u8 = 99;
pub const LOWERCASE_D: u8 = 100;
pub const LOWERCASE_E: u8 = 101;
pub const LOWERCASE_F: u8 = 102;
pub const LOWERCASE_G: u8 = 103;
pub const LOWERCASE_H: u8 = 104;
pub const LOWERCASE_I: u8 = 105;
pub const LOWERCASE_J: u8 = 106;
pub const LOWERCASE_K: u8 = 107;
pub const LOWERCASE_L: u8 = 108;
pub const LOWERCASE_M: u8 = 109;
pub const LOWERCASE_N: u8 = 110;
pub const LOWERCASE_O: u8 = 111;
pub const LOWERCASE_P: u8 = 112;
pub const LOWERCASE_Q: u8 = 113;
pub const LOWERCASE_R: u8 = 114;
pub const LOWERCASE_S: u8 = 115;
pub const LOWERCASE_T: u8 = 116;
pub const LOWERCASE_U: u8 = 117;
pub const LOWERCASE_V: u8 = 118;
pub const LOWERCASE_W: u8 = 119;
pub const LOWERCASE_X: u8 = 120;
pub const LOWERCASE_Y: u8 = 121;
pub const LOWERCASE_Z: u8 = 122;
