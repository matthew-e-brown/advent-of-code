use utils::{cli::{parse_cli, Part}, truncate};
use day6::{school_from_string, run};


fn main() {

    let config = parse_cli(
        "Lanternfish",
        "Advent of Code 2021, day 6"
    );

    for string in config.data {

        let display = truncate(&string, 12);

        match school_from_string(&string) {
            Ok(fishies) => {

                let days = match config.part {
                    Part::One => 80,
                    Part::Two => 256,
                };

                let result = run(&fishies, days);

                println!("Sequence '{:>12}' results in {} fishies after {} days.", display, result, days);

            },
            Err(e) => eprintln!("Sequence '{:>12}': {}", display, e)
        }

    }

}