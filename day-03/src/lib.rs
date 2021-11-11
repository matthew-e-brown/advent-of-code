use std::collections::HashSet;


pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


pub fn directions_from_string(string: &str) -> Result<Vec<Direction>, &'static str> {
    string.chars().map(|c| {
        match c {
            '^' => Ok(Direction::Up),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            'v' | 'V' => Ok(Direction::Down),
            _ => Err("Encountered malformed sequence."),
        }
    }).collect()
}


pub fn run_1(sequence: &Vec<Direction>) -> usize {
    let mut current_pos = (0, 0);

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for direction in sequence {
        match direction {
            Direction::Up => current_pos.1 += 1,
            Direction::Down => current_pos.1 -= 1,
            Direction::Left => current_pos.0 -= 1,
            Direction::Right => current_pos.0 += 1,
        }

        if let None = visited.get(&current_pos) {
            visited.insert(current_pos.clone());
        }
    }

    visited.len()
}


pub fn run_2(sequence: &Vec<Direction>) -> usize {
    let mut position_1 = (0, 0);
    let mut position_2 = (0, 0);
    let mut one_or_two = true;

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for direction in sequence {
        let current_pos = if one_or_two { &mut position_1 } else { &mut position_2 };

        match direction {
            Direction::Up => current_pos.1 += 1,
            Direction::Down => current_pos.1 -= 1,
            Direction::Left => current_pos.0 -= 1,
            Direction::Right => current_pos.0 += 1,
        }

        one_or_two = !one_or_two;

        if let None = visited.get(current_pos) {
            visited.insert(current_pos.clone());
        }
    }

    visited.len()
}



#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case(">",           2 ; "case 1")]
    #[test_case("^>v<",        4 ; "case 2")]
    #[test_case("^v^v^v^v^v",  2 ; "case 3")]
    fn part_1(string: &str, result: usize) {
        let sequence = directions_from_string(string).unwrap();
        assert_eq!(run_1(&sequence), result);
    }

    #[test_case("^v",           3 ; "case 1")]
    #[test_case("^>v<",         3 ; "case 2")]
    #[test_case("^v^v^v^v^v",  11 ; "case 3")]
    fn part_2(string: &str, result: usize) {
        let sequence = directions_from_string(string).unwrap();
        assert_eq!(run_2(&sequence), result);
    }

}