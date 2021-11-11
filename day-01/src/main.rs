use utils::parse_cli;
use day1::{directions_from_string, run_1};


fn main() {

    let config = parse_cli(
        "Not Quite Lisp",
        "Advent of Code 2015, day 1"
    );

    for string in config.data {
        let mut display_string = string.clone();

        if display_string.len() > 12 {
            display_string.truncate(9);
            display_string.push_str("...");
        }

        match directions_from_string(&string) {
            Ok(sequence) => {
                let result = run_1(&sequence);
                println!("Sequence '{:>12}' puts Santa on floor {}", display_string, result);
            },
            Err(e) => eprint!("Sequence '{:>12}': {}", display_string, e),
        }
    }
}