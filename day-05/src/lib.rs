pub mod part1 {
    use std::collections::HashMap;


    fn is_vowel(letter: char) -> bool {
        letter == 'a' || letter == 'e' ||
        letter == 'i' || letter == 'o' ||
        letter == 'u'
    }


    fn three_vowels(string: &str) -> bool {

        let mut n: u8 = 0;
        for c in string.chars() {
            if is_vowel(c) {
                n += 1;
                if n >= 3 { return true; }
            }
        }

        false
    }


    fn double_letter(string: &str) -> bool {
        let mut letter_counts = HashMap::new();

        // Count occurrences of each char
        for c in string.chars() {
            letter_counts.insert(c, match letter_counts.get(&c) {
                Some(n) => n + 1,
                None => 1u32
            });
        }

        // Now we know which chars appear more than once
        letter_counts
            .iter()
            .filter(|(_, &v)| v >= 2)
            .any(|(&c, _)| {
                let needle: String = vec![c, c].into_iter().collect();
                string.contains(&needle)
            })
    }


    fn no_banned_substrings(string: &str) -> bool {
        !(
            string.contains("ab") ||
            string.contains("cd") ||
            string.contains("pq") ||
            string.contains("xy")
        )
    }


    pub fn is_nice(string: &str) -> bool {
        no_banned_substrings(string) &&
        three_vowels(string) &&
        double_letter(string)
    }


    pub fn run(strings: &Vec<String>) -> usize {
        strings.iter().filter(|s| is_nice(s)).count()
    }
}



#[cfg(test)]
mod tests {

    use super::part1;
    use test_case::test_case;


    #[test_case("ugknbfddgicrmopn",  true; "case 1")]
    #[test_case("aaa",               true; "case 2")]
    #[test_case("jchzalrnumimnmhp", false; "case 3")]
    #[test_case("haegwjzuvuyypxyu", false; "case 4")]
    #[test_case("dvszwmarrgswjxmb", false; "case 5")]
    fn part_1(input: &str, expected: bool) {
        assert_eq!(part1::is_nice(input), expected);
    }

}