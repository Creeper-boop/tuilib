//! Defines element tree and all of its requirements.

use crate::tui;
use crate::tui::force_colors;
use std::collections::HashMap;

/// Tui element that renders elements in a tree like fashion.
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
    pub element_color: Option<tui::Color>,
    /// Line color.
    pub line_color: Option<tui::Color>,
    /// Background color.
    pub bg_color: Option<tui::Color>,
    /// Element visibility. <https://docs.unity3d.com/ScriptReference/Behaviour-enabled.html>
    pub enabled: bool,
}

/// Defines an icon.
#[derive(Debug)]
pub struct Icon {
    /// Icon color.
    pub color: Option<tui::Color>,
    /// Unicode symbol for the icon.
    pub char: char,
}

impl Icon {
    /// Returns string representation of the icon.
    pub fn to_string(&self) -> String {
        (if let Some(color) = self.color {
            tui::fg_color_to_string(color)
        } else {
            "".to_string()
        }) + &self.char.to_string()
    }
}

/// Line set used for element trees.
#[derive(Debug)]
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
    /// Character symbolising vertical continuation
    vertical_continuation: char,
}

/// Defines a part of the element tree, either an element or a folder with nested parts.
#[derive(Debug)]
pub enum Part {
    #[allow(missing_docs)]
    Element(Element),
    #[allow(missing_docs)]
    Folder(Folder),
}

/// Defines an element tree element.
#[derive(Debug)]
pub struct Element {
    /// Element name.
    pub name: String,
    /// Element icon id.
    pub icon_id: String,
}

/// Defines an element tree folder.
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
        element_color: Option<tui::Color>,
        line_color: Option<tui::Color>,
        bg_color: Option<tui::Color>,
    ) -> Vec<String> {
        let mut rows: Vec<String> = Vec::new();
        for i in 0..self.children.len() {
            rows.push(
                force_colors(line_color, bg_color)
                    + &if i == 0 && i == self.children.len() - 1 {
                        &line_set.top_and_only
                    } else if i == 0 {
                        &line_set.top_entry
                    } else if i == self.children.len() - 1 {
                        &line_set.bottom_entry
                    } else {
                        &line_set.middle_entry
                    }
                    .to_string()
                    + &if let Some(icon) = match self.children.get(i.clone()).unwrap() {
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
                    }
                    + &force_colors(line_color, bg_color)
                    + &line_set.horizontal_buffer.to_string()
                    + &force_colors(element_color, bg_color)
                    + match self.children.get(i.clone()).unwrap() {
                        Part::Element(element) => &element.name,
                        Part::Folder(folder) => &folder.name,
                    },
            );
            match self.children.get(i.clone()).unwrap() {
                Part::Element(_) => {}
                Part::Folder(folder) => {
                    rows.extend(folder.expand(line_set, icon_map, element_color, line_color, bg_color).iter().map(|e| if i == self.children.len().saturating_sub(1) {
                        " ".to_string()
                    } else {
                        line_set.vertical_buffer.to_string()
                    } + e))
                }
            }
        }
        rows
    }
}

impl tui::Element for ElementTree {
    fn print(&self) {
        // todo make it actually work
        let lines = self.elements.expand(
            &self.line_set,
            &self.icon_map,
            self.element_color,
            self.line_color,
            self.bg_color,
        );
        for i in 0..self.height {
            let mut line = String::new();
            line = lines.get(i as usize).unwrap_or(&line).clone();
            if i == self.height - 1 {
                line = line.replace(
                    self.line_set.vertical_buffer,
                    &self.line_set.vertical_continuation.to_string(),
                );
                if line.contains(self.line_set.middle_entry) {
                    line = line
                        .split(self.line_set.middle_entry)
                        .next()
                        .unwrap()
                        .to_string()
                        + &self.line_set.vertical_buffer.to_string();
                }
            }
            print!("\x1b[{};{}H{}", self.y + i, self.x, line,)
        }
    }

    fn get_z(&self) -> u16 {
        self.z
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
    vertical_continuation: '┋',
};
