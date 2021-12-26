use std::collections::HashMap;


fn non_overlapping_pair(string: &str) -> bool {

    let mut pair_indices: HashMap<(char, char), Vec<(usize, usize)>> = HashMap::new();

    // Create a map of all the positions of (a, b) pairs:

    let mut i = 0usize;
    for pair in string.chars().collect::<Vec<_>>().windows(2) {
        let pair = (pair[0], pair[1]);
        let indices = (i, i + 1);

        match pair_indices.get_mut(&pair) {
            Some(v) => v.push(indices),
            None => { pair_indices.insert(pair, vec![indices]); },
        };

        i += 1;
    }

    // For the string `qjhvhtzxzqqjkmpb`, the Map now looks like
    //
    //  {
    //    (q, j) => vec![ (0, 1), (10, 11) ]
    //    (j, h) => vec![ (1, 2) ]
    //    (h, v) => vec![ (2, 3) ]
    //    (v, h) => vec![ (3, 4) ]
    //    (h, t) => vec![ (4, 5) ]
    //    (t, z) => vec![ (5, 6) ]
    //    (z, x) => vec![ (6, 7) ]
    //    (x, z) => vec![ (7, 8) ]
    //    (z, q) => vec![ (8, 9) ]
    //    (q, q) => vec![ (9, 10) ]
    //    (j, k) => vec![ (11, 12) ]
    //    (k, m) => vec![ (12, 13) ]
    //    (m, p) => vec![ (13, 14) ]
    //    (p, b) => vec![ (14, 15) ]
    //  }

    // Filter the map to just the (a, b) letter pairs that appeared more than once
    //
    // {
    //      (q, j) => vec![ (0, 1), (10, 11) ]
    // }
    //
    // And check that any of those (a, b) pairs have a set of two ((1, 2) , (3, 4)) pairs that do not overlap

    pair_indices
        .iter()
        .filter(|(_, v)| v.len() >= 2)
        .any(|(_, vec)| {
            // Compute the (3, 4) and (1, 2) pairs for any given (2, 3) pair
            let mut would_overlap = HashMap::new();

            for pair in vec {
                let pair = pair.clone();
                let mut would = Vec::new();

                would.push((pair.1, pair.1 + 1));
                if pair.0 > 0 { would.push((pair.0 - 1, pair.0)); }

                would_overlap.insert(pair, would);
            }

            // Check and see if there is any vectors that don't overlap

            vec.iter().any(|pair| {
                // can unwrap because we know this hashmap was constructed from this vec directly
                let would_overlap = would_overlap.get(&pair).unwrap();
                vec.iter().any(|test| !would_overlap.contains(test) && pair != test)
            })
        })
}


fn one_letter_between(string: &str) -> bool {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|chunk| chunk[0] == chunk[2])
}


pub fn is_nice(string: &str) -> bool {
    one_letter_between(string) &&
    non_overlapping_pair(string)
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
    #[test_case("xxxddetvrlpzsfpq", false; "case 5")]
    #[test_case("xckozymymezzarpy",  true; "case 6")]
    fn has_pair(string: &str, expected: bool) {
        assert_eq!(non_overlapping_pair(string), expected);
    }


    #[test_case("qjhvhtzxzqqjkmpb",  true; "case 1")]
    #[test_case("xxyxx",             true; "case 2")]
    #[test_case("uurcxstgmygtbstg", false; "case 3")]
    #[test_case("ieodomkazucvgmuy",  true; "case 4")]
    #[test_case("xxxddetvrlpzsfpq",  true; "case 5")]
    #[test_case("xckozymymezzarpy",  true; "case 6")]
    fn has_between(string: &str, expected: bool) {
        assert_eq!(one_letter_between(string), expected);
    }


    #[test_case("qjhvhtzxzqqjkmpb",  true; "case 1")]
    #[test_case("xxyxx",             true; "case 2")]
    #[test_case("uurcxstgmygtbstg", false; "case 3")]
    #[test_case("ieodomkazucvgmuy", false; "case 4")]
    #[test_case("rxexcbwhiywwwwnu",  true; "case 5")]
    fn full_suite(string: &str, expected: bool) {
        assert_eq!(is_nice(string), expected);
    }

}