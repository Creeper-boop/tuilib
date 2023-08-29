//! Defines simple tui elements.

use crate::tui::{force_colors, Color, Element, LineSet};
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
    /// Element visibility. <https://docs.unity3d.com/ScriptReference/Behaviour-enabled.html>
    pub enabled: bool,
    /// Text content.
    pub text: String,
}

impl Element for Text {
    /// Prints content at the coordinates, splits into multiple lines at '\n'.
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
    /// Foreground color.
    pub fg_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Element visibility. <https://docs.unity3d.com/ScriptReference/Behaviour-enabled.html>
    pub enabled: bool,
    /// Text content.
    pub text: String,
}

impl Element for TextBox {
    /// Prints the string contents, ignoring '\n'.
    /// Wraps at ' ' or if unable in the middle of words.
    fn print(&self) {
        let text = self.text.replace('\n', "");
        let mut text: Vec<&str> = text.split(' ').collect();
        for i in 0..self.height {
            // todo this shit is pain
            //  what tf do we do if the text length is to long?
            //   just dont render the rest of the text and make it a scrollable box
            //   indicate it and ignore the fact the user cant see the rest? --> currently implemented action
            //   make the element reactive and still ignore the fact the user cant see it all do hower show the entire text scrollable at the bottom of the screen/when the element is selected
            let mut line = String::new();
            while line.len() < self.width as usize {
                if text.len() == 0 {
                    break;
                }
                if line.len() == 0 {
                    if line.len() + text[0].len() < self.width as usize {
                        line += text[0];
                        text.remove(0);
                    } else {
                        if text[0].chars().nth(self.width as usize - 1).unwrap() == '-' {
                            line += &text[0].get(0..self.width as usize).unwrap();
                            text[0] = &text[0][self.width as usize..text[0].len() - 1];
                        } else {
                            line = line + &text[0].get(0..self.width as usize - 1).unwrap() + "-"
                        }
                        break;
                    }
                } else {
                    if line.len() + 1 + text[0].len() < self.width as usize {
                        line = line + " " + text[0];
                        text.remove(0);
                    } else {
                        break;
                    }
                }
            }
            print!(
                "\x1b[{};{}H{}{}{}\x1b[0m",
                self.y + i,
                self.x,
                force_colors(self.fg_color, self.bg_color),
                line,
                " ".repeat((self.width as usize).saturating_sub(line.len()))
            );
        }
    }

    fn get_z(&self) -> u16 {
        self.z
    }
}
