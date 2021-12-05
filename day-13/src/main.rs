use utils::cli::{parse_cli, Part};
use day13::{create_table, run_1, run_2};


fn main() {

    let config = parse_cli(
        "Knights of the Dinner Table",
        "Advent of Code 2015, day 13"
    );

    match create_table(&config.data) {
        Ok(table) => {
            let (delta, order) = match config.part {
                Part::One => run_1(&table),
                Part::Two => run_2(&table),
            };

            println!(
                "The most optimal seating order is\n  {}\nwith a happiness delta of {}.",
                order.join(", "), delta
            );
        },
        Err(e) => eprintln!("Error: {}", e),
    }

}