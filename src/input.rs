//! Input handling module.

use libc::{ioctl, pid_t, TIOCGWINSZ};
use nix::pty::Winsize;
use nix::sys::termios;
use nix::sys::termios::Termios;
use nix::unistd::Pid;
use signal_hook::consts::signal::{SIGHUP, SIGINT, SIGQUIT, SIGTERM, SIGWINCH};
use signal_hook::iterator::Signals;
use std::io::{Read, Write};
use std::process::id;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use std::{io, thread};

use crate::input_callbacks::*;
use crate::input_observers::*;

/// Event debugging struct.
/// Contains the last key and mouse events.
pub struct TuiDebug {
    last_key_event: KeyEvent,
    last_mouse_event: MouseEvent,
}

/// Describes one key event.
#[derive(Copy, Clone, PartialEq)]
pub struct KeyEvent {
    /// Represents event type: key presses, releases or system hotkeys.
    pub code: u8,
}

/// Structs that implement this trait can be used to attach functions to key events.
pub trait KeyEventObserver {
    /// Called each time a key event is received, after binding to input.
    fn handle_key_event(&self, data: KeyEvent);
}

/// Describes one mouse event.
#[derive(Copy, Clone, PartialEq)]
pub struct MouseEvent {
    /// Represents event type: mouse button presses and releases, movements, drags and scrolling.
    pub code: u8,
    /// The x coordinate of the event from 1 to 223
    pub x: u8,
    /// The y coordinate of the event from 1 to 223
    pub y: u8,
}

/// Structs that implement this trait can be used to attach functions to mouse events.
pub trait MouseEventObserver {
    /// Called each time a mouse event is received, after binding to input.
    fn handle_mouse_event(&self, data: MouseEvent);
}

/// Contains main context of the input handler.
pub struct Input {
    /// Optionally contains debug info.
    ///
    /// See [TuiDebug].
    debug: Option<TuiDebug>,
    /// Contains the terminal emulator state before enabling raw input mode.
    return_state: Termios,
    /// Multithreaded receiver for emulator input events.
    input_rx: Receiver<[u8; 1]>,
    /// Used to listen to system signals such as SIGQUIT.
    sys_signals: Signals,
    /// The terminal emulator width.
    width: u16,
    /// The terminal emulator height.
    height: u16,
    /// All observers to notify of key events.
    ///
    /// See [KeyEventObserver].
    key_observers: Vec<Arc<dyn KeyEventObserver>>,
    /// All observers to notify of mouse events.
    ///
    /// See [MouseEventObserver].
    mouse_observers: Vec<Arc<dyn MouseEventObserver>>,
}

/// Enables the emulator raw mode, returns the previous state
fn set_raw_mode() -> Termios {
    let mut tio = termios::tcgetattr(0).expect("Unable to get terminal attribute!");
    let old = tio.clone();
    termios::cfmakeraw(&mut tio);
    termios::tcsetattr(0, termios::SetArg::TCSANOW, &tio).unwrap();
    old
}

/// Set mode of terminal emulator.
fn set_mode(attr: Termios) {
    termios::tcsetattr(0, termios::SetArg::TCSANOW, &attr).unwrap();
}

/// Returns the current emulator size.
fn get_size() -> (u16, u16) {
    let mut winsize = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    unsafe { ioctl(1, TIOCGWINSZ, &mut winsize) };
    (winsize.ws_row, winsize.ws_col)
}

/// Sends the system signal for reloading, same as ctrl + l.
pub fn reload() {
    nix::sys::signal::kill(Pid::from_raw(id() as pid_t), nix::sys::signal::SIGWINCH).unwrap();
}

/// Sends the system signal for quit, same as ctrl + c.
pub fn exit() {
    nix::sys::signal::kill(Pid::from_raw(id() as pid_t), nix::sys::signal::SIGINT).unwrap();
}

/// Prints "+" at the given mouse coordinates.
pub fn debug_pos(x: u8, y: u8) {
    print!("\x1b[{};{}H+", y, x);
}

impl Input {
    #[allow(missing_docs)] // UwU
    pub fn new(debug: bool) -> Input {
        let (height, width) = get_size();

        let (input_tx, input_rx) = mpsc::channel();
        thread::spawn(move || loop {
            let mut buffer = [0u8; 1];
            io::stdin().lock().read_exact(&mut buffer).unwrap();
            input_tx.send(buffer).unwrap();
        });

        let mut input = Input {
            debug: if debug {
                Some(TuiDebug {
                    last_key_event: KeyEvent { code: 0 },
                    last_mouse_event: MouseEvent {
                        code: 0,
                        x: 0,
                        y: 0,
                    },
                })
            } else {
                None
            },
            return_state: set_raw_mode(),
            input_rx,
            sys_signals: Signals::new(&[SIGWINCH, SIGTERM, SIGINT, SIGQUIT, SIGHUP]).unwrap(),
            width,
            height,
            key_observers: Vec::new(),
            mouse_observers: Vec::new(),
        };
        input.key_observers.push(Arc::new(ExitObserver {}));
        input.key_observers.push(Arc::new(ReloadObserver {}));

        if debug {
            input.mouse_observers.push(Arc::new(DebugObserver {}));
        }

        print!("\x1b[0m\x1b[H\x1b[J\x1b[?25l\x1b[?1003h");

        input
    }

    /// Handles system signals.
    pub fn handle_signals(&mut self) {
        for signal in self.sys_signals.pending() {
            match signal {
                SIGWINCH => {
                    (self.height, self.width) = get_size();
                    print!("\x1b[0m\x1b[H\x1b[J");
                }
                SIGTERM | SIGINT | SIGQUIT | SIGHUP => {
                    set_mode(self.return_state.clone());
                    print!("\x1b[0m\x1b[H\x1b[J\x1b[?25h\x1b[?1003l");
                    let _ = io::stdout().lock().flush();
                    std::process::exit(0);
                }
                _ => unreachable!(),
            }
        }
    }

    /// Handles input events.
    pub fn handle_input_events(&mut self, input_timeout: Duration) {
        while let Some(input) = self.input_rx.recv_timeout(input_timeout).ok() {
            if input[0] == MOUSE {
                let event = MouseEvent {
                    code: self
                        .input_rx
                        .recv_timeout(Duration::from_millis(1))
                        .expect("Mouse read error!")[0]
                        .clone(),
                    x: self
                        .input_rx
                        .recv_timeout(Duration::from_millis(1))
                        .expect("Mouse read error!")[0]
                        .clone()
                        - 32u8,
                    y: self
                        .input_rx
                        .recv_timeout(Duration::from_millis(1))
                        .expect("Mouse read error!")[0]
                        .clone()
                        - 32u8,
                };
                if self.debug.is_some() {
                    self.debug.as_mut().unwrap().last_mouse_event = event.clone();
                }
                for observer in &self.mouse_observers {
                    observer.handle_mouse_event(event);
                }
            } else {
                let event = KeyEvent {
                    code: input[0].clone(),
                };
                if self.debug.is_some() {
                    self.debug.as_mut().unwrap().last_key_event = event.clone();
                }
                for observer in &self.key_observers {
                    observer.handle_key_event(event);
                }
            }
        }
    }

    /// Prints debug information.
    pub fn debug(&self) {
        if self.debug.is_none() {
            return;
        }
        print!(
            "\x1b[{}Hw:{} h:{} key:{} mouse:{} x:{} y:{}\x1b[K",
            self.height,
            self.width,
            self.height,
            self.debug.as_ref().unwrap().last_key_event.code,
            self.debug.as_ref().unwrap().last_mouse_event.code,
            self.debug.as_ref().unwrap().last_mouse_event.x,
            self.debug.as_ref().unwrap().last_mouse_event.y,
        );
        let _ = io::stdout().lock().flush();
    }

    /// Main input loop.
    pub fn update(&mut self, input_timeout: Duration) {
        self.handle_signals();
        self.handle_input_events(input_timeout);
        self.debug();
    }
}
