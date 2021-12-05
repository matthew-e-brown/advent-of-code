use utils::cli::parse_cli;
use day14::{Reindeer, run_1};

fn main() {

    let config = parse_cli(
        "Reindeer Olympics",
        "Advent of Code 2015, day 14"
    );

    match Reindeer::new_from_list(&config.data) {
        Ok(reindeer) => {
            let (winner, distance) = run_1(&reindeer).unwrap();
            println!("{} has won the race, at a distance of {} km!", winner, distance);
        },
        Err(e) => eprintln!("Error: {}", e),
    }

}