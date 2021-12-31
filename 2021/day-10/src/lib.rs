#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BracketType {
    Round,
    Square,
    Curly,
    Angled,
}

impl BracketType {
    pub fn score(&self) -> usize {
        match self {
            BracketType::Round =>   3,
            BracketType::Square =>  57,
            BracketType::Curly =>   1197,
            BracketType::Angled =>  25137,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bracket {
    Opening(BracketType),
    Closing(BracketType),
}


pub struct Line {
    inner: Vec<Bracket>
}

impl Line {

    pub fn new(line: &str) -> Result<Self, String> {
        let mut brackets = Vec::new();

        for c in line.chars() {
            let b = match c {
                '(' => Bracket::Opening(BracketType::Round),
                '[' => Bracket::Opening(BracketType::Square),
                '{' => Bracket::Opening(BracketType::Curly),
                '<' => Bracket::Opening(BracketType::Angled),
                ')' => Bracket::Closing(BracketType::Round),
                ']' => Bracket::Closing(BracketType::Square),
                '}' => Bracket::Closing(BracketType::Curly),
                '>' => Bracket::Closing(BracketType::Angled),
                _ => return Err(format!("Encountered malformed line: {}", line)),
            };

            brackets.push(b);
        }

        Ok(Self { inner: brackets })
    }

    /// Returns `None` if the line is not corrupted (I.E. if it is 'incomplete' instead of corrupted), and returns
    /// an option that contains the broken bracket otherwise.
    pub fn is_corrupted(&self) -> Option<BracketType> {
        let mut stack = Vec::new();
        for &bracket in self.inner.iter() {
            match bracket {
                Bracket::Opening(bracket) => stack.push(bracket),
                // If this is a closing bracket, check that it matches the most recently pushed opening bracket
                Bracket::Closing(current) => match stack.pop() {
                    Some(expected) if expected == current => continue,
                    _ => return Some(current),
                },
            }
        }

        None
    }

}


pub fn run_1(data: &Vec<Line>) -> usize {
    data
        .iter()
        .map(|line| line.is_corrupted())
        .fold(0, |acc, cur| acc + match cur {
            Some(bracket_type) => bracket_type.score(),
            None => 0,
        })
}


#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case("[({(<(())[]>[[{[]{<()<>>", None; "case 1")]
    #[test_case("[(()[<>])]({[<{<<[]>>(",   None; "case 2")]
    #[test_case("{([(<{}[<>[]}>{[]{[(<()>", Some(BracketType::Curly); "case 3")]
    #[test_case("(((({<>}<{<{<>}{[]{[]{}",  None; "case 4")]
    #[test_case("[[<[([]))<([[{}[[()]]]",   Some(BracketType::Round); "case 5")]
    #[test_case("[{[{({}]{}}([{[{{{}}([]",  Some(BracketType::Square); "case 6")]
    #[test_case("{<[[]]>}<{[{[{[]{()[[[]",  None; "case 7")]
    #[test_case("[<(<(<(<{}))><([]([]()",   Some(BracketType::Round); "case 8")]
    #[test_case("<{([([[(<>()){}]>(<<{{",   Some(BracketType::Angled); "case 9")]
    #[test_case("<{([{{}}[<[[[<>{}]]]>[]]", None; "case 10")]
    fn example(line: &str, expected: Option<BracketType>) {

        let line = Line::new(line);
        assert!(line.is_ok());

        let line = line.unwrap();

        assert_eq!(line.is_corrupted(), expected);
    }

}