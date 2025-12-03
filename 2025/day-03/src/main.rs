// cspell:words joltage

fn main() {
    let input = aoc_utils::puzzle_input();

    // It would definitely be more efficient to do part 1 and 2 in a single pass, but then the debug output for both
    // parts is mixed together. I care too much about making things look pretty, so let's do them one at a time :)
    let total_joltage1 = run::<2>(input);
    let total_joltage2 = run::<12>(input);

    println!("Total output joltage with 2 batteries (part 1): {total_joltage1}");
    println!("Total output joltage with 12 batteries (part 2): {total_joltage2}");
}

fn run<const N: usize>(input: &str) -> usize {
    let mut total_joltage = 0;

    for bank in input.lines() {
        total_joltage += max_joltage::<N>(bank);
    }

    if aoc_utils::verbosity() > 0 {
        println!(); // Spacer
    }

    total_joltage
}

fn max_joltage<const N: usize>(bank: &str) -> usize {
    assert!(N > 0, "invalid program configuration: cannot ");
    assert!(bank.len() >= N, "invalid puzzle input: all banks should have at least {N} batteries");
    assert!(bank.chars().all(|c| c.is_ascii_digit()), "invalid puzzle input: banks should be digits only");

    // We need to select exactly N possibly noncontiguous digits from the bank so as to maximize their concatenated
    // value. Maximizing their value can be done by greedily selecting the largest digit as the leftmost digit in the
    // final value (e.g., even if we had 399, going up to 4__ will *always* be higher, even if there are no more 9s to
    // make up for the 2nd and 3rd digit we had). All we need to do is ensure we leave at least enough characters after
    // our left-most digit (or, rather, the current digit) to form the rest of the number.

    let n = bank.len();
    let digits = bank.bytes().map(|ascii| ascii - b'0').enumerate();

    let mut skip_to = 0;
    let mut bat_values = [0; N];
    let mut bat_indices = [0; N]; // Stored for debug printing in verbose mode

    for i in 0..N {
        // After selecting the 1st digit, we have N - 1 digits left to select. After the 2nd, there are only N - 2 to
        // select. We need to make sure we leave at least that many digits in the bank for subsequent iterations to
        // search through. Additionally, each subsequent iteration only needs to start after the previous.
        let digits = digits.clone().take(n - (N - i - 1)).skip(skip_to);
        let (idx, bat) = max_by_key(digits, |(_, d)| *d).unwrap();
        bat_values[i] = bat;
        bat_indices[i] = idx;
        skip_to = idx + 1;
    }

    let joltage = concat_digits(bat_values);

    if aoc_utils::verbosity() > 0 {
        debug_results(bank, bat_indices, joltage);
    }

    joltage
}

/// Finds the maximum element of an iterator using a key extraction function.
///
/// This method works almost exactly like [`Iterator::max_by_key`]. The only difference is that, when there are multiple
/// equally maximal elements, this function returns the *first* candidate; `Iterator::max_by_key` returns the *last.*
fn max_by_key<I: Iterator, B: Ord>(mut iter: I, mut f: impl FnMut(&I::Item) -> B) -> Option<I::Item> {
    let mut max_val = iter.next()?;
    let mut max_key = f(&max_val);

    for val in iter {
        let key = f(&val);
        if key > max_key {
            max_key = key;
            max_val = val;
        }
    }

    Some(max_val)
}

/// Concatenates a sequence of `N` digits into a single value.
fn concat_digits<const N: usize>(mut digits: [u8; N]) -> usize {
    let mut value = 0usize;
    let mut power = 1;
    digits.reverse();
    for d in digits {
        value += (d as usize) * power;
        power *= 10;
    }
    value
}

// I didn't really need to keep this... it was helpful for part 1, but extending it to work for Part 2 was probably a
// bit more than I needed. But I loved how it looked so much, I couldn't help myself! :)
fn debug_results<const N: usize>(bank: &str, indices: [usize; N], result: usize) {
    const ANSI_DIM: &str = "\x1b[38;5;240m";
    const ANSI_RESET: &str = "\x1b[0m";

    // Print the bank, but surround all the characters mentioned in `indices` with ANSI colour codes.
    print!("Bank: ");
    let mut i = 0;
    for (idx, digit) in bank.chars().enumerate() {
        if i < N && idx == indices[i] {
            print!("{digit}");
            i += 1;
        } else {
            print!("{ANSI_DIM}{digit}{ANSI_RESET}");
        }
    }

    println!(" => {result}");
}
