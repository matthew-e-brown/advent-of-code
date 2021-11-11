use utils::parse_cli;
use day3::{directions_from_string, run_1, run_2};


fn main() {

    let config = parse_cli(
        "Perfectly Spherical Houses in a Vacuum",
        "Advent of Code 2015, day 3"
    );

    for result in config.data {
        match result {
            Ok(string) => {
                let mut display_string = string.clone();

                if display_string.len() > 12 {
                    display_string.truncate(9);
                    display_string.push_str("...");
                }

                match directions_from_string(&string) {
                    Ok(sequence) => {
                        let result = if config.part == 1 { run_1(&sequence) } else { run_2(&sequence) };
                        println!("Sequence '{:>12}' results in {} houses getting presents.", display_string, result);
                    },
                    Err(e) => eprintln!("Sequence '{:>12}': {}", display_string, e),
                }
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}