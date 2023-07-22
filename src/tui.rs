//! Tui handling module.
use crate::input::{KeyEvent, KeyEventObserver, MouseEvent, MouseEventObserver};
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};

type MutexElement = Arc<Mutex<dyn Element>>;
type MutexReactive = Arc<Mutex<dyn Reactive>>;

/// Used for all tui elements.
pub trait Element: Sync + Send {
    /// Prints/renders the element.
    fn print(&self);
    /// Gets the z pos.
    fn get_z(&self) -> u16;
}

/// Used for reactive tui elements.
pub trait Reactive: Sync + Send {
    /// Handles the key event.
    fn keyboard(&self, data: KeyEvent);
    /// Handles the mouse event.
    fn mouse(&self, data: MouseEvent);
    /// Gets the x pos.
    fn get_x(&self) -> u16;
    /// Gets the y pos.
    fn get_y(&self) -> u16;
    /// Gets width.
    fn get_width(&self) -> u16;
    /// Gets height.
    fn get_height(&self) -> u16;
}

/// Keyboard observer for element event handling.
pub struct TuiKeyObserver {
    /// Reference to the tui.
    tui: Arc<Mutex<ReactiveTUI>>,
}

impl KeyEventObserver for TuiKeyObserver {
    fn handle_key_event(&self, data: KeyEvent) {
        let mut tui_lock = self.tui.lock().unwrap();
        if data.code == tui_lock.selection_next {
            tui_lock.selected_element += 1;
            tui_lock.selected_element %= tui_lock.reactive_elements.len();
        } else if data.code == tui_lock.selection_previous {
            tui_lock.selected_element += tui_lock.reactive_elements.len() - 1;
            tui_lock.selected_element %= tui_lock.reactive_elements.len();
        }
        if tui_lock.reactive_elements.len() > 0 {
            tui_lock
                .reactive_elements
                .get(tui_lock.selected_element % tui_lock.reactive_elements.len())
                .unwrap()
                .lock()
                .unwrap()
                .keyboard(data);
        }
    }
}

/// Mouse observer for element event handling.
pub struct TuiMouseObserver {
    /// Reference to the tui.
    tui: Arc<Mutex<ReactiveTUI>>,
}

impl MouseEventObserver for TuiMouseObserver {
    fn handle_mouse_event(&self, data: MouseEvent) {
        let tui_lock = self.tui.lock().unwrap();
        for element in &tui_lock.reactive_elements {
            let element_lock = element.lock().unwrap();
            let event_x = data.x as u16;
            let event_y = data.y as u16;
            if event_x >= element_lock.get_x()
                && event_x <= element_lock.get_x() + element_lock.get_width()
                && event_y >= element_lock.get_y()
                && event_y <= element_lock.get_y() + element_lock.get_height()
            {
                element_lock.mouse(MouseEvent {
                    code: data.code,
                    x: event_x.saturating_sub(element_lock.get_x()) as u8,
                    y: event_y.saturating_sub(element_lock.get_y()) as u8,
                })
            }
        }
    }
}

/// Defines an rgb color.
#[derive(Copy, Clone)]
pub struct Color {
    /// Red value.
    pub r: u8,
    /// Green value.
    pub g: u8,
    /// Blue value.
    pub b: u8,
}

/// Returns ansi escape sequence to set color as foreground.
pub fn fg_color_to_string(color: Color) -> String {
    format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b)
}

/// Returns ansi escape sequence to set color as background.
pub fn bg_color_to_string(color: Color) -> String {
    format!("\x1b[48;2;{};{};{}m", color.r, color.g, color.b)
}

/// Forces the use of given fg/bg colors if not given uses terminal default
pub fn force_colors(fg_color: Option<Color>, bg_color: Option<Color>) -> String {
    format!(
        "\x1b[0m{}{}",
        if let Some(color) = bg_color {
            bg_color_to_string(color)
        } else {
            "".to_string()
        },
        if let Some(color) = fg_color {
            fg_color_to_string(color)
        } else {
            "".to_string()
        }
    )
}

/// Defines a pallet of line drawing characters.
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

/// Trait that defines behaviour shared between tui contexts.
pub trait TUI {
    /// Prints all elements.
    fn update(&self) {
        let mut sorted_elements = self.get_elements();
        sorted_elements.sort_by(|a, b| a.lock().unwrap().get_z().cmp(&b.lock().unwrap().get_z()));
        for element in sorted_elements {
            element.lock().unwrap().print();
            let _ = io::stdout().lock().flush();
        }
    }

    /// Returns a copy of it's elements.
    fn get_elements(&self) -> Vec<MutexElement>;
}

/// Contains main context of the reactive tui.
pub struct ReactiveTUI {
    #[allow(missing_docs)]
    pub elements: Vec<MutexElement>,
    /// Reactive element list to cycle through.
    pub reactive_elements: Vec<MutexReactive>,
    /// Index of the selected element.
    selected_element: usize,
    /// Key event used to increase the selection index.
    pub selection_next: u8,
    /// Key event used to reduce the selection index
    pub selection_previous: u8,
}

impl ReactiveTUI {
    #[allow(missing_docs)]
    pub fn new(
        selection_next: u8,
        selection_previous: u8,
    ) -> (
        Arc<Mutex<ReactiveTUI>>,
        Arc<TuiKeyObserver>,
        Arc<TuiMouseObserver>,
    ) {
        let tui = Arc::new(Mutex::new(ReactiveTUI {
            elements: Vec::new(),
            reactive_elements: Vec::new(),
            selected_element: 0,
            selection_next,
            selection_previous,
        }));
        let key_observer = Arc::new(TuiKeyObserver { tui: tui.clone() });
        let mouse_observer = Arc::new(TuiMouseObserver { tui: tui.clone() });
        (tui, key_observer, mouse_observer)
    }
}

impl TUI for ReactiveTUI {
    fn get_elements(&self) -> Vec<MutexElement> {
        self.elements.clone()
    }
}

/// Contains main context of the static tui.
pub struct StaticTUI {
    #[allow(missing_docs)]
    pub elements: Vec<MutexElement>,
}

impl StaticTUI {
    #[allow(missing_docs)]
    pub fn new() -> Arc<Mutex<StaticTUI>> {
        let tui = Arc::new(Mutex::new(StaticTUI {
            elements: Vec::new(),
        }));
        tui
    }
}

impl TUI for StaticTUI {
    fn get_elements(&self) -> Vec<MutexElement> {
        self.elements.clone()
    }
}
