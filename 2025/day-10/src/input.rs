use std::fmt::Debug;
use std::num::IntErrorKind;
use std::str::FromStr;

use crate::debug::BitDebugExt;

// cspell:words joltage joltages

// These only need to be as large as u16, but using native-sized integer types should probably be the fastest in
// general.

/// The integer type used for bitfields in a [Machine].
pub type Bitfield = usize;

/// The integer type used for joltage counter values in a [Machine].
pub type Joltage = usize;

const BITFIELD_BITS: usize = Bitfield::BITS as usize;

/// A machine in the factory.
#[derive(Clone)]
pub struct Machine {
    pub lights: Bitfield,
    pub buttons: Vec<Bitfield>,
    pub joltages: Vec<Joltage>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lights = None;
        let mut buttons = Vec::new();
        let mut joltages = None;

        let format_err = |err: &str| format!("invalid machine description: {err}");

        for piece in s.split_whitespace() {
            if let Some(inner) = str_between(piece, '[', ']') {
                if lights.is_none() {
                    let n = inner.len(); // Knowing how many lights we saw is helpful for validation
                    let l = parse_lights(inner).map_err(format_err)?;
                    lights = Some((l, n));
                } else {
                    return Err(format_err("encountered multiple lighting diagrams"));
                }
            } else if let Some(inner) = str_between(piece, '(', ')') {
                let button = parse_button(inner).map_err(format_err)?;
                buttons.push(button);
            } else if let Some(inner) = str_between(piece, '{', '}') {
                if joltages.is_none() {
                    joltages = Some(parse_joltages(inner).map_err(format_err)?);
                } else {
                    return Err(format_err("encountered multiple joltage requirements"));
                }
            } else {
                return Err(format_err(&format!("encountered unknown token: {piece}")));
            }
        }

        let (lights, n) = lights.ok_or_else(|| format_err("missing lighting diagram"))?;

        if buttons.len() < 1 {
            return Err(format_err("at least one button is required"));
        }

        let joltages = joltages.ok_or_else(|| format_err("missing joltage requirements"))?;

        if joltages.len() != n {
            return Err(format_err("lighting diagram and joltage requirements should have the same size"));
        }

        Ok(Machine { lights, buttons, joltages })
    }
}

/// Extracts a substring from out from between two surrounding characters.
///
/// If `text` starts with `before` and ends with `after`, returns `text` with `before` and `after` trimmed. If it does
/// not start with `before` or it does not end with `after`, `None` is returned.
fn str_between(text: &str, before: char, after: char) -> Option<&str> {
    if text.starts_with(before) && text.ends_with(after) {
        let i = before.len_utf8();
        let j = text.len() - after.len_utf8();
        Some(&text[i..j])
    } else {
        None
    }
}

fn parse_lights(s: &str) -> Result<Bitfield, &'static str> {
    if s.len() == 0 {
        return Err("light diagram is empty");
    } else if s.len() > BITFIELD_BITS {
        return Err("light diagram contains too many lights");
    }

    let mut i = 0usize;
    let mut lights = 0 as Bitfield;
    for c in s.chars() {
        let b = match c {
            '.' => 0,
            '#' => 1,
            _ => return Err("light diagram contains invalid character(s)"),
        };

        lights |= b << i;
        i += 1;
    }

    Ok(lights)
}

fn parse_button(s: &str) -> Result<Bitfield, &'static str> {
    let mut button = 0;
    for num in s.split(',') {
        let idx = match num.parse::<usize>() {
            Ok(idx @ 0..BITFIELD_BITS) => idx as Bitfield,
            Ok(_) => return Err("button specified an index greater than allowed"),
            Err(_) => return Err("button specified an invalid integer as an index"),
        };
        button |= 1 << idx;
    }

    Ok(button)
}

fn parse_joltages(s: &str) -> Result<Vec<Joltage>, &'static str> {
    let mut joltages = Vec::new();
    for num in s.split(',') {
        let joltage = num.parse::<Joltage>().map_err(|err| match err.kind() {
            IntErrorKind::PosOverflow => "one or more joltages are too large for integer type",
            _ => "joltages contained an invalid integer",
        })?;

        joltages.push(joltage);
    }

    if joltages.len() == 0 {
        return Err("joltages should contain at least one joltage");
    }

    Ok(joltages)
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /// Helper that prints a bitfield like a button appears in the puzzle input.
        struct Button(Bitfield);
        impl Debug for Button {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({:?})", self.0.dbg_bit_indices())
            }
        }

        /// Helper that prints a list of buttons either with the classic `[]` debug-list format, or as space-separated
        /// to match puzzle input.
        struct ButtonList<'a>(&'a [Bitfield]);
        impl<'a> Debug for ButtonList<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let buttons = self.0.iter().copied().map(Button);
                if f.alternate() {
                    for button in buttons {
                        write!(f, " {button:?}")?;
                    }
                    Ok(())
                } else {
                    f.debug_list().entries(buttons).finish()
                }
            }
        }

        let lights = self.lights.dbg_bitfield(self.joltages.len());
        let buttons = ButtonList(&self.buttons);

        if f.alternate() {
            let lights = lights.chars('.', '#').green();
            write!(f, "[{lights:?}]{buttons:#?}")?; // ButtonList's alternate mode handles the space
            write!(f, " {{{:?}", self.joltages[0])?; // There is always at least one joltage
            for j in &self.joltages[1..] {
                write!(f, ",{j:?}")?;
            }
            write!(f, "}}")?;
            Ok(())
        } else {
            f.debug_struct("Machine")
                .field("lights", &lights)
                .field("buttons", &buttons)
                .field("joltages", &self.joltages)
                .finish()
        }
    }
}
