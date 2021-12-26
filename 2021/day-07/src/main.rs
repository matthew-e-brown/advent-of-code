use utils::{cli::{parse_cli, Part}, truncate};
use day7::{parse, run, FuelMode};

fn main() {

    let config = parse_cli(
        "The Treachery of Whales",
        "Advent of Code 2021, day 7"
    );

    for string in config.data {
        let display = truncate(&string, 12);

        match parse(&string) {
            Ok(crabs) => {
                let fuel_mode = match config.part {
                    Part::One => FuelMode::Constant,
                    Part::Two => FuelMode::Binomial,
                };

                let (pos, fuel) = run(&crabs, fuel_mode);

                println!("Sequences {:>12}: Position {} is cheapest, costing {} fuel.", display, pos, fuel);
            },
            Err(e) => eprintln!("Sequence {:>12}: {}", display, e),
        }
    }

}
