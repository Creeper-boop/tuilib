use crate::{
    elements::wrapping_print,
    input::{Action, Event, KeyEvent, MouseEvent},
    tui::{Element, Reactive},
    Color,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Tui element that defines a simple Button.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct Button {
    /// X position.
    pub x: u16,
    /// Y position.
    pub y: u16,
    /// Z position/printing priority.
    pub z: u16,
    /// Interactable width.
    pub width: u16,
    /// Interactable height.
    pub height: u16,
    /// Foreground color.
    pub text_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Foreground color.
    pub selected_text_color: Option<Color>,
    /// Background color.
    pub selected_bg_color: Option<Color>,
    /// Action called upon interaction.
    #[cfg_attr(feature = "serde", serde(skip_deserializing, skip_serializing))]
    pub action: Action,
    /// Element selection.
    pub selected: bool,
    /// Element functonality.
    pub enabled: bool,
    /// Element visibility.
    pub visible: bool,
    /// Text content.
    pub text: String,
}

impl Reactive for Button {
    fn keyboard(&self, data: KeyEvent) {
        (self.action.0)(Event::KeyEvent(data))
    }

    fn mouse(&self, data: MouseEvent) {
        (self.action.0)(Event::MouseEvent(data))
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

impl Element for Button {
    fn print(&self) {
        if self.selected {
            wrapping_print(
                self.x,
                self.y,
                self.width,
                self.height,
                self.selected_text_color,
                self.selected_bg_color,
                self.text.clone(),
            );
        } else {
            wrapping_print(
                self.x,
                self.y,
                self.width,
                self.height,
                self.text_color,
                self.bg_color,
                self.text.clone(),
            );
        }
    }

    fn get_z(&self) -> u16 {
        self.z
    }

    fn get_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}
