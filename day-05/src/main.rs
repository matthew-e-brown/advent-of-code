use utils::cli::{parse_cli, Part};

use day5::{run_1, run_2};


fn main() {

    let config = parse_cli(
        "Doesn't He Have Intern-Elves For This?",
        "Advent of Code 2015, day 5"
    );

    let result = match config.part {
        Part::One => run_1(&config.data),
        Part::Two => run_2(&config.data),
    };

    println!(
        "Of those strings, {} of them are nice; {} are naughty.",
        result, config.data.len() - result
    );

}