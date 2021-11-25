use day12::run;
use utils::cli::parse_cli;


fn main() {

    let config = parse_cli(
        "JSAbacusFramework.io",
        "Advent of Code 2015, day 12"
    );


    // For now, just pretend each line is an entire JSON input (because, for puzzle-input.json, it is).
    // Will need to update utils to handle getting the whole file as one string.

    for line in config.data {
        match run(&line) {
            Ok(sum) => println!("The sum of all the numbers in that JSON is {}.", sum),
            Err(e) => eprintln!("JSON error: {}", e)
        }
    }

}
