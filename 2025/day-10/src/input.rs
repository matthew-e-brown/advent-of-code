// cspell:words joltage joltages

use std::fmt::Debug;
use std::str::FromStr;

use crate::debug::{BitMask, BitPositions};

#[derive(Clone)]
pub struct Machine {
    /// The buttons on this machine. Each button toggles a specific set of lights, represented by a bit-mask.
    buttons: Vec<u32>,
    /// The bit pattern of the desired lights on this machine.
    lights: u32,
    /// The joltage requirements for this machine.
    joltages: Vec<u32>,
}

#[allow(unused)]
impl Machine {
    /// Returns the "size" of this machine: the number of lights it has in its lighting diagram.
    pub fn size(&self) -> usize {
        self.joltages.len()
    }

    /// Gets a bit-mask representing the desired lighting pattern on this screen.
    pub fn desired_lights(&self) -> u32 {
        self.lights
    }

    /// Gets the button descriptions on this machine.
    pub fn buttons(&self) -> &[u32] {
        &self.buttons
    }

    /// Gets the desired joltage values for this machine.
    pub fn desired_joltages(&self) -> &[u32] {
        &self.joltages
    }
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Thankfully, all the pieces in the input contain no whitespace within them
        let mut pieces = s.split_whitespace();

        let lights = pieces.next().ok_or("machine description is empty")?;
        let (lights, n) = parse_light_diagram(lights)?;

        let mut buttons = Vec::new();
        let joltages;
        loop {
            // If we run out of pieces before breaking ourselves later, input is malformed.
            let Some(piece) = pieces.next() else {
                if buttons.len() == 0 {
                    return Err("machine description is missing at least one button schematic");
                } else {
                    return Err("machine description is missing joltage requirements");
                }
            };

            if piece.starts_with('(') && piece.ends_with(')') {
                buttons.push(parse_button(&piece[1..piece.len() - 1], n)?);
            } else if piece.starts_with('{') && piece.ends_with('}') {
                joltages = parse_joltage(&piece[1..piece.len() - 1], n)?;
                break;
            } else {
                return Err("machine description contains invalid segment");
            }
        }

        Ok(Machine { buttons, lights, joltages })
    }
}

fn parse_light_diagram(s: &str) -> Result<(u32, usize), &'static str> {
    let mut chars = s.chars();
    if !chars.next().is_some_and(|c| c == '[') || !chars.next_back().is_some_and(|c| c == ']') {
        return Err("machine light diagram should be delineated by '[]'");
    }

    let n = s.len() - 2; // How many lights are we working with?
    let mut lights = 0u32;
    for (i, c) in chars.enumerate() {
        let b = match c {
            '.' => 0,
            '#' => 1,
            _ => return Err("machine light diagram contained invalid characters"),
        };

        // We want the lighting pattern to match what we see in the diagram: MSB is left. So for 4 characters, the 1st
        // bit needs to be shifted over 3 times, the 2nd 2 times, the 3rd 1 time, and the 4th 0 times.
        lights |= b << (n - 1 - i);
    }

    if n == 0 {
        return Err("machine light diagram is empty");
    }

    Ok((lights, n))
}

fn parse_button(s: &str, n: usize) -> Result<u32, &'static str> {
    let positions = s.split(',');

    // Now set the i'th bit for each number specified.
    let mut button = 0u32;
    for i in positions {
        let i = i
            .parse::<usize>()
            .map_err(|_| "machine button schematic contained invalid integers")?;

        if i >= n {
            return Err("machine button schematic referenced more lights than exist on light diagram");
        }

        button |= 1 << (n - 1 - i);
    }

    Ok(button)
}

fn parse_joltage(s: &str, n: usize) -> Result<Vec<u32>, &'static str> {
    let mut bits = s.split(',');

    let mut joltages = Vec::with_capacity(n);
    for _ in 0..n {
        let Some(piece) = bits.next() else {
            return Err("machine has fewer joltage requirements than lights on light diagram");
        };

        let joltage = piece
            .parse()
            .map_err(|_| "machine joltage requirements should contain valid integers")?;
        joltages.push(joltage);
    }

    if bits.next().is_some() {
        return Err("machine has more joltage requirements than lights on light diagram");
    }

    Ok(joltages)
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write lights:
        write!(f, "[{:?}]", BitMask::dbg(self.lights, self.size()).chars('.', '#').green())?;

        // Write buttons:
        for &button in &self.buttons {
            write!(f, " ({:?})", BitPositions::dbg(button, self.size()))?;
        }

        // Write joltages:
        let j0 = self.joltages[0];
        write!(f, " {{{j0}")?;
        for i in 1..self.size() {
            write!(f, ",{}", self.joltages[i])?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}
