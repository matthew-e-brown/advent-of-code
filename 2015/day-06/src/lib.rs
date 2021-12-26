use regex::Regex;

#[derive(Clone, Copy)]
pub enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

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


fn execute<T: Copy>(
    instructions: &Vec<Instruction>,
    default: T,
    get_action: fn(Action) -> fn(&mut T) -> (),
) -> Vec<Vec<T>> {

    // To avoid a stack overflow, we need to allocate this data on the heap. To do so, we are forced to use a Vec
    // instead of an array. This works fine with 1000x1000 booleans, but any more complex type, like u32, causes an
    // overflow.
    let mut lights = vec![vec![default; 1000]; 1000];

    for instruction in instructions {

        let Instruction { start, end, action } = instruction;

        let action = get_action(*action);
        let x_range = if start.0 <= end.0 { start.0..=end.0 } else { end.0..=start.0 };
        let y_range = if start.1 <= end.1 { start.1..=end.1 } else { end.1..=start.1 };

        let lights = lights[x_range]
            .iter_mut()
            .flat_map(|sub_array| &mut sub_array[y_range.clone()]);

        for light in lights { action(light); }
    }

    lights
}


pub fn run_1(instructions: &Vec<Instruction>) -> usize {
    let lights = execute(
        instructions,
        false,
        |action| {
            match action {
                Action::Toggle =>   |light| *light = !(*light),
                Action::TurnOn =>   |light| *light = true,
                Action::TurnOff =>  |light| *light = false,
            }
        }
    );

    lights
        .iter()
        .flatten()
        .filter(|light| **light)
        .count()
}


pub fn run_2(instructions: &Vec<Instruction>) -> u32 {
    let lights = execute(
        instructions,
        0,
        |action| {
            match action {
                Action::Toggle =>  |light| *light += 2,
                Action::TurnOn =>  |light| *light += 1,
                Action::TurnOff => |light| *light = if *light > 0 { *light - 1 } else { 0 },
            }
        }
    );

    lights
        .iter()
        .flatten()
        .sum()
}


#[cfg(test)]
mod tests {

    use super::*;

    mod algorithm {
        use super::*;

        #[test]
        fn example_1() {
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

        #[test]
        fn example_2() {
            let instructions = vec![
                Instruction { action: Action::TurnOn, start: (0, 0), end: (0, 0), },
                Instruction { action: Action::Toggle, start: (0, 0), end: (999, 999), },
            ];

            assert_eq!(
                run_2(&instructions),
                (1) + (1000 * 1000 * 2)
            );
        }

    }


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
}