use crate::{elements::print, tui::Element, Color};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Tui element that renders text.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Text {
    /// X position.
    pub x: u16,
    /// Y position.
    pub y: u16,
    /// Z position/printing priority.
    pub z: u16,
    /// Foreground color.
    pub text_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Element visibility.
    pub visible: bool,
    /// Text content.
    pub text: String,
}

impl Element for Text {
    fn print(&self) {
        print(
            self.x,
            self.y,
            self.text_color,
            self.bg_color,
            self.text.clone(),
        );
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
