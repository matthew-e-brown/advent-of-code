use utils::cli::{parse_cli, Part};
use day13::{parse, run_1, run_2};

fn main() {

    let config = parse_cli(
        "Transparent Origami",
        "Advent of Code 2021, day 13"
    );

    let data = parse(&config.data);

    match data {
        Ok((mut paper, folds)) => {
            match config.part {
                Part::One => {
                    run_1(&mut paper, &folds[0]);
                    println!("Total dots after folding: {}", paper.count());
                },
                Part::Two => {
                    run_2(&mut paper, &folds);
                    println!("Paper after folding:\n{}", paper);
                }
            }
        },
        Err(e) => eprintln!("{}", e),
    }

}
