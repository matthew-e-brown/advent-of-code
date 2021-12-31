use utils::cli::{parse_cli, Part};
use day10::{run_1, run_2, Line};

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
        Ok(lines) => match config.part {
            Part::One => {
                let result = run_1(&lines);
                println!("The total error score of all those lines is {}", result);
            },
            Part::Two => {
                let result = run_2(&lines);
                println!("The total completion score of all the incomplete lines is {}", result);
            },
        },
        Err(e) => eprintln!("Error: {}", e),
    }

}