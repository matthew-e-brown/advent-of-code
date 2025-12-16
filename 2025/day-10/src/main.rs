mod debug;
mod input;

// cspell:words joltage joltages
use self::input::Machine;

fn main() {
    let input = aoc_utils::puzzle_input();
    let machines = input.lines().map(|line| line.parse::<Machine>().unwrap()).collect::<Vec<_>>();

    // Just to validate that the initial parse went correctly:
    if aoc_utils::verbosity() >= 4 {
        println!("All machines: {machines:#?}\n");
    }

    let mut min_total = 0;
    for machine in &machines {
        let min = min_presses_lights(machine).expect("all puzzle machines should have at least one solution");
        min_total += min;
    }

    println!("Fewest button presses required to configure all machines' lights (part 1): {min_total}");
}

/// Checks if the `i`th bit "from the left" (where "the left" is determined by the width) is set in a bit-mask.
#[inline]
fn bit_set(x: u32, i: usize, w: usize) -> bool {
    (x >> (w - 1 - i) & 1) > 0
}

/// Finds the minimum number of button presses required to configure the lights on a [Machine].
///
/// If there is no possible way to achieve the desired lighting configuration on the machine, `None` is returned.
fn min_presses_lights(machine: &Machine) -> Option<usize> {
    // - The lights on the machine form a bit-mask
    // - The buttons control certain lights: they are also a bit-mask, and toggling them is an XOR.
    // - XOR is commutative, associative, and is its own inverse; if `C = A ^ B`, then `C ^ A = B`.
    // - Notably, that means that XORing one number into another twice does nothing: A^A = 0, and thanks to commutative
    //   and associative properties: `(A ^ B ^ C ^ D) ^ B = (A ^ (B ^ B) ^ C ^ D) = (A ^ 0 ^ C ^ D) = (A ^ C ^ D)`.
    // - That means that each button will either be pressed one or zero times.
    // - To check all possible combinations for n buttons, we can simply loop from zero up to 2^n-1 (n ones in binary).
    //   Then, the decision to include each button is determined by the i'th bit being set.

    let mut min_presses = None;
    let mut min_buttons = None;

    let buttons = machine.buttons();
    let num_buttons = buttons.len();
    let button_mask = u32::MAX >> (u32::BITS - num_buttons as u32); // 0b1111 for a machine with 4 buttons, etc.

    for mask in 0..=button_mask {
        let mut lights = 0u32;
        for i in 0..num_buttons {
            // If the i'th bit is set, test this button.
            if bit_set(mask, i, num_buttons) {
                lights ^= buttons[i];
            }
        }

        // If our bitmask matches after testing those lights, this combination is valid. Check if it's the fewest:
        if lights == machine.desired_lights() {
            let presses = mask.count_ones() as usize;
            if min_presses.is_none_or(|min| presses < min) {
                min_presses = Some(presses);
                min_buttons = Some(mask);
            }
        }
    }

    if aoc_utils::verbosity() >= 1
        && let Some(min) = min_presses
    {
        println!("{min:>2} for machine: {machine:#?}");
    }

    if aoc_utils::verbosity() >= 2
        && let Some(mask) = min_buttons
    {
        println!("    Buttons: ({:?})", debug::BitMask::dbg(mask, num_buttons).chars('_', '↓').red());
        if aoc_utils::verbosity() >= 3 {
            for i in 0..num_buttons {
                if bit_set(mask, i, num_buttons) {
                    let button = buttons[i];
                    println!("    Button #{i:2}: ({:?})", debug::BitMask::dbg(button, machine.size()).white());
                }
            }
        }
    }

    min_presses
}
