//! Predefined event observers.
use crate::input::{
    debug_pos, exit, reload, KeyEvent, KeyEventObserver, MouseEvent, MouseEventObserver,
};
use crate::input_callbacks::{KILL, MOUSE_MOVE, RELOAD};

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
