mod input;

fn main() {
    let input = aoc_utils::puzzle_input();
    let (shapes, regions) = input::parse_input(input).unwrap();

    println!("{regions:?}");
    println!("{shapes:#?}");
}
