//! Line set constants.
#![allow(missing_docs)]
use crate::tui::LineSet;

pub const LINES_LIGHT: LineSet = LineSet {
    horizontal: '─',
    vertical: '│',
    top_left: '┌',
    top_right: '┐',
    bottom_left: '└',
    bottom_right: '┘',
};

pub const LINES_HEAVY: LineSet = LineSet {
    horizontal: '━',
    vertical: '┃',
    top_left: '┏',
    top_right: '┓',
    bottom_left: '┗',
    bottom_right: '┛',
};

pub const LINES_DOUBLE: LineSet = LineSet {
    horizontal: '═',
    vertical: '║',
    top_left: '╔',
    top_right: '╗',
    bottom_left: '╚',
    bottom_right: '╝',
};
