use utils::{truncate, cli::{parse_cli, Part}};
use day3::{directions_from_string, run_1, run_2};


fn main() {

    let config = parse_cli(
        "Perfectly Spherical Houses in a Vacuum",
        "Advent of Code 2015, day 3"
    );

    for string in config.data {
        let display = truncate(&string, 12);

        match directions_from_string(&string) {
            Ok(sequence) => {

                let result = match config.part {
                    Part::One => run_1(&sequence),
                    Part::Two => run_2(&sequence),
                };

                println!("Sequence '{:>12}' results in {} houses getting presents.", display, result);

            },
            Err(e) => eprintln!("Sequence '{:>12}': {}", display, e),
        }
    }
}