mod input;

use self::input::{Bitfield, Joltage, Machine};

// cspell:words joltage joltages

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut lights_presses_total = 0u64;
    let mut joltage_presses_total = 0u64;
    for (i, line) in input.lines().enumerate() {
        let Machine { lights, buttons, mut joltages } = line.parse().expect("puzzle input should be valid");

        if aoc_utils::verbosity() >= 1 {
            println!("Machine #{i}:", i = i + 1);
        }

        // Part 1:
        let light_buttons = min_buttons_for_parity(&buttons, lights);
        lights_presses_total += light_buttons.count_ones() as u64;

        // Part 2:
        let joltage_presses = min_presses_for_joltages(&buttons, &mut joltages);
        joltage_presses_total += joltage_presses;

        if aoc_utils::verbosity() >= 1 {
            println!("Machine #{i} joltage presses: {joltage_presses}\n", i = i + 1);
        }
    }

    println!("Fewest button presses to configure all machines' lights (part 1): {lights_presses_total}");
    println!("Fewest button presses to configure all machines' joltages (part 2): {joltage_presses_total}");
}

/// Checks if the `i`'th bit is set in a bit-mask.
macro_rules! bit_set {
    ($x:expr, $i:expr) => {
        (($x >> $i) & 1) == 1
    };
}

/// Gets the minimum possible set of buttons required to achieve the given parity.
///
/// Button pressing is represented as a bitwise XOR operation:
///
/// - XOR is commutative, associative, and is its own inverse; if `C = A ^ B`, then `C ^ A = B`.
/// - Notably, that means that XORing one number into another twice does nothing: A^A = 0, and thanks to commutative and
///   associative properties: `(A ^ B ^ C ^ D) ^ B = (A ^ (B ^ B) ^ C ^ D) = (A ^ 0 ^ C ^ D) = (A ^ C ^ D)`.
/// - That means that each button will be pressed either zero or one times.
///
/// The answer is returned as a bit-mask of the indices within `buttons` which should be pressed to achieve the desired
/// parity.
fn min_buttons_for_parity(buttons: &[Bitfield], goal: Bitfield) -> usize {
    // Each bitfield can have at most Bitfield::BITS bits, but the maximum number of buttons is limited only by `usize`
    // (i.e., the machine might use u16 for its Bitfields, but allow for at most).
    let num_buttons = buttons.len();
    if num_buttons > usize::BITS as usize {
        panic!("only up to {} buttons are supported", usize::BITS);
    }

    let mut min_presses = Option::<Bitfield>::None;
    let max_button_mask = usize::MAX >> (usize::BITS - num_buttons as u32); // 0b0111 for a machine with 3 buttons
    for button_mask in 0..=max_button_mask {
        // If we already know another solution that uses fewer bits, don't bother trying this one.
        if min_presses.is_some_and(|min| min.count_ones() <= button_mask.count_ones()) {
            continue;
        }

        // Does this combination of buttons work?
        let mut lights: Bitfield = 0;
        for &button in buttons.iter().bit_filter(button_mask) {
            lights ^= button;
        }

        if lights == goal {
            if min_presses.is_none_or(|min| button_mask.count_ones() < min.count_ones()) {
                min_presses = Some(button_mask);
            }
        }
    }

    min_presses.expect("all puzzle machines should have at least one solution")
}

fn min_presses_for_joltages(buttons: &[Bitfield], counters: &mut [Joltage]) -> u64 {
    // First, figure out which buttons we need to press to make our counters have the **parity** they currently do.
    let parity_bits = parity(&counters);
    let parity_buttons = min_buttons_for_parity(buttons, parity_bits);
    let parity_presses = parity_buttons.count_ones() as u64;

    if aoc_utils::verbosity() >= 1 {
        println!("Joltage counters {counters:?} have parity {parity_bits:0n$b}", n = counters.len());
        print!("\tButtons {parity_buttons:0n$b}:", n = buttons.len());
        for &button in buttons.iter().bit_filter(parity_buttons) {
            print!(" ({button:0n$b})", n = counters.len());
        }
        println!();
    }

    // Next, assuming we press those buttons, what joltages will we have left to determine button presses for?
    // (parity_buttons is a bitmask of buttons to press, then each button is a bitmask of which joltages to affect):
    for &button in buttons.iter().bit_filter(parity_buttons) {
        for joltage in counters.iter_mut().bit_filter(button) {
            *joltage -= 1;
        }
    }

    if aoc_utils::verbosity() >= 1 {
        print!("\tAfter applying button presses, counters are {counters:?}. ");
    }

    // All remaining joltages should have an even parity. At the very least, they are divisible by two. However, we can
    // trim the search space by a few steps if we divide by a larger power of two. So, what's the largest power of two
    // that all joltages are divisible by?

    // The largest power of 2 that divides a number evenly can be found by looking at the number of trailing zeroes; as
    // long as the number is not zero. 20 = 0b0010100. There are 2 trailing zeroes, so 2^2 = 4 is the highest power of
    // two that divides 20.
    let power = counters.iter().copied().filter(|&j| j > 0).map(|j| j.trailing_zeros()).min();

    // If all children are zero, we don't need to recurse any more!
    let child_total = if let Some(power) = power {
        if aoc_utils::verbosity() >= 1 {
            println!("All are divisible by {}.", 1 << power);
        }

        // Divide all joltages by that power of 2:
        for joltage in counters.iter_mut() {
            *joltage >>= power;
        }

        // Now, how many times would those counters need to be pressed, in the optimal case?
        let child_count = min_presses_for_joltages(buttons, counters);
        let child_total = child_count << power; // How many times would we have to do that step?

        if aoc_utils::verbosity() >= 1 {
            println!("\tTotal child presses: {child_count} << {power} = {child_total}");
        }

        child_total
    } else {
        if aoc_utils::verbosity() >= 1 {
            println!("All finished!");
        }

        0
    };

    parity_presses + child_total
}

/// Determines the parity of a series of [Joltage] counters.
fn parity(counters: &[Joltage]) -> Bitfield {
    let mut bits = 0;
    for i in 0..counters.len() {
        let b = (counters[i] & 1) as Bitfield; // 1 for odd, 0 for even
        bits |= b << i;
    }
    bits
}


trait IterBitExt: Iterator + Sized {
    /// Filters items in this iterator based on the given bitmask and their indices.
    fn bit_filter(self, mask: usize) -> BitFilter<Self> {
        BitFilter { iter: self.enumerate(), mask }
    }
}

impl<I: Iterator + Sized> IterBitExt for I {}

struct BitFilter<I: Iterator> {
    iter: std::iter::Enumerate<I>,
    mask: usize,
}

impl<I: Iterator> Iterator for BitFilter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (i, item) = self.iter.next()?;
            if bit_set!(self.mask, i) {
                break Some(item);
            }
        }
    }
}
