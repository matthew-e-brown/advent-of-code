pub enum FuelMode {
    Constant,
    Binomial,
}

impl FuelMode {
    fn compute(&self, alignment: usize, position: usize) -> usize {

        fn abs_diff(a: usize, b: usize) -> usize {
            if a < b { b - a } else { a - b }
        }

        match self {
            FuelMode::Constant => abs_diff(alignment, position),
            FuelMode::Binomial => {
                let n = abs_diff(alignment, position);
                n * (n + 1) / 2
            },
        }
    }
}


pub fn parse(string: &str) -> Result<Vec<usize>, String> {
    string
        .split(",")
        .map(|s| {
            s.parse().or_else(|_| Err(format!("{} is not a positive number.", s)))
        })
        .collect()
}


pub fn run(crabs: &Vec<usize>, fuel_mode: FuelMode) -> (usize, usize) {

    let width = *crabs.iter().max().or(Some(&0)).unwrap();
    let mut min_fuel = usize::MAX;
    let mut position = 0;

    'outer: for alignment in 0..=width {

        let mut total_fuel = 0;

        for &position in crabs.iter() {
            let fuel = fuel_mode.compute(alignment, position);

            total_fuel += fuel;

            if total_fuel >= min_fuel {
                continue 'outer;
            }
        }

        min_fuel = total_fuel;
        position = alignment;
    }

    (position, min_fuel)
}


#[cfg(test)]
mod tests {

    use super::*;

    fn example_data() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn example_1() {
        let crabs = parse(example_data()).unwrap();
        assert_eq!(run(&crabs, FuelMode::Constant), (2, 37));
    }


    #[test]
    fn example_2() {
        let crabs = parse(example_data()).unwrap();
        assert_eq!(run(&crabs, FuelMode::Binomial), (5, 168));
    }

}