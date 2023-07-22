//! Color constants.
#![allow(missing_docs)]
use crate::tui::Color;

pub const ORANGE: Color = Color {
    r: 0xFF,
    g: 0xB0,
    b: 0x00,
};

pub const GREY: Color = Color {
    r: 0x1F,
    g: 0x1F,
    b: 0x1F,
};
/*
HashMap<&str, Color> = HashMap::from([
    (
        "fg",
        Color {
            r: 0xFF,
            g: 0xB0,
            b: 0x00,
        },
    ),
    (
        "fg_75",
        Color {
            r: 0xCC,
            g: 0x91,
            b: 0x0D,
        },
    ),
    (
        "fg_50",
        Color {
            r: 0x9A,
            g: 0x73,
            b: 0x1B,
        },
    ),
    (
        "fg_dark",
        Color {
            r: 0x35,
            g: 0x35,
            b: 0x35,
        },
    ),
    (
        "bg",
        Color {
            r: 0x1F,
            g: 0x1F,
            b: 0x1F,
        },
    ),
    (
        "bg_light",
        Color {
            r: 0x35,
            g: 0x35,
            b: 0x35,
        },
    ),
    (
        "bg_select",
        Color {
            r: 0xFF,
            g: 0xCC,
            b: 0x00,
        },
    ),
]);
*/
