//! Defines simple tui elements.

use crate::tui::{force_colors, Color, Element, LineSet};

/// Tui element that renders text.
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
    /// Element visibility. <https://docs.unity3d.com/ScriptReference/Behaviour-enabled.html>
    pub enabled: bool,
    /// Text content.
    pub text: String,
}

impl Element for Text {
    /// Prints content at the coordinates, doesn't split into multiple lines.
    fn print(&self) {
        if self.enabled {
            let lines: Vec<&str> = self.text.split('\n').collect();
            for i in 0..lines.len() {
                print!(
                    "\x1b[{};{}H{}{}\x1b[0m",
                    self.y + i as u16,
                    self.x,
                    force_colors(self.text_color, self.bg_color),
                    lines.get(i).unwrap_or(&"")
                );
            }
        }
    }

    fn get_z(&self) -> u16 {
        self.z
    }
}

/// Tui element that renders a box outline around an area.
pub struct Box {
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
    /// Foreground color.
    pub line_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Element visibility. <https://docs.unity3d.com/ScriptReference/Behaviour-enabled.html>
    pub enabled: bool,
    /// Set of line drawing characters to use.
    pub line_set: LineSet,
}

impl Element for Box {
    fn print(&self) {
        print!(
            "\x1b[{};{}H{}{}{}{}",
            self.y,
            self.x,
            force_colors(self.line_color, self.bg_color),
            self.line_set.top_left,
            self.line_set
                .horizontal
                .to_string()
                .repeat(self.width.saturating_sub(2) as usize),
            self.line_set.top_right
        );
        for i in 1..self.height {
            print!(
                "\x1b[{};{}H{}{}{}",
                self.y + i,
                self.x,
                self.line_set.vertical,
                " ".repeat(self.width.saturating_sub(2) as usize),
                self.line_set.vertical,
            )
        }
        print!(
            "\x1b[{};{}H{}{}{}\x1b[0m",
            self.y + self.height,
            self.x,
            self.line_set.bottom_left,
            self.line_set
                .horizontal
                .to_string()
                .repeat(self.width.saturating_sub(2) as usize),
            self.line_set.bottom_right
        )
    }

    fn get_z(&self) -> u16 {
        self.z
    }
}
