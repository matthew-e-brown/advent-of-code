use utils::cli::parse_cli;
use day9::run_1;

fn main() {

    let config = parse_cli(
        "All in a Single Night",
        "Advent of Code 2015, day 9"
    );

    let result = run_1(&config.data);

    match result {
        Ok(n) => println!("With those distances, the shorted path is {}.", n),
        Err(e) => eprintln!("Error occurred: {}", e),
    }

}
