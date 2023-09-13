use crate::{
    input::{KeyAction, MouseAction},
    tui::Reactive,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Tui element that defines a simple interactable element.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct Interactable {
    /// X position.
    pub x: u16,
    /// Y position.
    pub y: u16,
    /// Interactable width.
    pub width: u16,
    /// Interactable height.
    pub height: u16,
    /// Action called upon mouse interaction.
    #[cfg_attr(feature = "serde", serde(skip_deserializing, skip_serializing))]
    pub mouse_action: MouseAction,
    /// Action called upon key interaction.
    #[cfg_attr(feature = "serde", serde(skip_deserializing, skip_serializing))]
    pub keyboard_action: KeyAction,
    /// Element selection.
    pub selected: bool,
    /// Element functonality.
    pub enabled: bool,
}

impl Reactive for Interactable {
    fn keyboard(&self, data: crate::input::KeyEvent) {
        (self.keyboard_action.0)(data)
    }

    fn mouse(&self, data: crate::input::MouseEvent) {
        (self.mouse_action.0)(data)
    }

    fn get_x(&self) -> u16 {
        self.x
    }

    fn get_y(&self) -> u16 {
        self.y
    }

    fn get_width(&self) -> u16 {
        self.width
    }

    fn get_height(&self) -> u16 {
        self.height
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    fn get_enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
