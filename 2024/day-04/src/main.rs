use aoc_utils::Grid;

mod part1;
mod part2;

fn main() {
    let input = aoc_utils::puzzle_input();
    let grid = Grid::from_lines(input.lines()).unwrap();

    // Part 1 and 2 are different enough (at least in the way I chose to implement them) that I'll just do them both in
    // separate modules.
    println!("Number of 'XMAS' found (part 1): {}", part1::main(&grid));
    println!("Number of 'X-MAS' found (part 2): {}", part2::main(&grid));
}
