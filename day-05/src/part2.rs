use std::collections::{HashMap, HashSet};


fn non_overlapping_pair(string: &str) -> bool {
    let chars: Vec<_> = string.chars().collect();
    let mut pair_indices: HashMap<(char, char), Vec<(usize, usize)>> = HashMap::new();

    // Create a map of all the positions of (a, b) pairs:

    let mut i = 0usize;
    for pair in chars.windows(2) {
        let pair = (pair[0], pair[1]);
        let indices = (i, i + 1);

        match pair_indices.get_mut(&pair) {
            Some(v) => v.push(indices),
            None => { pair_indices.insert(pair, vec![indices]); },
        };

        i += 1;
    }

    // For the string `abcdabefcd`, the Map now looks like
    //
    // {
    //      (a, b) => vec![ (0, 1), (4, 5) ],
    //      (b, c) => vec![ (1, 2) ],
    //      (c, d) => vec![ (2, 3), (8, 9) ],
    //      (d, a) => vec![ (3, 4) ],
    //      (b, e) => vec![ (5, 6) ],
    //      (e, f) => vec![ (6, 7) ],
    //      (f, c) => vec![ (7, 8) ],
    // }
    //
    // For the string `xxyxx`, the Map now looks like
    //
    // {
    //      (x, x) => vec![ (0, 1), (3, 4) ],
    //      (x, y) => vec![ (1, 2) ],
    //      (y, x) => vec![ (2, 3) ],
    // }

    // Run through the map, checking if any of the pairs that appear more than once don't overlap
    pair_indices
        .iter()
        .filter(|(_, v)| v.len() >= 2)
        .any(|(_, v)| {
            let mut would_overlap = HashSet::new();

            // Compute what (2, 3) pair would be an overlap for any given (1, 2) pair
            for indices in v {
                would_overlap.insert(( indices.1, indices.1 + 1 ));
            }

            // Check if there's any (1, 2) pair without its (2, 3) pair
            v.iter().any(|i| !would_overlap.contains(i))
        })
}


fn one_letter_between(string: &str) -> bool {
    false
}


pub fn is_nice(string: &str) -> bool {
    non_overlapping_pair(string) &&
    one_letter_between(string)
}


pub fn run(strings: &Vec<String>) -> usize {
    strings.iter().filter(|s| is_nice(s)).count()
}


#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;


    #[test_case("qjhvhtzxzqqjkmpb",  true; "case 1")]
    #[test_case("xxyxx",             true; "case 2")]
    #[test_case("uurcxstgmygtbstg",  true; "case 3")]
    #[test_case("ieodomkazucvgmuy", false; "case 4")]
    fn has_pair(string: &str, expected: bool) {
        assert_eq!(non_overlapping_pair(string), expected);
    }

}