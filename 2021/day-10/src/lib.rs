#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BracketType {
    Round,
    Square,
    Curly,
    Angled,
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

    fn check(&self) -> (Option<BracketType>, Option<Vec<BracketType>>) {
        let mut stack = Vec::new();
        for &bracket in self.inner.iter() {
            match bracket {
                Bracket::Opening(bracket) => stack.push(bracket),
                Bracket::Closing(current) => match stack.pop() {
                    // If this is a closing bracket, check that it matches the most recently pushed opening bracket
                    Some(expected) if expected == current => continue,
                    // If not, this is a syntax error
                    _ => return (Some(current), None)
                },
            }
        }

        (None, Some(stack))
    }

    fn check_corruption(&self) -> Option<BracketType> {
        self.check().0
    }

    fn check_incomplete(&self) -> Option<Vec<BracketType>> {
        self.check().1
    }
}


pub fn run_1(data: &Vec<Line>) -> usize {
    data
        .iter()
        .map(|line| line.check_corruption())
        .filter(|line| line.is_some())
        .fold(0, |acc, cur| {
            let score = match cur.unwrap() {
                BracketType::Round =>   3,
                BracketType::Square =>  57,
                BracketType::Curly =>   1197,
                BracketType::Angled =>  25137,
            };

            acc + score
        })
}


pub fn run_2(data: &Vec<Line>) -> usize {
    let scores = data
        .iter()
        .map(|line| line.check_incomplete())
        .filter(|line| line.is_some())
        .map(|line| {
            let stack = line.unwrap();
            stack.iter().rev().fold(0, |acc, &cur| {
                acc * 5 + match cur {
                    BracketType::Round =>   1,
                    BracketType::Square =>  2,
                    BracketType::Curly =>   3,
                    BracketType::Angled =>  4,
                }
            })
        })
        .collect::<Vec<_>>();

    scores[scores.len() / 2 + 1]
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
    fn example_1(line: &str, expected: Option<BracketType>) {

        let line = Line::new(line);
        assert!(line.is_ok());

        let line = line.unwrap();

        assert_eq!(line.check_corruption(), expected);
    }


    #[test_case("[({(<(())[]>[[{[]{<()<>>", true; "case 1")]
    #[test_case("[(()[<>])]({[<{<<[]>>(", true; "case 2")]
    #[test_case("{([(<{}[<>[]}>{[]{[(<()>", false; "case 3")]
    #[test_case("(((({<>}<{<{<>}{[]{[]{}", true; "case 4")]
    #[test_case("[[<[([]))<([[{}[[()]]]", false; "case 5")]
    #[test_case("[{[{({}]{}}([{[{{{}}([]", false; "case 6")]
    #[test_case("{<[[]]>}<{[{[{[]{()[[[]", true; "case 7")]
    #[test_case("[<(<(<(<{}))><([]([]()", false; "case 8")]
    #[test_case("<{([([[(<>()){}]>(<<{{", false; "case 9")]
    #[test_case("<{([{{}}[<[[[<>{}]]]>[]]", true; "case 10")]
    fn example_2(line: &str, should_be_corrupt: bool) {
        let line = Line::new(line);
        assert!(line.is_ok());

        let line = line.unwrap();

        let check = line.check();
        if should_be_corrupt {
            assert!(check.0.is_none());
            assert!(check.1.is_some());
        } else {
            assert!(check.0.is_some());
            assert!(check.1.is_none());
        }
    }

}