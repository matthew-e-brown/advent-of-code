pub enum Direction {
    Up,
    Down,
}


pub fn directions_from_string(string: &str) -> Result<Vec<Direction>, &'static str> {
    string.chars().map(|c| {
        match c {
            '(' => Ok(Direction::Up),
            ')' => Ok(Direction::Down),
            _ => Err("Malformed input sequence."),
        }
    }).collect()
}


pub fn run_1(sequence: &Vec<Direction>) -> isize {
    let mut floor = 0;

    for dir in sequence.iter() {
        match dir {
            Direction::Up => floor += 1,
            Direction::Down => floor -= 1,
        }
    }

    floor
}


pub fn run_2(sequence: &Vec<Direction>) -> usize {
    let mut floor: isize = 0;

    for (p, dir) in sequence.iter().enumerate() {
        match dir {
            Direction::Up => floor += 1,
            Direction::Down => floor -= 1,
        }

        if floor < 0 { return p + 1; }
    }

    0
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("(())",      0 ; "case 1")]
    #[test_case("()()",      0 ; "case 2")]
    #[test_case("(((",       3 ; "case 3")]
    #[test_case("(((",       3 ; "case 4")]
    #[test_case("))(((((",   3 ; "case 5")]
    #[test_case("())",      -1 ; "case 6")]
    #[test_case("))(",      -1 ; "case 7")]
    #[test_case(")))",      -3 ; "case 8")]
    #[test_case(")())())",  -3 ; "case 9")]
    fn part_1(str_sequence: &str, result: isize) {
        let sequence = directions_from_string(str_sequence).unwrap();
        assert_eq!(run_1(&sequence), result);
    }


    #[test_case(")",      1 ; "case 1")]
    #[test_case("()())",  5 ; "case 2")]
    fn part_2(str_sequence: &str, result: usize) {
        let sequence = directions_from_string(str_sequence).unwrap();
        assert_eq!(run_2(&sequence), result);
    }

}