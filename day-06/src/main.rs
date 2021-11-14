use utils::cli::parse_cli;

use day6::{instructions_from_strings, run_1};

fn main() {

    let config = parse_cli(
        "Probably a Fire Hazard",
        "Advent of Code 2015, day 6"
    );

    match instructions_from_strings(&config.data) {
        Ok(instructions) => {
            let result = run_1(&instructions);

            println!("That sequence of instructions results in {} lights on by the end.", result);
        },
        Err(e) => eprintln!("{}", e),
    }

}
