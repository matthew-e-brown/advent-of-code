use std::fmt::Debug;

use crate::bit_set;

pub trait BitDebugExt {
    /// Creates a wrapper struct that prints this bitfield
    fn dbg_bitfield(self, width: usize) -> BitfieldPrinter;
    fn dbg_bit_indices(self) -> BitIndicesPrinter;
}

macro_rules! impl_ext {
    ($ty:ty) => {
        impl BitDebugExt for $ty {
            fn dbg_bitfield(self, width: usize) -> BitfieldPrinter {
                BitfieldPrinter {
                    bits: usize::try_from(self).unwrap(),
                    width,
                    chars: ['0', '1'],
                    color: None,
                }
            }

            fn dbg_bit_indices(self) -> BitIndicesPrinter {
                BitIndicesPrinter {
                    bits: usize::try_from(self).unwrap(),
                    sep: ",",
                }
            }
        }
    };
}

impl_ext!(usize);
impl_ext!(u64);
impl_ext!(u32);
impl_ext!(u16);
impl_ext!(u8);

pub struct BitfieldPrinter {
    bits: usize,
    width: usize,
    chars: [char; 2],
    color: Option<Color>,
}

pub struct BitIndicesPrinter {
    bits: usize,
    sep: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Cyan,
    Green,
    White,
}

#[rustfmt::skip]
#[allow(unused)]
impl BitfieldPrinter {
    /// Sets the characters to use for printing zeroes and ones respectively.
    pub const fn chars(self, c0: char, c1: char) -> Self {
        Self { chars: [c0, c1], ..self }
    }

    /// This printer will print ones in red.
    pub const fn red(self) -> Self {
        Self { color: Some(Color::Red), ..self }
    }

    pub const fn cyan(self) -> Self {
        Self { color: Some(Color::Cyan), ..self }
    }

    /// This printer will print ones in green.
    pub const fn green(self) -> Self {
        Self { color: Some(Color::Green), ..self }
    }

    /// This printer will print ones in white.
    pub const fn white(self) -> Self {
        Self { color: Some(Color::White), ..self }
    }
}

impl BitIndicesPrinter {
    /// Updates the separator this printer uses.
    pub const fn sep(self, sep: &'static str) -> Self {
        Self { sep, ..self }
    }
}

#[allow(unused)]
mod ansi {
    pub const RED: &str = "\x1b[38;5;9m";
    pub const CYAN: &str = "\x1b[38;5;14m";
    pub const GREEN: &str = "\x1b[38;5;10m";
    pub const WHITE: &str = "\x1b[38;5;15m"; // Technically "bright" white
    pub const BLACK: &str = "\x1b[38;5;238m"; // Technically dark gray
    pub const RESET: &str = "\x1b[0m";
}

impl Debug for BitfieldPrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [char0, char1] = self.chars;
        let [ansi0, ansi1] = match self.color {
            None => ["", ""],
            Some(Color::Red) => [ansi::BLACK, ansi::RED],
            Some(Color::Cyan) => [ansi::BLACK, ansi::CYAN],
            Some(Color::Green) => [ansi::BLACK, ansi::GREEN],
            Some(Color::White) => [ansi::BLACK, ansi::WHITE],
        };

        for i in 0..self.width {
            if bit_set!(self.bits, i) {
                write!(f, "{ansi1}{char1}")?;
            } else {
                write!(f, "{ansi0}{char0}")?;
            }
        }

        if self.color.is_some() {
            write!(f, "{}", ansi::RESET)?;
        }

        if f.alternate() {
            write!(f, "/{}{:0n$b}{}", ansi::BLACK, self.bits, ansi::RESET, n = self.width)?;
        }

        Ok(())
    }
}

impl Debug for BitIndicesPrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let top1 = usize::BITS - self.bits.leading_zeros();
        let mut first = true;
        for i in 0..top1 {
            if bit_set!(self.bits, i) {
                match first {
                    true => first = false,
                    false => write!(f, "{}", self.sep)?,
                }
                write!(f, "{i}")?;
            }
        }
        Ok(())
    }
}
