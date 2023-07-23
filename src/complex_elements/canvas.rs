//! Defines canvas and all of its requirements.

use crate::tui;
use crate::tui::{bg_color_to_string, fg_color_to_string, force_colors};

/// Tui element that renders elements on a limited plane.
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
    pub elements: Vec<Element>,
    /// Default element color.
    pub element_color: Option<tui::Color>,
    /// Background fill color.
    pub bg_color: Option<tui::Color>,
    /// Element visibility. <https://docs.unity3d.com/ScriptReference/Behaviour-enabled.html>
    pub enabled: bool,
}

/// Element rendered on a canvas
#[derive(Clone)]
pub struct Element {
    /// X position.
    pub x: isize,
    /// Y position.
    pub y: isize,
    /// Element look as String.
    ///
    /// For multi lined elements use \n as indication of a new line.
    pub look: String,
    /// Element look color.
    pub fg_color: Option<tui::Color>,
    /// Element background color.
    pub bg_color: Option<tui::Color>,
}

impl tui::Element for Canvas {
    fn print(&self) {
        let mut canvas = vec![vec![" ".to_string(); self.width as usize]; self.height as usize];
        for element in self.elements.clone() {
            let mut element_grid: Vec<Vec<char>> = Vec::new();
            for row in element.look.split('\n') {
                element_grid.push(row.chars().collect());
            }
            for y in 0..element_grid.len() {
                if element.y.saturating_add(y as isize) >= 0
                    && element.y.saturating_add(y as isize) < self.height as isize
                {
                    for x in 0..element_grid[y].len() {
                        if element.x.saturating_add(x as isize) >= 0
                            && element.x.saturating_add(x as isize) < self.width as isize
                        {
                            canvas[element.y.saturating_add(y as isize) as usize]
                                [element.x.saturating_add(x as isize) as usize] =
                                if let Some(color) = element.fg_color {
                                    fg_color_to_string(color)
                                } else {
                                    "".to_string()
                                } + &if let Some(color) = element.bg_color {
                                    bg_color_to_string(color)
                                } else {
                                    "".to_string()
                                } + &*element_grid[y][x].to_string()
                                    + &if let Some(color) = self.element_color {
                                        fg_color_to_string(color)
                                    } else {
                                        "".to_string()
                                    }
                                    + &if let Some(color) = self.bg_color {
                                        bg_color_to_string(color)
                                    } else {
                                        "".to_string()
                                    };
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
                canvas[i].iter().map(|e| e.to_string()).collect::<String>()
            )
        }
    }

    fn get_z(&self) -> u16 {
        self.z
    }
}
