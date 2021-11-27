use utils::cli::{Part, parse_cli};
use day9::{run_1, run_2};

fn main() {

    let config = parse_cli(
        "All in a Single Night",
        "Advent of Code 2015, day 9"
    );

    // let result = run_1(&config.data);
    let result = match config.part {
        Part::One => run_1(&config.data),
        Part::Two => run_2(&config.data),
    };

    match result {
        Ok((v, p)) => {
            let s = match config.part {
                Part::One => "shortest",
                Part::Two => "longest",
            };

            println!("The {} path that covers all cities is:\n\t{}\n\twith length {}", s, p, v);
        },
        Err(e) => eprint!("Error: {}", e),
    }

}