//! Defines canvas and all of its requirements.

use crate::colors::{bg_color_to_string, fg_color_to_string, force_colors, Color};
use crate::input::{KeyAction, MouseAction};
use crate::tui::{self, Reactive};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

/// Tui element that renders elements on a limited plane.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct Canvas {
    /// X position.
    pub x: u16,
    /// Y position.
    pub y: u16,
    /// Z position/printing priority.
    pub z: u16,
    /// Canvas width.
    pub width: u16,
    /// Canvas height.
    pub height: u16,
    /// Parts of the tree.
    pub elements: Vec<Arc<RwLock<Element>>>,
    /// Default element color.
    pub element_color: Option<Color>,
    /// Background fill color.
    pub bg_color: Option<Color>,
    /// Action called upon mouse interaction.
    #[cfg_attr(feature = "serde", serde(skip_deserializing, skip_serializing))]
    pub mouse_action: MouseAction,
    /// Action called upon key interaction.
    #[cfg_attr(feature = "serde", serde(skip_deserializing, skip_serializing))]
    pub keyboard_action: KeyAction,
    /// Element visibility.
    pub visible: bool,
    /// Element selected.
    pub selected: bool,
    /// Element functionality.
    pub enabled: bool,
}

/// Element rendered on a canvas
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub struct Element {
    /// X position.
    pub x: isize,
    /// Y position.
    pub y: isize,
    /// Z position/printing priority
    pub z: u16,
    /// Element look as String.
    ///
    /// For multi lined elements use \n as indication of a new line.
    pub look: String,
    /// Element look color.
    pub fg_color: Option<Color>,
    /// Element background color.
    pub bg_color: Option<Color>,
}

impl tui::Element for Canvas {
    fn print(&self) {
        let mut canvas = vec![vec![" ".to_string(); self.width as usize]; self.height as usize];
        let mut sorted_elements = self.elements.clone();
        sorted_elements.sort_by(|a, b| {
            let a_z = a.read().unwrap().z;
            let b_z = b.read().unwrap().z;
            a_z.cmp(&b_z)
        });
        for element in sorted_elements {
            let mut element_grid: Vec<Vec<char>> = Vec::new();
            let element_read = element.read().unwrap();
            for row in element_read.look.split('\n') {
                element_grid.push(row.chars().collect());
            }
            for y in 0..element_grid.len() {
                if element_read.y.saturating_add(y as isize) >= 0
                    && element_read.y.saturating_add(y as isize) < self.height as isize
                {
                    for x in 0..element_grid[y].len() {
                        if element_read.x.saturating_add(x as isize) >= 0
                            && element_read.x.saturating_add(x as isize) < self.width as isize
                        {
                            let field = canvas[element_read.y.saturating_add(y as isize) as usize]
                                [element_read.x.saturating_add(x as isize) as usize]
                                .clone();
                            canvas[element_read.y.saturating_add(y as isize) as usize]
                                [element_read.x.saturating_add(x as isize) as usize] =
                                if let Some(color) = element_read.fg_color {
                                    fg_color_to_string(color)
                                } else {
                                    "".to_string()
                                } + &if let Some(color) = element_read.bg_color {
                                    bg_color_to_string(color)
                                } else if field.contains("\x1b[48;") {
                                    "\x1b[48;".to_string()
                                        + field
                                            .split_once("\x1b[48;")
                                            .unwrap()
                                            .1
                                            .split_once(";bm")
                                            .unwrap()
                                            .0
                                        + ";bm"
                                } else {
                                    "".to_string()
                                } + &*element_grid[y][x].to_string();
                        }
                    }
                }
            }
        }
        for i in 0..canvas.len() {
            print!(
                "\x1b[{};{}H{}{}\x1b[0m",
                self.y + i as u16,
                self.x,
                force_colors(self.element_color, self.bg_color),
                canvas[i]
                    .iter()
                    .map(|e| e.to_string()
                        + &if let Some(color) = self.element_color {
                            fg_color_to_string(color)
                        } else {
                            "".to_string()
                        }
                        + &if let Some(color) = self.bg_color {
                            bg_color_to_string(color)
                        } else {
                            "".to_string()
                        })
                    .collect::<String>()
            )
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

impl Reactive for Canvas {
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
