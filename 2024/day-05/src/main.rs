fn main() {
    let input = aoc_utils::puzzle_input();

    let mut lines = input.lines();
    let rule_lines = lines.by_ref().take_while(|line| line.trim().len() > 0);

    let mut sorted_mid_sums = 0;
    let mut unsorted_mid_sums = 0;

    println!("Sum of already-sorted middle elements (part 1): {}", sorted_mid_sums);
    println!("Sum of freshly-sorted middle elements (part 2): {}", unsorted_mid_sums);
}
