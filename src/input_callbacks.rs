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
/// designator that makes the next three return codes designate mouse events
pub const MOUSE: u8 = 77;
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
