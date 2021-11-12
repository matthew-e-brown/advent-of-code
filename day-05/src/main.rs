use utils::cli::parse_cli;

use day5::part1;


fn main() {

    let config = parse_cli(
        "Doesn't He Have Intern-Elves For This?",
        "Advent of Code 2015, day 5"
    );

    let result = part1::run(&config.data);

    println!(
        "Of those strings, {} of them are nice; {} are naughty.",
        result, config.data.len() - result
    );

}