use std::fmt::Debug;

use crate::bit_set;

/// Debugging helper that prints a bit-mask as ASCII characters.
///
/// For Example:
///
/// ```
/// let s = format!("{:?}", BitMask::dbg(0b_00011101, 8));
/// assert_eq!(s, "...###.#");
/// ```
pub struct BitMask {
    mask: u32,
    width: usize,
    chars: [char; 2],
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Gray,
    Green,
    White,
}

/// Debugging helper that, given a bit-mask, prints a comma-separated list of the indices of the bits that are set in
/// it **indexed in left-to-right reading order.**
///
/// For example:
///
/// ```
/// let s = format!("{:?}", BitPositions::dbg(0b_010110, 6));
/// assert_eq!(s, "1,3,4")
/// ```
pub struct BitPositions {
    mask: u32,
    width: usize,
}

impl BitMask {
    pub const fn dbg(mask: u32, width: usize) -> Self {
        Self {
            mask,
            width,
            chars: ['0', '1'],
            color: Color::Gray,
        }
    }

    pub const fn chars(self, c0: char, c1: char) -> Self {
        Self { chars: [c0, c1], ..self }
    }

    pub const fn red(self) -> Self {
        Self { color: Color::Red, ..self }
    }

    pub const fn green(self) -> Self {
        Self { color: Color::Green, ..self }
    }

    pub const fn white(self) -> Self {
        Self { color: Color::White, ..self }
    }
}

impl BitPositions {
    pub const fn dbg(mask: u32, width: usize) -> Self {
        Self { mask, width }
    }
}

const ANSI_RED: &str = "\x1b[38;5;9m";
const ANSI_GRAY: &str = "\x1b[38;5;7m"; // Technically the non-bright "white"
const ANSI_GREEN: &str = "\x1b[38;5;10m";
const ANSI_WHITE: &str = "\x1b[38;5;15m"; // Technically "bright" white
const ANSI_BLACK: &str = "\x1b[38;5;238m"; // Technically dark gray
const ANSI_RESET: &str = "\x1b[0m";

impl Debug for BitMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [char0, char1] = self.chars;
        let ansi_color = match self.color {
            Color::Red => ANSI_RED,
            Color::Gray => ANSI_GRAY,
            Color::Green => ANSI_GREEN,
            Color::White => ANSI_WHITE,
        };

        for i in 0..self.width {
            if bit_set(self.mask, i, self.width) {
                write!(f, "{ansi_color}{char1}")?;
            } else {
                write!(f, "{ANSI_BLACK}{char0}")?;
            }
        }
        write!(f, "{ANSI_RESET}")?;

        Ok(())
    }
}

impl Debug for BitPositions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut need_comma = false;
        for i in 0..self.width {
            // Is the i'th bit set?
            if bit_set(self.mask, i, self.width) {
                match need_comma {
                    true => write!(f, ",")?,
                    false => need_comma = true,
                }
                write!(f, "{i}")?;
            }
        }

        Ok(())
    }
}
