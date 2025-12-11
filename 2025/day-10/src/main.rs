// cspell:words joltage joltages

use aoc2025_10::Machine;

fn main() {
    let input = aoc_utils::puzzle_input();
    let machines = input.lines().map(|line| line.parse::<Machine>().unwrap()).collect::<Vec<_>>();

    println!("Parsed machines: {machines:#?}");
}
