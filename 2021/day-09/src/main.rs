use utils::cli::{parse_cli, Part};
use day9::{parse, run_1, run_2};

fn main() {

    let config = parse_cli(
        "Smoke Basin",
        "Advent of Code 2021, day 9"
    );

    match parse(&config.data) {
        Ok(map) => match config.part {
            Part::One => {
                let result = run_1(&map);
                println!("The sum of all risk-levels is {}.", result);
            },
            Part::Two => {
                let result = run_2(&map);
                println!("The product of the top three basin-sizes is {}.", result);
            },
        },
        Err(e) => eprintln!("{}", e),
    }
}
