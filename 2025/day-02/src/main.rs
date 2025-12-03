use std::fmt::Write;

fn main() {
    let input = aoc_utils::puzzle_input().trim().split(',').map(|range| {
        let (a, b) = range.split_once('-').expect("puzzle input should have dash-separated ranges");
        let a = a.parse::<u64>().expect("puzzle input should contain valid u64s");
        let b = b.parse::<u64>().expect("puzzle input should contain valid u64s");
        a..=b
    });

    // Simple scratch-space to avoid re-allocating a new string for every number:
    let mut buf = String::new();

    let mut invalid_sum1 = 0_u64;
    let mut invalid_sum2 = 0_u64;
    for range in input {
        if aoc_utils::verbosity() > 0 {
            println!("Range {range:?}");
        }

        for id in range {
            if invalid_id1(id, &mut buf) {
                if aoc_utils::verbosity() > 0 {
                    println!("\tID {id} is invalid.");
                }

                invalid_sum1 += id;
            }

            if invalid_id2(id, &mut buf) {
                if aoc_utils::verbosity() > 0 {
                    println!("\tID {id} is invalid (part 2).");
                }

                invalid_sum2 += id;
            }
        }
    }

    println!("Sum of all invalid IDs (part 1): {invalid_sum1}");
    println!("Sum of all invalid IDs (part 2): {invalid_sum2}");
}

fn invalid_id1(id: u64, buf: &mut String) -> bool {
    // Format the integer as a string into the buffer:
    buf.clear();
    write!(buf, "{id}").unwrap();

    // Is the string an even number of digits?
    if buf.len() % 2 == 0 {
        // If so, cut it in half. Then just check if the two are equal:
        let (a, b) = buf.split_at(buf.len() / 2);
        a == b
    } else {
        // Otherwise, the ID is valid.
        false
    }
}

fn invalid_id2(id: u64, buf: &mut String) -> bool {
    buf.clear();
    write!(buf, "{id}").unwrap();

    // Subsequence must be repeated at least twice to be invalid.
    if buf.len() < 2 {
        return false;
    }

    // We know this string is made entirely of ascii digits, since we just
    // formatted it ourselves. So we'll work with a slice of bytes instead
    // of UTF-8 chars.
    let buf = buf.as_bytes();

    // Since we need at least two repetitions, the repeating string can be at
    // most half the length of the string.
    'outer: for len in 1..=(buf.len() / 2) {
        // We only need to bother checking slices of this length if the overall
        // length is divisible by it; otherwise we could never fill the string
        // fully.
        if buf.len() % len != 0 {
            continue;
        }

        let sub_count = buf.len() / len;

        // Each sub-slice is `len` long, we have `sub_count` of them.
        // `sub_count` is at least 2, since `len` is at most half the full length.
        let sub1 = &buf[0..len];
        for i in 1..sub_count {
            let sub = &buf[len * i..len * (i + 1)];
            if sub != sub1 {
                continue 'outer;
            }
        }

        // If we get through all `sub_count` sub-slices, and they all matched, then
        // the string is invalid!
        return true;
    }

    // If we get through all sub-slices of all lengths... string is valid.
    false
}
