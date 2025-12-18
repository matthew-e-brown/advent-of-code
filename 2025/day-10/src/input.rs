// cspell:words joltage joltages

use std::fmt::Debug;
use std::str::FromStr;

use crate::debug::BitMask;

#[derive(Clone)]
pub struct Machine {
    /// The bit pattern of the desired lights on this machine.
    lights: u32,
    /// The buttons on this machine. Each button toggles a specific set of lights, represented by a bit-mask.
    buttons: Box<[Button]>,
    /// The joltage requirements for this machine.
    joltages: Box<[u32]>,
}

#[derive(Clone)]
pub struct Button(Box<[usize]>);

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
    pub fn buttons(&self) -> &[Button] {
        &self.buttons
    }

    /// Gets the desired joltage values for this machine.
    pub fn desired_joltages(&self) -> &[u32] {
        &self.joltages
    }
}

impl Button {
    /// Gets a bit-mask made up of all the positions this button affects.
    pub fn mask(&self) -> u32 {
        let mut mask = 0;
        for &i in &self.0 {
            mask |= 1 << i;
        }
        mask
    }

    /// Gets a list of all the positions this button activates.
    pub fn positions(&self) -> &[usize] {
        &self.0
    }

    /// Apply this button's joltage changes to the given cells.
    pub fn apply(&self, cells: &mut [u32]) {
        for &i in self.positions() {
            cells[i] += 1;
        }
    }
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // I love state machines, raaaahh!!
        #[rustfmt::skip]
        enum State {
            Empty,
            LightsDone { lights: u32, n: usize, buttons: Vec<Button> },
            AllDone { lights: u32, buttons: Vec<Button>, joltages: Vec<u32> },
        }

        // Thankfully, all the pieces in the input contain no whitespace within them
        let mut pieces = s.split_whitespace();
        let mut state = State::Empty;

        while let Some(piece) = pieces.next() {
            match state {
                State::Empty => {
                    // First piece needs to be lights.
                    let (lights, n) = parse_light_diagram(piece)?;
                    state = State::LightsDone { lights, n, buttons: Vec::new() };
                },
                State::LightsDone { lights, n, mut buttons, .. } => {
                    if let Some(piece) = surrounded_by(piece, '(', ')') {
                        buttons.push(parse_button(piece, n)?);
                        state = State::LightsDone { lights, n, buttons }
                    } else if let Some(piece) = surrounded_by(piece, '{', '}') {
                        let joltages = parse_joltages(piece, n)?;
                        state = State::AllDone { lights, buttons, joltages };
                    } else {
                        return Err("machine description contains invalid segment");
                    }
                },
                State::AllDone { .. } => return Err("machine description contains extra components"),
            }
        }

        const ERR_EMPTY: &str = "machine description is empty";
        const ERR_NO_BUTTONS: &str = "machine description is missing at least one button schematic";
        const ERR_NO_JOLTAGE: &str = "machine description is missing joltage requirements";

        match state {
            State::Empty => Err(ERR_EMPTY),
            State::LightsDone { buttons, .. } if buttons.len() == 0 => Err(ERR_NO_BUTTONS),
            State::LightsDone { .. } => Err(ERR_NO_JOLTAGE),
            State::AllDone { lights, buttons, joltages } => Ok(Machine {
                lights,
                buttons: buttons.into_boxed_slice(),
                joltages: joltages.into_boxed_slice(),
            }),
        }
    }
}

fn surrounded_by(s: &str, before: char, after: char) -> Option<&str> {
    if s.starts_with(before) && s.ends_with(after) {
        let start = before.len_utf8();
        let end = s.len() - after.len_utf8();
        Some(&s[start..end])
    } else {
        None
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

        lights |= b << i;
    }

    if n == 0 {
        return Err("machine light diagram is empty");
    }

    Ok((lights, n))
}

fn parse_button(s: &str, n: usize) -> Result<Button, &'static str> {
    let bits = s.split(',');

    // Now set the i'th bit for each number specified.
    let mut positions = Vec::new();
    for bit in bits {
        let bit = bit.parse().map_err(|_| "machine button schematic contained invalid integers")?;
        if bit >= n {
            return Err("machine button schematic contains higher-indexed light than exists on light diagram");
        }

        positions.push(bit);
    }

    if positions.len() == 0 {
        return Err("machine button schematic is empty");
    }

    // The positions should also be sorted in ascending order, and we shouldn't have any duplicates.
    positions.sort_unstable();
    positions.dedup();
    Ok(Button(positions.into_boxed_slice()))
}

fn parse_joltages(s: &str, n: usize) -> Result<Vec<u32>, &'static str> {
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
        for button in &self.buttons {
            write!(f, " {:?}", button)?;
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

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b0 = self.0[0];
        write!(f, "({b0}")?;
        for i in 1..self.0.len() {
            write!(f, ",{}", self.0[i])?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
