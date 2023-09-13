use crate::{elements::wrapping_print, tui::Element, Color};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Tui element that renders wrapping text inside a box.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct TextBox {
    /// X position.
    pub x: u16,
    /// Y position.
    pub y: u16,
    /// Z position/printing priority.
    pub z: u16,
    /// Box width.
    pub width: u16,
    /// Box height.
    pub height: u16,
    /// Text color.
    pub text_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Element visibility. <https://docs.unity3d.com/ScriptReference/Behaviour-enabled.html>
    pub visible: bool,
    /// Text content.
    pub text: String,
}

impl Element for TextBox {
    fn print(&self) {
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
