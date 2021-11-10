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
            _ => Err("Malformed sequence."),
        }
    }).collect()
}


pub fn run(sequence: &Vec<Direction>) -> i32 {
    let mut current_pos = (0, 0);
    let mut total_gifts = 1;

    let mut visited = HashSet::new();
    visited.insert(current_pos.clone());

    for direction in sequence {
        match direction {
            Direction::Up => current_pos.1 += 1,
            Direction::Down => current_pos.1 -= 1,
            Direction::Left => current_pos.0 -= 1,
            Direction::Right => current_pos.0 += 1,
        }

        if let None = visited.get(&current_pos) {
            visited.insert(current_pos.clone());
            total_gifts += 1;
        }
    }

    total_gifts
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_1() {
        let sequence = directions_from_string(">").unwrap();
        assert_eq!(run(&sequence), 2);
    }

    #[test]
    fn case_2() {
        let sequence = directions_from_string("^>v<").unwrap();
        assert_eq!(run(&sequence), 4);
    }

    #[test]
    fn case_3() {
        let sequence = directions_from_string("^v^v^v^v^v").unwrap();
        assert_eq!(run(&sequence), 2);
    }

}