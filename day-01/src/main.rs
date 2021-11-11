use utils::{parse_cli, Part};
use day1::{directions_from_string, run_1, run_2};


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
                match config.part {
                    Part::One => {
                        let result = run_1(&sequence);
                        println!("Sequence '{:>12}' puts Santa on floor {}", display_string, result);
                    }
                    Part::Two => {
                        let result = run_2(&sequence);
                        println!("Sequence '{:>12}' puts Santa in the basement at position {}", display_string, result);
                    }
                }
            },
            Err(e) => eprint!("Sequence '{:>12}': {}", display_string, e),
        }
    }
}