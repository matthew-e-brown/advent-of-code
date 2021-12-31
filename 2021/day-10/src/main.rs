use utils::cli::parse_cli;
use day10::{run_1, Line};

fn main() {

    let config = parse_cli(
        "Syntax Scoring",
        "Advent of Code 2021, day 10"
    );

    let lines = config.data
        .iter()
        .map(|s| Line::new(s))
        .collect::<Result<Vec<_>, String>>();

    match lines {
        Ok(lines) => {
            let result = run_1(&lines);
            println!("The total error score of all those lines is {}", result);
        },
        Err(e) => eprintln!("Error: {}", e),
    }

}