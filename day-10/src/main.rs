use utils::{cli::{parse_cli, Part}, truncate};
use day10::run;

fn main() {

    let config = parse_cli(
        "Elves Look, Elves Say",
        "Advent of Code 2015, day 10"
    );


    for string in config.data {
        let display = truncate(&string, 12);

        let result = run(&string, match config.part {
            Part::One => 40, // part one, 40 times
            Part::Two => 50, // part two, 50 times
        });

        match result {
            Ok(output) => println!(
                "The output of sequence '{:>12}' is {} characters long.",
                display, output.len()
            ),
            Err(e) => eprint!("Sequence '{:>12}': {}", display, e),
        }
    }

}
