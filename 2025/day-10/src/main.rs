mod debug;
mod input;

use self::debug::BitDebugExt;
use self::input::{Bitfield, Joltage, Machine};

// cspell:words joltage joltages
// cspell:ignore dprintln dprint

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut lights_presses_total = 0u64;
    let mut joltage_presses_total = 0u64;
    for (i, line) in input.lines().enumerate() {
        let i = i + 1;

        let machine = line.parse().expect("puzzle input should be valid");
        if aoc_utils::verbosity() >= 1 {
            println!("Machine #{i}: {machine:#?}");
        }

        let Machine { lights, buttons, joltages } = machine;

        // Part 1:
        let light_presses =
            min_presses_lights(&buttons, lights).expect("all puzzle machines should have at least one solution");
        lights_presses_total += light_presses;

        // Part 2:
        let joltage_presses =
            min_presses_joltages(&buttons, joltages, 0).expect("all puzzle machines should have at least one solution");
        joltage_presses_total += joltage_presses;

        if aoc_utils::verbosity() >= 1 {
            println!("\tPresses for lights: {light_presses}");
            println!("\tPresses for joltage: {joltage_presses}\n");
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

pub(crate) use bit_set;

/// Determines the parity of a series of [Joltage] counters.
fn parity(counters: &[Joltage]) -> Bitfield {
    let mut bits = 0;
    for i in 0..counters.len() {
        let b = (counters[i] & 1) as Bitfield; // 1 for odd, 0 for even
        bits |= b << i;
    }
    bits
}

/// Gets all possible combinations of the `buttons` that yield the given parity.
///
/// Button pressing is represented as a bitwise XOR operation:
///
/// - XOR is commutative, associative, and is its own inverse; if `C = A ^ B`, then `C ^ A = B`.
/// - Notably, that means that XORing one number into another twice does nothing: A^A = 0, and thanks to commutative and
///   associative properties: `(A ^ B ^ C ^ D) ^ B = (A ^ (B ^ B) ^ C ^ D) = (A ^ 0 ^ C ^ D) = (A ^ C ^ D)`.
/// - That means that each button will be pressed either zero or one times.
///
/// The combinations are returned as a bit-mask of the indices within `buttons` which should be pressed to achieve the
/// desired parity.
fn parity_combinations(buttons: &[Bitfield], goal: Bitfield) -> impl Iterator<Item = usize> {
    // The `Bitfield` type is specifically for the machine's bitfields, and may be smaller; when picking combinations of
    // buttons, we'll use a `usize` directly.
    let num_buttons = buttons.len();
    if num_buttons > usize::BITS as usize {
        panic!("only combinations of up to {} buttons are supported", usize::BITS);
    }

    let max_button_mask = usize::MAX >> (usize::BITS - num_buttons as u32); // 0b0111 for a machine with 3 buttons
    (0..=max_button_mask).filter_map(move |button_mask| {
        // What will the lights look like after we press these buttons?
        let mut lights: Bitfield = 0;
        for &button in buttons.iter().bit_filter(button_mask) {
            lights ^= button;
        }

        // Does this combination of buttons work?
        (lights == goal).then_some(button_mask)
    })
}

/// Gets the minimum possible set of buttons required to achieve the given parity.
fn min_presses_lights(buttons: &[Bitfield], lights: Bitfield) -> Option<u64> {
    parity_combinations(buttons, lights)
        .map(|button_mask| button_mask.count_ones() as u64)
        .min()
}

fn min_presses_joltages(buttons: &[Bitfield], counters: Box<[Joltage]>, depth: usize) -> Option<u64> {
    // ===========================================================
    macro_rules! dprint {
        () => { print!("{:indent$}", "", indent = 8 * depth); };
        ($($tokens:tt)*) => { dprint!(); print!($($tokens)*); };
    }
    macro_rules! dprintln {
        () => { dprint!("\n"); };
        ($($tokens:tt)*) => { dprint!(); println!($($tokens)*); };
    }
    // ===========================================================

    // First, figure out which buttons we need to press to make our counters have the **parity** they currently do.
    let parity_bits = parity(&counters);

    if aoc_utils::verbosity() >= 2 {
        let parity = parity_bits.dbg_bitfield(counters.len()).chars('.', '#').cyan();
        dprintln!("\tJoltage counters {counters:?} have parity [{parity:?}]");
    }

    let all_combinations = parity_combinations(buttons, parity_bits).filter_map(|button_mask| -> Option<u64> {
        // How many presses did this particular combination take?
        let parity_presses = button_mask.count_ones() as u64;

        let dbg_buttons = button_mask.dbg_bitfield(buttons.len()).chars('_', '↓').red();

        if aoc_utils::verbosity() >= 2 {
            dprint!("\t\tPushing button combo [{dbg_buttons:?}]:");
            for &button in buttons.iter().bit_filter(button_mask) {
                print!(" ({:?})", button.dbg_bit_indices());
            }
            println!();
        }

        // What happens to our joltages when we press those buttons?
        let mut counters = counters.clone();
        for &button in buttons.iter().bit_filter(button_mask) {
            for joltage in counters.iter_mut().bit_filter(button) {
                // If pressing these buttons results in us underflowing a counter, stop considering it.
                *joltage = match joltage.checked_sub(1) {
                    Some(j) => j,
                    None => {
                        if aoc_utils::verbosity() >= 2 {
                            dprintln!("\t\tPushing button combination causes underflow; pruning search.");
                        }
                        return None;
                    },
                }
            }
        }

        if aoc_utils::verbosity() >= 2 {
            dprintln!("\t\tAfter pushing, joltages are {counters:?}");
        }

        // After pressing those buttons, all remaining joltages should be divisible by two. Cut them in half and check
        // how to achieve those joltages optimally:
        if counters.iter().all(|&j| j == 0) {
            // If all our joltages are zero, we're done!
            if aoc_utils::verbosity() >= 2 {
                dprintln!("\t\tAll finished! Optimal presses is {parity_presses}.");
            }

            Some(parity_presses)
        } else {
            if aoc_utils::verbosity() >= 2 {
                dprintln!("\t\tAll joltages divisible by 2, stepping down:");
            }

            for joltage in counters.iter_mut() {
                *joltage >>= 1;
            }

            let child_total = match min_presses_joltages(buttons, counters, depth + 1) {
                Some(child_count) => {
                    if aoc_utils::verbosity() >= 2 {
                        dprintln!(
                            "\t\tFinished recursion: child step took {child_count} presses: doubles to {}",
                            child_count << 1
                        );
                    }
                    child_count << 1
                },
                None => {
                    if aoc_utils::verbosity() >= 2 {
                        dprintln!("\t\tFinished recursion: found no solution starting with [{dbg_buttons:?}].");
                    }
                    return None;
                },
            };

            Some(parity_presses + child_total)
        }
    });

    all_combinations.min()
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
