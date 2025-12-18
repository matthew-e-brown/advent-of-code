mod combo;
mod debug;
mod input;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use self::combo::Combinations;
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
fn min_presses_lights(machine: &Machine) -> u64 {
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
            let presses = mask.count_ones() as u64;
            if min_presses.is_none_or(|min| presses < min) {
                min_presses = Some(presses);
                min_buttons = Some(mask);
            }
        }
    }

    let min_presses = min_presses.expect("all puzzle machines should have at least one solution");
    let min_buttons = min_buttons.unwrap();

    if aoc_utils::verbosity() >= 1 {
        println!("\tMinimum button presses for lights: {min_presses}");
    }

    if aoc_utils::verbosity() >= 2 {
        println!("\t\tButtons: ({:?})", BitMask::dbg(min_buttons, num_buttons).chars('_', '↓').red());
    }

    if aoc_utils::verbosity() >= 3 {
        for i in 0..num_buttons {
            if bit_set(min_buttons, i) {
                let button = buttons[i].mask();
                println!("\t\tButton #{i:2}: ({:?})", BitMask::dbg(button, machine.size()).white());
            }
        }
    }

    min_presses
}

fn min_presses_joltages(machine: &Machine) -> u64 {
    // If some cell `i` which requires some `J` joltage, that means *some* combination of all the buttons which affect
    // that cell must be pressed a total of `J` times. So, we can start by trying all those combinations and seeing what
    // that does to the other cell joltages. No matter which combination we pick, cell `i` will have its final joltage,
    // as long as we never press any buttons that affect cell `i` again.
    //
    // 1. Pick some cell `i` which is not yet finished, and has some `J` goal joltage.
    // 2. Figure out the set of buttons `B` affect that cell.
    // 3. For each branch of the state tree:
    //    1. Cell `i` might have an existing joltage `j` in that state. Within this branch, we want to press some subset
    //       of the `B` buttons, of size `J-j`. We want to do this with replacement; i.e., maybe we one button `J-j`
    //       times, maybe it's two buttons `J-j/2` times.
    //    2. Try all combinations and see what it does to the other cells. Save those in the next level of state.
    // 4. Remove all buttons `B` from future consideration.
    //
    // When performing step 2., if there are no more buttons that affect cell `i`, then we may be out of solutions.
    // Remember: we picked all previous buttons based on the fact that they optimized another cell. If doing so claimed
    // all buttons that affect this cell, then either (a) this cell should *also* be optimized (i.e., two cells with the
    // same count), or (b) there are no more solutions. Or, more precisely, there are no more solutions that live under
    // the current branch. To stop searching, we simply don't push any more states into the tree.

    // To try and optimize our search space, we want to fill in cells based on whichever one requires the fewest
    // presses. Of course, we need to make sure we keep track of the index they came from to do that.
    let mut cells = machine
        .desired_joltages()
        .iter()
        .enumerate()
        .map(|(index, &joltage)| Reverse((joltage, index)))
        .collect::<BinaryHeap<Reverse<(u32, usize)>>>();

    // Each iteration, we handle the same set of buttons for all states. So we can keep track of which ones are
    // remaining in one central place.
    let buttons = machine.buttons();
    let mut remaining_buttons = (0..buttons.len()).map(Some).collect::<Vec<Option<usize>>>();

    // At each step of the search, we need to know (1) how many buttons have been pressed up to that point and (2) what
    // the current joltage values are after pressing those buttons.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct State {
        presses: u64,
        cells: Box<[u32]>,
    }

    // There may be multiple ways to get to each possible state: we only care about one for each level. We are going to
    // drain from the current level, process, and insert into the next level.
    let mut curr_level = HashSet::<State>::new();
    let mut next_level = HashSet::<State>::new();

    // Initial state: nothing pressed, all cells zero.
    curr_level.insert(State {
        presses: 0,
        cells: vec![0u32; machine.size()].into_boxed_slice(),
    });

    // Each time through the loop, we want to handle some subset of the buttons. We'll need to compute combinations of
    // those buttons, so we'll need a temporary buffer to store those into; we'll generate each combination by simply
    // indexing into this vector.
    let mut active_buttons = Vec::<usize>::new();

    if aoc_utils::verbosity() >= 2 {
        println!("\tDetermining button presses for joltage...");
    }

    while let Some(Reverse((goal_joltage, cell_index))) = cells.pop() {
        if aoc_utils::verbosity() >= 2 {
            println!("\t\tExploring choices for cell #{cell_index}: goal joltage: {goal_joltage}.");
        }

        // Okay, we've picked our next cell. Which buttons affect this cell?
        active_buttons.clear();
        for button_idx in &mut remaining_buttons[..] {
            if let Some(b) = *button_idx
                && let button = &buttons[b]
                && button.positions().binary_search(&cell_index).is_ok()
            {
                active_buttons.push(b);
                *button_idx = None; // Remove this button from consideration.
            }
        }

        if aoc_utils::verbosity() >= 3 {
            println!("\t\t\tButton choices: {active_buttons:?}");
        }

        // Now, for each current state in the tree...
        for state in curr_level.drain() {
            if aoc_utils::verbosity() >= 4 {
                print!("\t\t\tHandling {state:?}. ");
            }

            // How many presses do we need to get this cell up to the right value?
            let curr_joltage = state.cells[cell_index];
            let Some(presses_needed) = goal_joltage.checked_sub(curr_joltage) else {
                // If we somehow pressed enough buttons to overflow one of the cells, this entire branch of the state
                // tree is invalid.
                if aoc_utils::verbosity() >= 4 {
                    println!("Current at {curr_joltage}, but goal was only {goal_joltage}. Dropping state.");
                }

                continue;
            };

            if aoc_utils::verbosity() >= 4 {
                println!("Currently at {curr_joltage} jolts, need {presses_needed} more.");
            }

            // If there are no buttons remaining that affect this cell, then:
            if active_buttons.len() == 0 {
                // Only the states that don't need any more pressing for this cell advance to the next step. Otherwise,
                // this state is not valid.
                if presses_needed == 0 {
                    if aoc_utils::verbosity() >= 4 {
                        println!("\t\t\t\tPassing state to next level.");
                    }
                    next_level.insert(state);
                } else if aoc_utils::verbosity() >= 4 {
                    println!("\t\t\t\tNo valid configurations in this branch. Dropping branch.");
                }
            } else {
                // Try all the possible buttons that will bring it up to that level.
                let mut combinations = Combinations::new(&active_buttons[..], presses_needed as usize);
                while let Some(combo) = combinations.next() {
                    let mut state = state.clone();

                    if aoc_utils::verbosity() >= 4 {
                        print!("\t\t\t\tSimulating button combo {combo:?}: ");
                    }

                    // Apply all the buttons in this combination:
                    state.presses += presses_needed as u64;
                    for &&idx in combo {
                        buttons[idx].apply(&mut state.cells);
                    }

                    if aoc_utils::verbosity() >= 4 {
                        print!("Adding {state:?} to next level");
                    }

                    let was_new = next_level.insert(state);
                    if aoc_utils::verbosity() >= 4 {
                        if was_new {
                            println!(".");
                        } else {
                            println!(": State overlapped with existing.");
                        }
                    }
                }
            }
        }

        // Swap the next level over to become the new current level.
        std::mem::swap(&mut next_level, &mut curr_level);
    }

    // By the time we get down here, `curr` level should have the last of the possible states. All should now be valid.
    // We just need to find which one took the fewest number of presses.
    let min_presses = curr_level
        .into_iter()
        .map(|state| state.presses)
        .min()
        .expect("all puzzle machines should have at least one solution");

    if aoc_utils::verbosity() >= 1 {
        println!("\tMinimum button presses for joltages: {min_presses}");
    }

    min_presses
}
