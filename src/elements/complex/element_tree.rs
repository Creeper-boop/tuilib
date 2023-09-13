//! Defines element tree and all of its requirements.

use crate::{fg_color_to_string, force_colors, tui, Color};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tui element that renders elements in a tree like fashion.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct ElementTree {
    /// X position.
    pub x: u16,
    /// Y position.
    pub y: u16,
    /// Z position/printing priority.
    pub z: u16,
    /// Element tree width.
    pub width: u16,
    /// Element tree height.
    pub height: u16,
    /// Maps icon ids to icons.
    pub icon_map: HashMap<String, Icon>,
    /// Line set for element tree.
    pub line_set: LineSet,
    /// Parts of the tree.
    pub elements: Folder,
    /// Element color.
    pub element_color: Option<Color>,
    /// Line color.
    pub line_color: Option<Color>,
    /// Background color.
    pub bg_color: Option<Color>,
    /// Element visibility.
    pub visible: bool,
}

/// Defines an icon.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct Icon {
    /// Icon color.
    pub color: Option<Color>,
    /// Unicode symbol for the icon.
    pub char: char,
}

impl Icon {
    /// Returns string representation of the icon.
    pub fn to_string(&self) -> String {
        (if let Some(color) = self.color {
            fg_color_to_string(color)
        } else {
            "".to_string()
        }) + &self.char.to_string()
    }
}

/// Line set used for element trees.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct LineSet {
    /// Character for the top entry.
    top_entry: char,
    /// Character for the middle entry.
    middle_entry: char,
    /// Character for the bottom entry.
    bottom_entry: char,
    /// Character for the top entry if it's the only one.
    top_and_only: char,
    /// Character for the vertical buffer.
    vertical_buffer: char,
    /// Character for the buffer between the entry symbol and name.
    horizontal_buffer: char,
    /// Character symbolising vertical continuation.
    vertical_continuation: char,
    /// Character symbolising horizontal continuation.
    horizontal_continuation: char,
}

/// Defines a part of the element tree, either an element or a folder with nested parts.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Part {
    #[allow(missing_docs)]
    Element(Element),
    #[allow(missing_docs)]
    Folder(Folder),
}

/// Defines an element tree element.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Element {
    /// Element name.
    pub name: String,
    /// Element icon id.
    pub icon_id: String,
}

/// Defines an element tree folder.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Folder {
    /// Element name.
    pub name: String,
    /// Icon id when closed.
    pub icon_closed_id: String,
    /// Icon id when open.
    pub icon_open_id: String,
    /// Whether the folder is open or closed.
    pub is_open: bool,
    /// Nested parts of the element tree.
    pub children: Vec<Part>,
}

impl Folder {
    /// Returns a vector of strings representing the structure of nested elements.
    fn expand(
        &self,
        line_set: &LineSet,
        icon_map: &HashMap<String, Icon>,
    ) -> Vec<(String, String, String, String)> {
        let mut rows: Vec<(String, String, String, String)> = Vec::new();
        for i in 0..self.children.len() {
            rows.push((
                if i == 0 && i == self.children.len() - 1 {
                    &line_set.top_and_only
                } else if i == 0 {
                    &line_set.top_entry
                } else if i == self.children.len() - 1 {
                    &line_set.bottom_entry
                } else {
                    &line_set.middle_entry
                }
                .to_string(),
                if let Some(icon) = match self.children.get(i.clone()).unwrap() {
                    Part::Element(element) => icon_map.get(&*element.icon_id),
                    Part::Folder(folder) => icon_map.get(if folder.is_open {
                        &*folder.icon_open_id
                    } else {
                        &*folder.icon_closed_id
                    }),
                } {
                    icon.to_string()
                } else {
                    "".to_string()
                },
                line_set.horizontal_buffer.to_string(),
                match self.children.get(i.clone()).unwrap() {
                    Part::Element(element) => element.name.clone(),
                    Part::Folder(folder) => folder.name.clone(),
                },
            ));
            match self.children.get(i.clone()).unwrap() {
                Part::Element(_) => {}
                Part::Folder(folder) => {
                    rows.extend(folder.expand(line_set, icon_map).iter().map(|e| {
                        (
                            if i == self.children.len().saturating_sub(1) {
                                " ".to_string()
                            } else {
                                line_set.vertical_buffer.to_string()
                            } + &e.0.clone(),
                            e.1.clone(),
                            e.2.clone(),
                            e.3.clone(),
                        )
                    }))
                }
            }
        }
        rows
    }
}

impl tui::Element for ElementTree {
    fn print(&self) {
        // todo make it actually work
        let lines = self.elements.expand(&self.line_set, &self.icon_map);
        for i in 0..self.height {
            let mut line = force_colors(None, self.bg_color);
            let mut line_parts = lines
                .get(i as usize)
                .unwrap_or(&(String::new(), String::new(), String::new(), String::new()))
                .clone();
            if line_parts.0.len() > 0 {
                if i == self.height - 1 {
                    line_parts = (
                        line_parts
                            .0
                            .replace(
                                self.line_set.vertical_buffer,
                                &self.line_set.vertical_continuation.to_string(),
                            )
                            .replace(
                                self.line_set.middle_entry,
                                &self.line_set.vertical_continuation.to_string(),
                            ),
                        String::new(),
                        String::new(),
                        String::new(),
                    );
                    line = force_colors(self.line_color, self.bg_color) + &line_parts.0 + "  ";
                } else {
                    if line_parts.0.chars().count() + 2 + line_parts.3.chars().count()
                        > self.width as usize
                    {
                        line_parts.3.truncate(
                            (self.width as usize).saturating_sub(line_parts.0.chars().count() + 3),
                        );
                        line_parts.3 += &self.line_set.horizontal_continuation.to_string();
                    }
                    line = force_colors(self.line_color, self.bg_color)
                        + &line_parts.0
                        + &line_parts.1
                        + &force_colors(self.line_color, self.bg_color)
                        + &line_parts.2
                        + &force_colors(self.element_color, self.bg_color)
                        + &line_parts.3
                }
            }
            print!(
                "\x1b[{};{}H{}{}\x1b[0m",
                self.y + i,
                self.x,
                line,
                " ".repeat(
                    (self.width as usize).saturating_sub(if line_parts.0.len() > 0 {
                        line_parts.0.chars().count() + 2 + line_parts.3.chars().count()
                    } else {
                        0
                    })
                )
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

#[allow(missing_docs)]
pub const OPEN_FOLDER: Icon = Icon {
    color: None,
    char: 'v',
};
#[allow(missing_docs)]
pub const CLOSED_FOLDER: Icon = Icon {
    color: None,
    char: '>',
};
#[allow(missing_docs)]
pub const SIMPLE_SET: LineSet = LineSet {
    top_entry: '┢',
    middle_entry: '┣',
    bottom_entry: '┗',
    top_and_only: '┕',
    vertical_buffer: '┃',
    horizontal_buffer: '╸',
    vertical_continuation: '┇',
    horizontal_continuation: '┅',
};
