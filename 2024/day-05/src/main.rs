mod graph;
mod part1;

use graph::PageGraph;


fn main() {
    let input = aoc_utils::puzzle_input();
    let mut lines = input.lines();

    let rule_lines = lines.by_ref().take_while(|line| line.trim().len() > 0);
    let graph = PageGraph::from_input(rule_lines);

    // println!("{graph:#?}");

    let sorted_mid_sums = part1::main(&graph, lines);
    println!("Sum of already-sorted middle elements (part 1): {}", sorted_mid_sums);
}


/// Parses a comma-separated list string of page numbers into a vector.
fn parse_nums<S: AsRef<str>>(line: S) -> Vec<u32> {
    line.as_ref()
        .split(',')
        .map(|n| n.parse::<u32>().expect("invalid page number"))
        .collect::<Vec<_>>()
}
