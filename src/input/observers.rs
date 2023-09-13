//! Predefined event observers.
use std::sync::{Arc, RwLock};

use crate::{
    input::{debug_pos, exit, reload, KeyEvent, KeyEventObserver, MouseEvent, MouseEventObserver},
    tui::{ReactiveTUI, RwLockReactive},
};

use super::callbacks::{KILL, MOUSE_MOVE, RELOAD};

/// Handles ctrl + c.
pub struct ExitObserver {}

impl KeyEventObserver for ExitObserver {
    fn handle_key_event(&self, data: KeyEvent) {
        if data.code == KILL {
            exit()
        }
    }
}

/// Handles ctrl + l.
pub struct ReloadObserver {}

impl KeyEventObserver for ReloadObserver {
    fn handle_key_event(&self, data: KeyEvent) {
        if data.code == RELOAD {
            reload()
        }
    }
}

/// Handles mouse debug.
///
/// See [debug_pos]
pub struct DebugObserver {}

impl MouseEventObserver for DebugObserver {
    fn handle_mouse_event(&self, data: MouseEvent) {
        if data.code == MOUSE_MOVE {
            debug_pos(data.x, data.y)
        }
    }
}

/// Keyboard observer for element event handling.
pub struct TuiKeyObserver {
    /// Reference to the tui.
    pub tui: Arc<RwLock<ReactiveTUI>>,
}

impl KeyEventObserver for TuiKeyObserver {
    fn handle_key_event(&self, data: KeyEvent) {
        let mut tui_write = self.tui.write().unwrap();
        let reactive_elements: Vec<RwLockReactive> = tui_write
            .reactive_elements
            .iter()
            .filter(|e| e.read().unwrap().get_enabled())
            .map(|e| e.clone())
            .collect();
        if reactive_elements.len() > 0 {
            for element in tui_write.reactive_elements.clone() {
                element.write().unwrap().set_selected(false);
            }
            if data.code == tui_write.selection_next {
                tui_write.selected_element += 1;
                tui_write.selected_element %= reactive_elements.len();
            } else if data.code == tui_write.selection_previous {
                tui_write.selected_element += tui_write.reactive_elements.len() - 1;
                tui_write.selected_element %= tui_write.reactive_elements.len();
            }
            let mut selected_element_write = reactive_elements
                .get(tui_write.selected_element % reactive_elements.len())
                .unwrap()
                .write()
                .unwrap();
            selected_element_write.keyboard(data);
            selected_element_write.set_selected(true);
        } else {
            tui_write.selected_element = 0
        }
    }
}

/// Mouse observer for element event handling.
pub struct TuiMouseObserver {
    /// Reference to the tui.
    pub tui: Arc<RwLock<ReactiveTUI>>,
}

impl MouseEventObserver for TuiMouseObserver {
    fn handle_mouse_event(&self, data: MouseEvent) {
        let tui_read = self.tui.read().unwrap();
        for element in &tui_read.reactive_elements {
            let element_lock = element.read().unwrap();
            let event_x = data.x as u16;
            let event_y = data.y as u16;
            if event_x >= element_lock.get_x()
                && event_x < element_lock.get_x() + element_lock.get_width()
                && event_y >= element_lock.get_y()
                && event_y < element_lock.get_y() + element_lock.get_height()
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
