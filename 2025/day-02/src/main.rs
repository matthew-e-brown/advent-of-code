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

    let mut invalid_sum = 0_u64;
    for range in input {
        if aoc_utils::verbosity() > 0 {
            println!("Range {range:?}");
        }

        for id in range {
            if !valid_id(id, &mut buf) {
                if aoc_utils::verbosity() > 0 {
                    println!("\tID {id} is invalid.");
                }

                invalid_sum += id;
            }
        }
    }

    println!("Sum of all invalid IDs (part 1): {invalid_sum}");
}

fn valid_id(id: u64, buf: &mut String) -> bool {
    // Format the integer as a string into the buffer:
    buf.clear();
    write!(buf, "{id}").unwrap();

    // Is the string an even number of digits?
    if buf.len() % 2 == 0 {
        // If so, cut it in half. Then just check if the two are equal:
        let (a, b) = buf.split_at(buf.len() / 2);
        a != b
    } else {
        true
    }
}
