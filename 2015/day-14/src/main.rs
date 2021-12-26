use utils::cli::{parse_cli, Part};
use day14::{Reindeer, run_1, run_2};

fn main() {

    let config = parse_cli(
        "Reindeer Olympics",
        "Advent of Code 2015, day 14"
    );

    match Reindeer::new_list(&config.data) {
        Ok(reindeer) => {
            match config.part {
                Part::One => {
                    let (winner, distance) = run_1(&reindeer, 2503).unwrap();
                    println!("{} has won the race, at a distance of {} km!", winner, distance);
                },
                Part::Two => {
                    let (winner, points) = run_2(&reindeer, 2503).unwrap();
                    println!("{} has won, with {} points!", winner, points);
                }
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }

}