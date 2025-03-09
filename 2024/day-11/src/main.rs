use std::fmt::{Display, Write};

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut buffer1 = input
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("puzzle input should contain valid numbers"))
        .collect::<Vec<_>>();
    let mut buffer2 = Vec::with_capacity(buffer1.capacity());

    let mut in_buff = &mut buffer1;
    let mut out_buff = &mut buffer2;

    #[allow(unused)] // Just used for debugging.
    let mut fmt_string = String::new();

    println!("Input: {}", fmt_list(in_buff.iter(), &mut fmt_string));

    for _blink in 1..=25 {
        out_buff.clear();

        for &stone in in_buff.iter() {
            if stone == 0 {
                out_buff.push(1);
            } else if let Some([l, r]) = split_digits(stone) {
                out_buff.push(l);
                out_buff.push(r);
            } else {
                out_buff.push(stone * 2024);
            }
        }

        // println!("After {_blink} blinks:\n{}", fmt_list(out_buff.iter(), &mut fmt_string));

        // Swap the two references for next time:
        std::mem::swap(&mut in_buff, &mut out_buff);
    }

    // Whichever buffer was most recently pushed into was swapped to be `in_buff` at the end of the last loop.
    let buffer = in_buff;
    println!("Number of stones after blinking 25 times (part 1): {}", buffer.len());
}

// For debugging. Formats a list of displayable items into a single string, without allocating a bunch of intermediate
// strings.
#[allow(unused)]
fn fmt_list(items: impl Iterator<Item = impl Display>, buf: &mut String) -> &str {
    buf.clear();
    for x in items {
        write!(buf, "{x} ").unwrap();
    }
    buf.trim_end()
}

fn split_digits(n: u64) -> Option<[u64; 2]> {
    let num_digits = if n == 0 { 1 } else { n.ilog10() + 1 };
    if num_digits % 2 == 0 {
        let d = num_digits / 2; // e.g., 1012 => d = 2.
        let s = 10u64.pow(d); // e.g., 1012 => s = 10^2 = 100.
        // - To extract the left half, divide by s: 1012 / 100 = 10.
        // - To extract the right half, modulo by s. 1012 % 100 = 12.
        Some([n / s, n % s])
    } else {
        None
    }
}
