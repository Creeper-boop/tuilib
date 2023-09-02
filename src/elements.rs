//! Defines simple tui elements.

use crate::{
    input::{Action, Event, KeyAction, MouseAction},
    tui::{force_colors, Color, Element, LineSet, Reactive},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Prints the string contents, ignoring '\n'.
/// Wraps at ' ' or if unable in the middle of words.
fn wrapping_print(
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    text: String,
) {
    let text = text.replace('\n', "");
    let mut text: Vec<&str> = text.split(' ').collect();
    for i in 0..height {
        // todo this shit is pain
        //  what tf do we do if the text length is to long?
        //   just dont render the rest of the text and make it a scrollable box
        //   indicate it and ignore the fact the user cant see the rest? --> currently implemented action
        //   make the element reactive and still ignore the fact the user cant see it all do hower show the entire text scrollable at the bottom of the screen/when the element is selected
        let mut line = String::new();
        while line.len() < width as usize {
            if text.len() == 0 {
                break;
            }
            if line.len() == 0 {
                if line.len() + text[0].len() < width as usize {
                    line += text[0];
                    text.remove(0);
                } else {
                    if text[0].chars().nth(width as usize - 1).unwrap() == '-' {
                        line += &text[0].get(0..width as usize).unwrap();
                        text[0] = &text[0][width as usize..text[0].len() - 1];
                    } else {
                        line = line + &text[0].get(0..width as usize - 1).unwrap() + "-"
                    }
                    break;
                }
            } else {
                if line.len() + 1 + text[0].len() < width as usize {
                    line = line + " " + text[0];
                    text.remove(0);
                } else {
                    break;
                }
            }
        }
        print!(
            "\x1b[{};{}H{}{}{}\x1b[0m",
            y + i,
            x,
            force_colors(fg_color, bg_color),
            line,
            " ".repeat((width as usize).saturating_sub(line.len()))
        );
    }
}

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
    /// Prints content at the coordinates, splits into multiple lines at '\n'.
    fn print(&self) {
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
    fn keyboard(&self, data: crate::input::KeyEvent) {
        (self.action.0)(Event::KeyEvent(data))
    }

    fn mouse(&self, data: crate::input::MouseEvent) {
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
