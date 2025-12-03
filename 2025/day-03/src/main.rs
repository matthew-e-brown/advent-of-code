// cspell:words joltage

fn main() {
    let input = aoc_utils::puzzle_input();
    let banks = input.lines();

    let mut total_joltage = 0_usize;
    for bank in banks {
        let (joltage, _, _) = max_joltage(bank);
        total_joltage += joltage as usize;
    }

    println!("Total output joltage (part 1): {total_joltage}");
}

fn max_joltage(bank: &str) -> (u8, usize, usize) {
    assert!(bank.chars().all(|c| c.is_ascii_digit()), "invalid puzzle input: banks should be digits only");
    assert!(bank.len() >= 2, "invalid puzzle input: all banks should have at least two batteries");

    // We need to select exactly two (non-contiguous digits) from the bank so as to maximize their concatenated value.
    // This essentially amounts to finding the biggest two digits. But a larger first digit always makes the number
    // bigger, no matter what the second digit is. So, the strategy is:
    // - Find the largest digit in the first 0..n-1 characters, at position i.
    // - Find the largest digit in the i..n characters. Will either be the last character or a larger one somewhere
    //   earlier.

    // ...part 2 is gonna make me extend this to N batteries, isn't it?

    let bytes = bank.as_bytes();
    let n = bank.len();

    let digits1 = bytes.into_iter().map(|ascii| ascii - b'0').enumerate();
    let digits2 = digits1.clone(); // Need to iterate twice

    // Can unwrap because n-1 is at least 1, will always be at least 1 digit.
    let (i, b1) = max_by_key(digits1.take(n - 1), |(_, d)| *d).unwrap();
    let (j, b2) = max_by_key(digits2.skip(i + 1), |(_, d)| *d).unwrap();
    let joltage = b1 * 10 + b2;

    if aoc_utils::verbosity() > 0 {
        print!("Batteries {i:2} and {j:2}: ");
        if aoc_utils::verbosity() > 1 {
            // Print the selected digits in a different colour:
            const C: &str = "\x1b[0;31m"; // red
            const X: &str = "\x1b[0m"; // reset
            let a = &bank[0..i]; // Up to, not including b1
            let b = &bank[i + 1..j]; // Between b1 and b2
            let c = &bank[j + 1..]; // After b2
            print!("{a}{C}{b1}{X}{b}{C}{b2}{X}{c}");
        } else {
            print!("{b1} + {b2}");
        }
        println!(" => {joltage}");
    }

    (joltage, i, j)
}

/// Works exactly like [`std::iter::Iterator::max_by_key`], except that, in the case of multiple equally maximal
/// elements, the *first* element is returned.
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
