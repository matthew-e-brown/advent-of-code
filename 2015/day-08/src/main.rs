use utils::cli::{parse_cli, Part};
use day8::{run_1, run_2};


fn main() {

    let config = parse_cli(
        "Matchsticks",
        "Advent of Code 2015, day 8"
    );

    let result = match config.part {
        Part::One => run_1(&config.data),
        Part::Two => run_2(&config.data),
    };

    println!("For those strings, the result is {}", result);
}