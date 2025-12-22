mod debug;
mod input;

use std::collections::HashMap;

use aoc_utils::println_v;

use self::input::{Bitfield, Joltage, Machine};

// cspell:words joltage joltages

fn main() {
    let input = aoc_utils::puzzle_input();
    let machines = input.lines().enumerate().map(|(i, line)| {
        let parsed = line.parse::<Machine>().expect("puzzle input should be valid");
        (i + 1, parsed) // 1-based index for debug printing
    });

    let mut lights_presses_total = 0;
    let mut joltage_presses_total = 0;
    for (i, machine) in machines {
        println_v!(1, "Machine #{i}: {machine:#?}");

        // Pressing a button toggles a light between on and off. We model this XORs on bitfields:
        //
        // - XOR is commutative, associative, and is its own inverse; if `C = A ^ B`, then `C ^ A = B`.
        // - Notably, that means that XORing one number into another twice does nothing: A^A = 0, and thanks to
        //   commutative and associative properties:
        //   `(A ^ B ^ C ^ D) ^ B = (A ^ (B ^ B) ^ C ^ D) = (A ^ 0 ^ C ^ D) = (A ^ C ^ D)`.
        // - That means that, for any given parity, each button will be pressed exactly zero or one time.
        //
        // We can apply the same logic to help us solve part 2: instead of toggling the machine's lights, each button
        // press toggles the *parity* of the joltage counters. Over the course of part 2, we'll need to recursively
        // check many, many possible configurations of the lights/parity; so the most efficient course of action here is
        // to precompute all the possible combinations.
        let parity_combos = compute_parity_map(&machine);

        let light_presses = configure_lights(machine.lights, &parity_combos);
        let joltage_presses = configure_joltages(&machine.buttons, machine.joltages, &parity_combos);

        println_v!(1, "\tPresses for lights: {light_presses}");
        println_v!(1, "\tPresses for joltage: {joltage_presses}\n");

        lights_presses_total += light_presses;
        joltage_presses_total += joltage_presses;
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

/// Computes **all** possible button combinations on this machine that would yield **any** possible parity on its
/// lights/joltage counters.
///
/// Since each button is always pressed exactly 0 or 1 times, the combinations are returned as bitmasks which describe
/// indices in the machine's `buttons` array that should be pressed to achieve the desired parity.
fn compute_parity_map(machine: &Machine) -> HashMap<Bitfield, Option<Vec<usize>>> {
    // In theory, we could have gotten away with not computing all possible combinations for all possible parities. We
    // *could* have populated a map lazily while doing the recursion in `configure_joltages`; however, that introduces
    // lifetime issues when trying to do recursion. Each step would try to push a its newly discovered button
    // combinations into the map, but the upper level would still be holding onto its list of combinations; trying to
    // push into the hashmap might cause a reallocation and move the vector out from under the upper level. Since there
    // are at most 2^10 = 1024 possible parity configurations for any given machine, though, it's really not so bad to
    // compute them up front.
    let mut parity_combos = HashMap::new();

    // What are all the possible *parity* values this machine could have on its lights/joltage counters?
    let max_parity_mask = Bitfield::MAX >> (Bitfield::BITS - machine.size() as u32); // 1 << N, but avoids overflow
    let max_button_mask = usize::MAX >> (usize::BITS - machine.buttons.len() as u32);

    // Seed the hashmap with `None`. Then we can simply iterate over all the button combinations and insert replace only
    // the relevant ones with `Some`.
    for parity in 0..=max_parity_mask {
        parity_combos.insert(parity, None);
    }

    for button_mask in 0..=max_button_mask {
        let mut parity: Bitfield = 0;
        for &button in machine.buttons.iter().bit_filter(button_mask) {
            parity ^= button;
        }

        parity_combos
            .get_mut(&parity)
            .expect("all parity combos should have already been computed")
            .get_or_insert_with(|| Vec::new())
            .push(button_mask);
    }

    parity_combos
}

/// In theory, it's possible to specify a machine whose lights or joltage counters are not possible to configure with
/// their set of buttons. However, for Advent of Code input, all machines should be valid.
const AT_LEAST_ONE: &str = "all puzzle machines should have at least one solution";

/// Determines the optimal set of buttons to press to configure a machine's lights.
fn configure_lights(lights: Bitfield, parity_combos: &HashMap<Bitfield, Option<Vec<usize>>>) -> u64 {
    // Look through all the combinations that would yield the parity pattern for the machine's lights:
    let light_combos = parity_combos[&lights].as_deref().expect(AT_LEAST_ONE);

    // Then, find out which one has the fewest button presses:
    let light_presses = light_combos.iter().map(|button_mask| button_mask.count_ones() as u64);
    light_presses.min().expect(AT_LEAST_ONE)
}

/// Determines the optimal set of buttons to press to configure a machine's joltage counters.
fn configure_joltages(
    buttons: &[Bitfield],
    joltages: Box<[Joltage]>,
    parity_combos: &HashMap<Bitfield, Option<Vec<usize>>>,
) -> u64 {
    // To keep things fast, we'll use cache the optimal solutions for each of the possible joltage values, since the end
    // of the recursion tree will likely have a lot of repetition. Also, it's a good idea to cache whether or not the
    // even is a solution for that particular set of joltages, so we'll use an Option. We can initialize the solution
    // for `[0, 0, 0, ..., 0]` joltages now, since we already know it would take zero presses to reach joltages of all
    // zeroes.
    let mut joltage_solutions = HashMap::<Box<[Joltage]>, Option<u64>>::new();
    joltage_solutions.insert(vec![0; joltages.len()].into(), Some(0));

    fn configure_recursive(
        buttons: &[Bitfield],
        joltages: Box<[Joltage]>,
        parity_combos: &HashMap<Bitfield, Option<Vec<usize>>>,
        joltage_solutions: &mut HashMap<Box<[Joltage]>, Option<u64>>,
    ) -> Option<u64> {
        // First, a preliminary check: do we already know how many button presses this particular set of joltages takes?
        // Note that this also covers the case of all zeroes.
        if let Some(answer) = joltage_solutions.get(&joltages).copied() {
            return answer;
        }

        // Okay, now: what parity does this branch of the recursion tree need to figure out the presses for? Which
        // button combinations will yield that parity, if any?
        let curr_parity = parity(&joltages);
        let curr_combos = parity_combos[&curr_parity].as_deref();

        // Okay, for each of those possible button combinations (if any), which one yields the best solution?
        let min_presses = curr_combos.and_then(|combos| {
            let all_solutions = combos.into_iter().filter_map(|&button_mask| -> Option<u64> {
                // Apply this set of buttons to our joltages, what do we get? If applying the buttons goes below 0 jolts
                // on any counter, it's not a valid solution; short-circuit.
                let mut next_joltages = joltages.clone();
                for &button in buttons.iter().bit_filter(button_mask) {
                    for joltage in next_joltages.iter_mut().bit_filter(button) {
                        *joltage = joltage.checked_sub(1)?;
                    }
                }

                // At this point, if we can somehow figure out the optimal way to reach `next_joltages`, we would simply
                // have to perform `curr_presses` to get the final result we want. Since `curr_presses` were determined
                // simply based on *parity*, applying them should leave all the joltages as even numbers. That is,
                // they're all divisible by two! So, we can divide `next_joltages` by two, figure out *those* joltages'
                // optimal solution, then multiply the answer we get by two.
                for joltage in next_joltages.iter_mut() {
                    *joltage >>= 1;
                }

                let next_presses = configure_recursive(buttons, next_joltages, parity_combos, joltage_solutions)?;
                let curr_presses = button_mask.count_ones() as u64;
                Some(curr_presses + (next_presses << 1))
            });

            all_solutions.min()
        });

        // If we didn't already know the answer, store it before returning.
        joltage_solutions.insert(joltages, min_presses);
        min_presses
    }

    configure_recursive(buttons, joltages, parity_combos, &mut joltage_solutions).expect(AT_LEAST_ONE)
}

/// Determines the parity of a series of [Joltage] counters.
fn parity(counters: &[Joltage]) -> Bitfield {
    let mut bits: Bitfield = 0;
    for i in 0..counters.len() {
        let b = counters[i] & 1; // 1 for odd, 0 for even
        bits |= b << i;
    }
    bits
}

/// An extension trait that adds the [`bit_filter`][IterBitExt::bit_filter] method to iterators.
trait IterBitExt: Iterator + Sized {
    /// Filters items in this iterator based on the given bitmask and their indices.
    ///
    /// # Example
    ///
    /// ```
    /// let x = [1, 2, 3, 4, 5, 6, 7, 8];
    ///
    /// // Select items 0, 1, 4, and 7:
    /// let y = x.into_iter().bit_filter(0b10010011).collect::<Vec<_>>();
    ///
    /// assert_eq!(&y, &[1, 2, 5, 8]);
    /// ```
    fn bit_filter(self, mask: usize) -> BitFilter<Self> {
        BitFilter { iter: self.enumerate(), mask }
    }
}

/// `IterBitExt` is automatically implemented for all iterators.
impl<I: Iterator + Sized> IterBitExt for I {}

/// An iterator that filters its contents based on a bitmask. See [`IterBitExt::bit_filter`] for details.
struct BitFilter<I: Iterator> {
    iter: std::iter::Enumerate<I>,
    mask: usize,
}

impl<I: Iterator> Iterator for BitFilter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (i, item) = self.iter.next()?;
            // Handle possible left-shift overflow for iterators >=64 bits long:
            if i >= (usize::BITS as usize) {
                return None;
            } else if bit_set!(self.mask, i) {
                return Some(item);
            }
        }
    }
}
