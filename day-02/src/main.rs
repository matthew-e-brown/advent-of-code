use utils::{parse_cli, Part};
use day2::*;

fn main() {
    let config = parse_cli(
        "I Was Told There Would Be No Math",
        "Advent of Code 2015, day 2"
    );

    match boxes_from_strings(&config.data) {
        Ok(boxes) => {
            match config.part {
                Part::One => {
                    let result = run_1(&boxes);
                    println!("Those gifts add up to a total required {} sqft of wrapping paper.", result);
                },
                Part::Two => {
                    eprintln!("Not implemented yet.");
                }
            }
        },
        Err(e) => eprintln!("{}", e),
    };
}
