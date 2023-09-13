use crate::{force_colors, tui::Element, Color};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Tui element that renders a box outline around an area.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
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
    /// Element visibility.
    pub visible: bool,
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
        for i in 1..self.height - 1 {
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
            self.y + self.height - 1,
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

    fn get_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

/// Defines a pallet of line drawing characters.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct LineSet {
    /// Character for straight horizontal line.
    pub horizontal: char,
    /// Character for straight vertical line.
    pub vertical: char,
    /// Character for the top left corner of a box.
    pub top_left: char,
    /// Character for the top right corner of a box.
    pub top_right: char,
    /// Character for the bottom left corner of a box.
    pub bottom_left: char,
    /// Character for the bottom right corner of a box.
    pub bottom_right: char,
}

pub const LINES_LIGHT: LineSet = LineSet {
    horizontal: '─',
    vertical: '│',
    top_left: '┌',
    top_right: '┐',
    bottom_left: '└',
    bottom_right: '┘',
};

pub const LINES_HEAVY: LineSet = LineSet {
    horizontal: '━',
    vertical: '┃',
    top_left: '┏',
    top_right: '┓',
    bottom_left: '┗',
    bottom_right: '┛',
};

pub const LINES_DOUBLE: LineSet = LineSet {
    horizontal: '═',
    vertical: '║',
    top_left: '╔',
    top_right: '╗',
    bottom_left: '╚',
    bottom_right: '╝',
};
