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
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

use crate::input_callbacks::*;
use crate::input_observers::*;

/// Event debugging struct.
/// Contains the last key and mouse events.
#[derive(Debug)]
pub struct TuiDebug {
    last_key_event: KeyEvent,
    last_mouse_event: MouseEvent,
    last_mouse_readout: [u8; 3],
    width: u16,
    height: u16,
}

/// Enum that describes an event.
pub enum Event {
    /// Event containing a KeyEvent.
    KeyEvent(KeyEvent),
    /// Event containing a MouseEvent.
    MouseEvent(MouseEvent),
}

#[derive(Clone)]
/// Wrapper type for Fn(KeyEvent) callbacks.
pub struct KeyAction(pub Arc<dyn Fn(KeyEvent) + Send + Sync + 'static>);

impl Default for KeyAction {
    fn default() -> Self {
        Self(Arc::new(move |_: KeyEvent| {}))
    }
}

#[derive(Clone)]
/// Wrapper type for Fn(MouseEvent) callbacks.
pub struct MouseAction(pub Arc<dyn Fn(MouseEvent) + Send + Sync + 'static>);

impl Default for MouseAction {
    fn default() -> Self {
        Self(Arc::new(move |_: MouseEvent| {}))
    }
}

#[derive(Clone)]
/// Wrapper type for Fn(Event) callbacks.
pub struct Action(pub Arc<dyn Fn(Event) + Send + Sync + 'static>);

impl Default for Action {
    fn default() -> Self {
        Self(Arc::new(move |_: Event| {}))
    }
}

/// Describes one key event.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct KeyEvent {
    /// Represents event type: key presses, releases or system hotkeys.
    pub code: u8,
}

/// Structs that implement this trait can be used to attach functions to key events.
pub trait KeyEventObserver: Sync + Send {
    /// Called each time a key event is received, after binding to input.
    fn handle_key_event(&self, data: KeyEvent);
}

/// Describes one mouse event.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MouseEvent {
    /// Represents event type: mouse button presses and releases, movements, drags and scrolling.
    pub code: u8,
    /// The x coordinate of the event from 1 to 223
    pub x: u8,
    /// The y coordinate of the event from 1 to 223
    pub y: u8,
}

/// Structs that implement this trait can be used to attach functions to mouse events.
pub trait MouseEventObserver: Sync + Send {
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
    /// All observers to notify of key events.
    ///
    /// See [KeyEventObserver].
    pub key_observers: Arc<Mutex<Vec<Arc<dyn KeyEventObserver>>>>,
    /// All observers to notify of mouse events.
    ///
    /// See [MouseEventObserver].
    pub mouse_observers: Arc<Mutex<Vec<Arc<dyn MouseEventObserver>>>>,
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
        let (input_tx, input_rx) = mpsc::channel();
        thread::spawn(move || loop {
            let mut buffer = [0u8; 1];
            io::stdin().lock().read_exact(&mut buffer).unwrap();
            input_tx.send(buffer).unwrap();
        });

        let input = Input {
            debug: if debug {
                let (height, width) = get_size();
                Some(TuiDebug {
                    last_key_event: KeyEvent { code: 0 },
                    last_mouse_event: MouseEvent {
                        code: 0,
                        x: 0,
                        y: 0,
                    },
                    last_mouse_readout: [0u8; 3],
                    width,
                    height,
                })
            } else {
                None
            },
            return_state: set_raw_mode(),
            input_rx,
            sys_signals: Signals::new(&[SIGWINCH, SIGTERM, SIGINT, SIGQUIT, SIGHUP]).unwrap(),
            key_observers: Arc::new(Mutex::new(Vec::new())),
            mouse_observers: Arc::new(Mutex::new(Vec::new())),
        };
        input
            .key_observers
            .lock()
            .unwrap()
            .push(Arc::new(ExitObserver {}));
        input
            .key_observers
            .lock()
            .unwrap()
            .push(Arc::new(ReloadObserver {}));

        if debug {
            input
                .mouse_observers
                .lock()
                .unwrap()
                .push(Arc::new(DebugObserver {}));
        }

        print!("\x1b[0m\x1b[H\x1b[J\x1b[?25l\x1b[?1003h");

        input
    }

    /// Handles system signals.
    pub fn handle_signals(&mut self) {
        for signal in self.sys_signals.pending() {
            match signal {
                SIGWINCH => {
                    if let Some(debug) = self.debug.as_mut() {
                        (debug.height, debug.width) = get_size();
                    }
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
            let mut mouse = [0u8; 3];
            if input[0] == MOUSE_EVENT_START {
                mouse[0] = self
                    .input_rx
                    .recv_timeout(Duration::from_millis(1))
                    .unwrap_or([0u8])[0]
                    .clone();
                mouse[1] = self
                    .input_rx
                    .recv_timeout(Duration::from_millis(1))
                    .unwrap_or([0u8])[0]
                    .clone()
                    .saturating_sub(32u8);
                mouse[2] = self
                    .input_rx
                    .recv_timeout(Duration::from_millis(1))
                    .unwrap_or([0u8])[0]
                    .clone()
                    .saturating_sub(32u8);
                if self.debug.is_some() {
                    self.debug.as_mut().unwrap().last_mouse_readout = mouse.clone();
                }
            }
            if mouse[0] != 0 {
                let event = MouseEvent {
                    code: mouse[0],
                    x: mouse[1],
                    y: mouse[2],
                };
                if self.debug.is_some() {
                    self.debug.as_mut().unwrap().last_mouse_event = event.clone();
                }
                for observer in self.mouse_observers.lock().unwrap().iter() {
                    observer.handle_mouse_event(event);
                }
            } else {
                let event = KeyEvent {
                    code: input[0].clone(),
                };
                if self.debug.is_some() {
                    self.debug.as_mut().unwrap().last_key_event = event.clone();
                }
                for observer in self.key_observers.lock().unwrap().iter() {
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
            "\x1b[{}Hw:{} h:{} key:{} mouse:{} x:{} y:{} readout:{:?}\x1b[K",
            self.debug.as_ref().unwrap().height,
            self.debug.as_ref().unwrap().width,
            self.debug.as_ref().unwrap().height,
            self.debug.as_ref().unwrap().last_key_event.code,
            self.debug.as_ref().unwrap().last_mouse_event.code,
            self.debug.as_ref().unwrap().last_mouse_event.x,
            self.debug.as_ref().unwrap().last_mouse_event.y,
            self.debug.as_ref().unwrap().last_mouse_readout,
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
