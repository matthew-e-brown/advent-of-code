use utils::cli::{parse_cli, Part};
use day12::{run_1, run_2};


fn main() {

    let config = parse_cli(
        "JSAbacusFramework.io",
        "Advent of Code 2015, day 12"
    );


    // For now, just pretend each line is an entire JSON input (because, for puzzle-input.json, it is).
    // Will need to update utils to handle getting the whole file as one string.

    for line in config.data {

        let (result, message) = match config.part {
            Part::One => (run_1(&line) , "The sum of all numbers in that JSON is "),
            Part::Two => (run_2(&line) , "The sum of all numbers in that JSON, excluding \"red\" objects, is "),
        };

        match result {
            Ok(sum) => {
                print!("{}", message);
                println!("{}", sum);
            },
            Err(e) => eprintln!("JSON error: {}", e)
        }
    }

}
