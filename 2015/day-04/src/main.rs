use utils::{truncate, cli::{parse_cli, Part}};

use day4::run;


fn main() {
    let config = parse_cli(
        "The Ideal Stocking Stuffer",
        "Advent of Code 2015, day 4"
    );

    let threshold = match config.part {
        Part::One => 5,
        Part::Two => 6,
    };

    for string in config.data {
        let display = truncate(&string, 12);

        let result = run(&string, threshold);
        println!("Input '{:>12}' has solution {} ({} zeroes)", display, result, threshold);
    }
}