use utils::cli::parse_cli;
use day7::run_1;


fn main() {

    let config = parse_cli(
        "Some Assembly Required",
        "Advent of Code 2015, day 7"
    );

    match run_1(&config.data) {
        Ok(n) => {
            println!("That configuration results in 'a' having the value {}.", n);
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }

}