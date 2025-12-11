// cspell:words joltage joltages

use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone)]
pub struct Machine {
    /// The number of lights this machine has on its lighting diagram.
    size: usize,
    /// A bit-mask representing the current status of the lights on this machine.
    pub lights: u32,
    /// The bit pattern of the desired lights on this machine.
    desired_lights: u32,
    /// The buttons on this machine. Each button toggles a specific set of lights, represented by a bit-mask.
    buttons: Vec<u32>,
    /// The joltage requirements for this machine.
    desired_joltages: Vec<u32>,
}

impl Machine {
    /// Returns the "size" of this machine: the number of lights it has in its lighting diagram.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Gets a bit-mask representing the desired lighting pattern on this screen.
    pub fn desired_lights(&self) -> u32 {
        self.desired_lights
    }

    /// Returns a bit-mask of only the bits involved in this machine's lighting diagram.
    pub fn mask(&self) -> u32 {
        u32::MAX >> (u32::BITS - self.size as u32)
    }

    /// Gets the button descriptions on this machine.
    pub fn buttons(&self) -> &[u32] {
        &self.buttons
    }

    /// Gets the desired joltage values for this machine.
    pub fn desired_joltages(&self) -> &[u32] {
        &self.desired_joltages
    }

    pub fn lights_are_correct(&self) -> bool {
        self.lights == self.desired_lights
    }
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Thankfully, all the pieces in the input contain no whitespace within them
        let mut pieces = s.split_whitespace();

        let lights = pieces.next().ok_or("machine description is empty")?;
        let (desired_lights, n) = parse_light_diagram(lights)?;

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

        Ok(Machine {
            size: n,
            lights: 0,
            desired_lights,
            buttons,
            desired_joltages: joltages,
        })
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
    let bits = s.split(',');

    // Now set the n'th bit for each number. But, to match the diagram, we again want '0' to be the *most* significant
    // bit, not the least. So we instead shift over by `n - 1 - b`.
    let mut button = 0u32;
    for b in bits {
        let b = b
            .parse::<usize>()
            .map_err(|_| "machine button schematic contained invalid integers")?;
        if b >= n {
            return Err("machine button schematic referenced more lights than exist on light diagram");
        }

        button |= 1 << (n - 1 - b);
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
        if !f.alternate() {
            f.debug_struct("Machine")
                .field("size", &self.size)
                .field("lights", &self.lights)
                .field("desired_lights", &self.desired_lights)
                .field("buttons", &self.buttons)
                .field("desired_joltages", &self.desired_joltages)
                .finish()
        } else {
            const ANSI_BLACK: &str = "\x1b[38;5;238m"; // Technically dark gray
            const ANSI_WHITE: &str = "\x1b[38;5;15m";
            const ANSI_RESET: &str = "\x1b[0m";

            // Write lights:
            write!(f, "[")?;
            for i in 0..self.size {
                // The 0th bit is actually stored as the MSB, to match the diagram.
                let is_on = (self.lights >> (self.size - 1 - i)) > 0;
                let should = (self.desired_lights >> (self.size - 1 - i)) > 0;
                let color = if is_on { ANSI_WHITE } else { ANSI_BLACK };
                let shape = if should { '#' } else { '.' };
                write!(f, "{color}{shape}")?;
            }
            write!(f, "{ANSI_RESET}]")?;

            // Write buttons:
            for &button in &self.buttons {
                write!(f, " (")?;
                for i in 0..self.size {
                    // Does this button affect bit number i?
                    let affects = (button >> (self.size - 1 - i)) > 0;
                    if affects {
                        write!(f, "{i}")?;
                        if i < self.size - 1 {
                            write!(f, ",")?;
                        }
                    }
                }
                write!(f, ")")?;
            }

            // Write joltage:
            let j0 = self.desired_joltages[0];
            write!(f, " {{{j0}")?;
            for i in 1..self.desired_joltages.len() {
                write!(f, ",{}", self.desired_joltages[i])?;
            }
            write!(f, "}}")?;

            Ok(())
        }
    }
}
