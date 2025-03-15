use std::collections::BTreeMap;

const MAX_BLINK: usize = 75;

fn main() {
    let input = aoc_utils::puzzle_input();

    let stones = input
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("puzzle input should contain valid numbers"));

    // Because every stone changes and splits entirely independently of all the other stones around it, the order of the
    // stones is actually completely irrelevant. It's a red-herring in the instructions. So, instead of actually storing
    // the stones, we can simply store _how many_ of each stone there is; even if we have thousands of the same number
    // line, we only need 2×8 bytes, instead of the thousands of bytes it would've taken before (when we were using a
    // Vec to simulate the line, for part 1).
    let mut list1 = stones.map(|stone| (stone, 1)).collect::<BTreeMap<u64, u64>>();
    let mut list2 = BTreeMap::new();

    let mut src_list = &mut list1;
    let mut dst_list = &mut list2;

    let mut stone_counts = [0; MAX_BLINK];
    for blink in 0..MAX_BLINK {
        dst_list.clear();

        for (&stone, &count) in src_list.iter() {
            if stone == 0 {
                *(dst_list.entry(1).or_insert(0)) += count;
            } else if let Some([l, r]) = split_digits(stone) {
                *(dst_list.entry(l).or_insert(0)) += count;
                *(dst_list.entry(r).or_insert(0)) += count;
            } else {
                *(dst_list.entry(stone * 2024).or_insert(0)) += count;
            }
        }

        stone_counts[blink] = dst_list.iter().fold(0, |acc, (_, &count)| acc + count);
        std::mem::swap(&mut src_list, &mut dst_list);
    }

    println!("Stone counts over time: {stone_counts:?}\n");
    println!("Number of stones after blinking 25 times (part 1): {}", stone_counts[25 - 1]);
    println!("Number of stones after blinking 75 times (part 2): {}", stone_counts[75 - 1]);
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
