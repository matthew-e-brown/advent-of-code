use fancy_regex::Regex;

pub fn run_1(strings: &Vec<String>) -> usize {
    let re1 = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let re2 = Regex::new(r"(.)\1").unwrap();
    let re3 = Regex::new(r"(:?ab|cd|pq|xy)").unwrap();

    strings
        .iter()
        .filter(|text|
            re1.is_match(text).unwrap() &&
            re2.is_match(text).unwrap() &&
            !re3.is_match(text).unwrap()
        )
        .count()
}

pub fn run_2(strings: &Vec<String>) -> usize {
    let re1 = Regex::new(r"(..).*\1").unwrap();
    let re2 = Regex::new(r"(.).\1").unwrap();

    strings
        .iter()
        .filter(|text|
                re1.is_match(text).unwrap() &&
                re2.is_match(text).unwrap()
        )
        .count()
}


#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;


    #[test_case("ugknbfddgicrmopn",  true; "case 1")]
    #[test_case("aaa",               true; "case 2")]
    #[test_case("jchzalrnumimnmhp", false; "case 3")]
    #[test_case("haegwjzuvuyypxyu", false; "case 4")]
    #[test_case("dvszwmarrgswjxmb", false; "case 5")]
    fn part_1(input: &str, expected: bool) {
        let as_vec = vec![input.to_owned()];
        let count = run_1(&as_vec);

        assert!(count == if expected { 1 } else { 0 });
    }


    #[test_case("qjhvhtzxzqqjkmpb",  true; "case 1")]
    #[test_case("xxyxx",             true; "case 2")]
    #[test_case("uurcxstgmygtbstg", false; "case 3")]
    #[test_case("ieodomkazucvgmuy", false; "case 4")]
    fn part_2(input: &str, expected: bool) {
        let as_vec = vec![input.to_owned()];
        let count = run_2(&as_vec);

        assert!(count == if expected { 1 } else { 0 });
    }

}