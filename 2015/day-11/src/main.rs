use utils::{cli::{parse_cli, Part}, truncate};
use day11::run;

fn main() {

    let config = parse_cli(
        "Corporate Policy",
        "Advent of Code 2015, day 11"
    );

    // Wrap the two 'run' calls in a single call with '?' so we don't need like 4 matches in main body
    fn wrapper(s: &str, part: &Part) -> Result<String, &'static str> {
        let result = run(s)?;
        match part {
            Part::One => Ok(result),
            Part::Two => run(&result),
        }
    }

    for string in config.data {

        match wrapper(&string, &config.part) {
            Ok(res) => {
                match config.part {
                    Part::One => println!("Next password after '{}' is '{}'", string, res),
                    Part::Two => println!("Two passwords after '{}' is '{}'", string, res),
                }
            }
            Err(e) => {
                let display = truncate(&string, 8);
                eprintln!("'{}': {}", display, e);
            },
        }

    }

}