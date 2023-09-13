//! Contains premade elements.

use crate::colors::{force_colors, Color};

pub mod complex;
pub mod simple;

/// Prints content at the coordinates, splits into multiple lines at '\n'
pub fn print(x: u16, y: u16, fg_color: Option<Color>, bg_color: Option<Color>, text: String) {
    let lines: Vec<&str> = text.split('\n').collect();
    for i in 0..lines.len() {
        print!(
            "\x1b[{};{}H{}{}\x1b[0m",
            y + i as u16,
            x,
            force_colors(fg_color, bg_color),
            lines.get(i).unwrap_or(&"")
        );
    }
}

/// Prints the string contents, ignoring '\n'.
/// Wraps at ' ' or if unable in the middle of words.
pub fn wrapping_print(
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    text: String,
) {
    let text = text.replace('\n', "");
    let mut text: Vec<&str> = text.split(' ').collect();
    for i in 0..height {
        // todo this shit is pain
        //  what tf do we do if the text length is to long?
        //   just dont render the rest of the text and make it a scrollable box
        //   indicate it and ignore the fact the user cant see the rest? --> currently implemented action
        //   make the element reactive and still ignore the fact the user cant see it all do hower show the entire text scrollable at the bottom of the screen/when the element is selected
        let mut line = String::new();
        while line.len() < width as usize {
            if text.len() == 0 {
                break;
            }
            if line.len() == 0 {
                if line.len() + text[0].len() < width as usize {
                    line += text[0];
                    text.remove(0);
                } else {
                    if text[0].chars().nth(width as usize - 1).unwrap() == '-' {
                        line += &text[0].get(0..width as usize).unwrap();
                        text[0] = &text[0][width as usize..text[0].len() - 1];
                    } else {
                        line = line + &text[0].get(0..width as usize - 1).unwrap() + "-"
                    }
                    break;
                }
            } else {
                if line.len() + 1 + text[0].len() < width as usize {
                    line = line + " " + text[0];
                    text.remove(0);
                } else {
                    break;
                }
            }
        }
        print!(
            "\x1b[{};{}H{}{}{}\x1b[0m",
            y + i,
            x,
            force_colors(fg_color, bg_color),
            line,
            " ".repeat((width as usize).saturating_sub(line.len()))
        );
    }
}
