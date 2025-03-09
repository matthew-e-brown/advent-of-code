fn main() {
    let input = aoc_utils::puzzle_input();

    let stones = input
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("puzzle input should contain valid numbers"));

    let mut buff1 = Vec::new();
    let mut buff2 = Vec::new();

    let mut total_25 = 0;
    let mut total_75 = 0;
    for stone in stones {
        let [s25, s75] = process_stone(stone, [25, 75], (&mut buff1, &mut buff2));
        total_25 += s25;
        total_75 += s75;
    }

    println!("Number of stones after blinking 25 times (part 1): {}", total_25);
    println!("Number of stones after blinking 75 times (part 2): {}", total_75);
}

/// Takes a single stone and watches what happens when it is blinked at a certain number of times.
///
/// `checks` is an array of blink-counts at which to "check in." For each value in this array, the stones are tallied;
/// the result of each count is returned in an array of the same length. `checks` **must** be in ascending order.
///
/// Buffers are borrowed from outside to avoid reallocating new buffers for each successive stone.
fn process_stone<const N: usize>(stone: u64, checks: [u32; N], buffers: (&mut Vec<u64>, &mut Vec<u64>)) -> [usize; N] {
    assert!(checks.is_sorted(), "checks should be in order");

    let mut r = 0;
    let mut results = [0; N];
    let Some(&max) = checks.last() else {
        return [0; N]; // Only possible when N == 0.
    };

    let (mut src_buff, mut dst_buff) = buffers;
    src_buff.clear();
    dst_buff.clear();
    src_buff.push(stone);

    for blink in 1..=max {
        dst_buff.clear();

        for &stone in src_buff.iter() {
            if stone == 0 {
                dst_buff.push(1);
            } else if let Some([l, r]) = split_digits(stone) {
                dst_buff.push(l);
                dst_buff.push(r);
            } else {
                dst_buff.push(stone * 2024);
            }
        }

        if blink == checks[r] {
            results[r] = dst_buff.len();
            r += 1;
        }

        std::mem::swap(&mut src_buff, &mut dst_buff);
    }

    results
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
