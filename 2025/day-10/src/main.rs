mod combo;
mod debug;
mod input;

use self::debug::BitMask;
use self::input::Machine;

// cspell:words joltage joltages

fn main() {
    let input = aoc_utils::puzzle_input();
    let machines = input.lines().map(|line| line.parse::<Machine>().unwrap()).collect::<Vec<_>>();

    // Just to validate that the initial parse went correctly:
    if aoc_utils::verbosity() >= 4 {
        println!("All machines: {machines:#?}\n");
    }

    let mut lights_total = 0;
    let mut joltage_total = 0;
    for (i, machine) in machines.iter().enumerate() {
        if aoc_utils::verbosity() >= 1 {
            // Just print the machine, the two other functions will do the rest of the debug output.
            println!("Machine #{i:>3}: {machine:#?}", i = i + 1);
        }

        lights_total += min_presses_lights(machine);
        joltage_total += min_presses_joltages(machine);
    }

    println!("Fewest button presses to configure all machines' lights (part 1): {lights_total}");
    println!("Fewest button presses to configure all machines' joltage counters (part 2): {joltage_total}");
}

/// Checks if the `i`'th bit is set in a bit-mask.
#[inline]
fn bit_set(x: u32, i: usize) -> bool {
    ((x >> i) & 1) > 0
}

/// Finds the minimum number of button presses required to configure the lights on a [Machine].
///
/// If there is no possible way to achieve the desired lighting configuration on the machine, `None` is returned.
fn min_presses_lights(machine: &Machine) -> usize {
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
            if bit_set(mask, i) {
                lights ^= buttons[i].mask();
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

    let min_presses = min_presses.expect("all puzzle machines should have at least one solution");
    let min_buttons = min_buttons.unwrap();

    if aoc_utils::verbosity() >= 1 {
        println!("    Minimum button presses for lights: {min_presses}");
    }

    if aoc_utils::verbosity() >= 2 {
        println!("        Buttons: ({:?})", BitMask::dbg(min_buttons, num_buttons).chars('_', '↓').red());
    }

    if aoc_utils::verbosity() >= 3 {
        for i in 0..num_buttons {
            if bit_set(min_buttons, i) {
                let button = buttons[i].mask();
                println!("        Button #{i:2}: ({:?})", BitMask::dbg(button, machine.size()).white());
            }
        }
    }

    min_presses
}

fn min_presses_joltages(machine: &Machine) -> usize {
    todo!("part 2");
}
