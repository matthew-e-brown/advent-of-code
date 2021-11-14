use regex::Regex;

#[derive(Debug)]
pub enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

#[derive(Debug)]
pub struct Instruction {
    pub action: Action,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

pub fn instructions_from_strings(strings: &Vec<String>) -> Result<Vec<Instruction>, &'static str> {
    let re = Regex::new(
        r"^(toggle|turn (?:on|off)) \(?(\d{1,3}), ?(\d{1,3})\)? through \(?(\d{1,3}), ?(\d{1,3})\)?$"
    ).unwrap();

    // (toggle|on|off) (123, 456) through (123, 456)
    // ↑____________↑  ↑_↑   ↑_↑            ↑_↑  ↑_↑
    //       1          2     3              4    5
    //
    // Supports optional brackets and spaces in tuples.

    let mut instructions = Vec::new();

    for string in strings {
        if let Some(caps) = re.captures(string) {

            // Can unwrap on captures because regex is strict enough to enforce they all exist

            let action = match caps.get(1).unwrap().as_str() {
                "turn off" => Action::TurnOff,
                "turn on" => Action::TurnOn,
                "toggle" => Action::Toggle,
                _ => unreachable!(), // regex will only ever match one of the above variants
            };

            // Can unwrap on numbers because regex checks if the tuples are made from \d's. By using {1,3}, we also
            // guarantee that the indices are within range.

            let start = (
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str().parse().unwrap(),
            );

            let end = (
                caps.get(4).unwrap().as_str().parse().unwrap(),
                caps.get(5).unwrap().as_str().parse().unwrap(),
            );

            instructions.push(Instruction { action, start, end });
        } else {
            return Err("Encountered malformed instruction.");
        }
    }

    Ok(instructions)
}


pub fn run_1(instructions: &Vec<Instruction>) -> usize {
    // The lights all start off
    let mut lights = [[false; 1000]; 1000];

    for inst in instructions {

        let Instruction { start, end, action } = inst;

        // Do the check just this once and construct a closure, so that we don't have to potentially match 1000x1000
        // times.

        let action = match action {
            Action::TurnOff =>  |light: &mut bool| *light = false,
            Action::Toggle =>   |light: &mut bool| *light = !(*light),
            Action::TurnOn =>   |light: &mut bool| *light = true,
        };

        // Need to construct our slice with these if-elses because the rust (N..M) syntax breaks if N > M. Heaven forbid
        // anybody ever need to loop through something backwards.

        let x_range = if start.0 <= end.0 { start.0..=end.0 } else { end.0..=start.0 };
        let y_range = if start.1 <= end.1 { start.1..=end.1 } else { end.1..=start.1 };

        // Will have to test and see if this is actually faster than using x,y counters in a while loop...

        let lights = lights[x_range]
            .iter_mut()
            .flat_map(|sub| &mut sub[y_range.clone()]);

        for light in lights { action(light); }
    }

    lights
        .iter()
        .flatten()
        .filter(|light| **light)
        .count()
}


#[cfg(test)]
mod tests {

    use super::*;

    mod parsing {
        use super::*;


        #[test]
        fn completely_wrong() {
            let result = instructions_from_strings(&vec![
                "invalid string".to_owned(),
            ]);

            assert!(result.is_err());
        }

        #[test]
        fn mostly_valid() {
            // almost correct, but has extra text that we don't feel like parsing.
            let result = instructions_from_strings(&vec![
                "turn off lights 123,546 through 789,012".to_owned(),
            ]);

            assert!(result.is_err());
        }

        #[test]
        fn all_valid() {
            let result = instructions_from_strings(&vec![
                "toggle 461,550 through 564,900".to_owned(),
                "turn off 370,39 through 425,839".to_owned(),
                "turn on 464,858 through 833,915".to_owned(),
            ]);

            assert!(result.is_ok());
        }

    }

    mod algorithm {
        use super::*;

        #[test]
        fn example() {
            let instructions = vec![
                Instruction { action: Action::TurnOn, start: (0, 0), end: (999, 999), },
                Instruction { action: Action::Toggle, start: (0, 0), end: (999, 0), },
                Instruction { action: Action::TurnOff, start: (10, 10), end: (10, 10), },
            ];

            assert_eq!(
                run_1(&instructions),
                (1000 * 1000) - (1000 * 1) - (1 * 1)
            );
        }

    }
}