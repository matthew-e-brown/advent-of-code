use utils::{cli::{parse_cli, Part}, truncate};
use day6::run;


fn main() {

    let config = parse_cli(
        "Lanternfish",
        "Advent of Code 2021, day 6"
    );

    for string in config.data {

        let display = truncate(&string, 12);

        let days = match config.part {
            Part::One => 80,
            Part::Two => 256,
        };

        match run(&string, days) {
            Ok(n) => println!("Sequence '{:>12}' results in {} fishies after {} days.", display, n, days),
            Err(e) => eprintln!("Sequence '{:>12}': {}", display, e),
        }

    }

}