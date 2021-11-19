use utils::cli::{Part, parse_cli};
use day7::{run_1, run_2};


fn main() {

    let config = parse_cli(
        "Some Assembly Required",
        "Advent of Code 2015, day 7"
    );


    let result = match config.part {
        Part::One => run_1(&config.data),
        Part::Two => run_2(&config.data),
    };


    match result {
        Ok(n) => {
            println!("That configuration results in 'a' having the value {}.", n);
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }

}