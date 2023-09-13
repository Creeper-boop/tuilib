//! Color constants and functions.
#![allow(missing_docs)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Defines an rgb color.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Debug)]
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

pub const YELLOW: Color = Color {
    r: 0xFF,
    g: 0xCC,
    b: 0x00,
};

pub const ORANGE: Color = Color {
    r: 0xFF,
    g: 0xB0,
    b: 0x00,
};

pub const ORANGE_75: Color = Color {
    r: 0xCC,
    g: 0x91,
    b: 0x0D,
};

pub const ORANGE_50: Color = Color {
    r: 0x9A,
    g: 0x73,
    b: 0x1B,
};

pub const LIME: Color = Color {
    r: 0x33,
    g: 0xFF,
    b: 0x00,
};

pub const GREEN: Color = Color {
    r: 0x2D,
    g: 0xE0,
    b: 0x00,
};

pub const GREEN_75: Color = Color {
    r: 0x2F,
    g: 0xB5,
    b: 0x0D,
};

pub const GREEN_50: Color = Color {
    r: 0x31,
    g: 0x8B,
    b: 0x1B,
};

pub const WHITE: Color = Color {
    r: 0xF6,
    g: 0xF8,
    b: 0xFF,
};

pub const LIGHT_GREY: Color = Color {
    r: 0xDF,
    g: 0xE3,
    b: 0xED,
};

pub const LIGHT_GREY_75: Color = Color {
    r: 0xB4,
    g: 0xB7,
    b: 0xBF,
};

pub const LIGHT_GREY_50: Color = Color {
    r: 0x8A,
    g: 0x8C,
    b: 0x91,
};

pub const BLACK: Color = Color {
    r: 0x1F,
    g: 0x1F,
    b: 0x1F,
};

pub const GREY: Color = Color {
    r: 0x35,
    g: 0x35,
    b: 0x35,
};
