//! Tui handling module.
use std::io;
use std::io::Write;
use std::sync::{Arc, RwLock};

use crate::input::observers::{TuiKeyObserver, TuiMouseObserver};
use crate::input::{KeyEvent, MouseEvent};
pub type RwLockElement = Arc<RwLock<dyn Element>>;
pub type RwLockReactive = Arc<RwLock<dyn Reactive>>;

/// Used for all tui elements.
pub trait Element: Sync + Send {
    /// Prints/renders the element.
    fn print(&self);
    /// Gets the z pos.
    fn get_z(&self) -> u16;
    /// Get visibility.
    fn get_visible(&self) -> bool;
    /// Set visibility.
    fn set_visible(&mut self, visible: bool);
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
    /// Sets Selection.
    fn set_selected(&mut self, selected: bool);
    /// Get actionability. (if the element reacts to actions)
    fn get_enabled(&self) -> bool;
    /// Set actionability.
    fn set_enabled(&mut self, enabled: bool);
}

/// Element and reactive element group.
pub struct Group {
    /// All elements that are part of the group.
    pub elements: Vec<RwLockElement>,
    /// All reactive elements that are part of the group.
    pub reactive_elements: Vec<RwLockReactive>,
}

impl Group {
    /// Set visibility for all elements within a group.
    pub fn set_visibility(&self, visibility: bool) {
        for element in &self.elements {
            element.write().unwrap().set_visible(visibility);
        }
    }

    /// Set actionability for all elements within a group.
    pub fn set_enabled(&self, enabled: bool) {
        for element in &self.reactive_elements {
            element.write().unwrap().set_enabled(enabled);
        }
    }
}

/// Trait that defines behaviour shared between tui contexts.
pub trait TUI {
    /// Prints all elements.
    fn update(&self) {
        let mut sorted_elements = self.get_elements();
        sorted_elements.sort_by(|a, b| {
            let a_z = a.read().unwrap().get_z();
            let b_z = b.read().unwrap().get_z();
            a_z.cmp(&b_z)
        });
        for element in sorted_elements {
            let element_lock = element.read().unwrap();
            if element_lock.get_visible() {
                element_lock.print();
            }
            drop(element_lock);
            let _ = io::stdout().lock().flush();
        }
    }

    /// Returns a copy of it's elements.
    fn get_elements(&self) -> Vec<RwLockElement>;
}

/// Contains main context of the reactive tui.
pub struct ReactiveTUI {
    #[allow(missing_docs)]
    pub elements: Vec<RwLockElement>,
    /// Reactive element list to cycle through.
    pub reactive_elements: Vec<RwLockReactive>,
    /// Index of the selected element.
    pub selected_element: usize,
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
        Arc<RwLock<ReactiveTUI>>,
        Arc<TuiKeyObserver>,
        Arc<TuiMouseObserver>,
    ) {
        let tui = Arc::new(RwLock::new(ReactiveTUI {
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
    fn get_elements(&self) -> Vec<RwLockElement> {
        self.elements.clone()
    }
}

/// Contains main context of the static tui.
pub struct StaticTUI {
    #[allow(missing_docs)]
    pub elements: Vec<RwLockElement>,
}

impl StaticTUI {
    #[allow(missing_docs)]
    pub fn new() -> Arc<RwLock<StaticTUI>> {
        let tui = Arc::new(RwLock::new(StaticTUI {
            elements: Vec::new(),
        }));
        tui
    }
}

impl TUI for StaticTUI {
    fn get_elements(&self) -> Vec<RwLockElement> {
        self.elements.clone()
    }
}
