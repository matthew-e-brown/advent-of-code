use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

type PageNum = usize;

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut lines = input.lines();
    let mut sorted_mid_sums = 0;
    let mut unsorted_mid_sums = 0;

    // Grab just the first section, until we hit a blank line. These rules combine to form a partial ordering over the
    // set of page numbers. There are not enough rules to warrant with a second layer of `HashSet`; we'll just use a
    // linear search over a vector.
    let mut ordering = HashMap::<PageNum, Vec<PageNum>, ahash::RandomState>::default();
    let rules = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| line.parse::<Rule>().unwrap());
    for Rule(a, b) in rules {
        ordering.entry(a).or_default().push(b);
    }

    // Now that we've built the ordering table, we can just read in every update and see if they match the sorted order.
    let mut unsorted = Vec::new();
    let mut sorted = Vec::new();
    for line in lines {
        parse_update_to_vec(line, &mut unsorted).unwrap();
        sorted.clone_from(&unsorted);

        // Rust's `sort_by` functions all expect the user provided comparison to implement a **total ordering**.
        // However, our hashmap only provides a partial ordering: it only tells us if `a < b`. And, actually, it doesn't
        // even fully define a partial ordering, since a valid partial ordering requires that `a < b => b > a` (called
        // **duality** in the `PartialOrd` docs). In order to properly define a total ordering, we need to check both
        // directions, since we don't know which way std's sorting algorithm will end up doing comparisons; if we get
        // unlucky and whichever sort impl that std ends up using only happens to call this comparator function in the
        // "wrong direction," it won't sort properly.
        //
        // We'll use a stable sort just in case there are any pages that don't participate in the ordering. Of course,
        // the *most* correct thing to do would be a topological sort, but... this puzzle is carefully designed so that
        // that doesn't matter, and the "middle" element always turns out to be correct.
        sorted.sort_by(|&a, &b| {
            // If A must go before B, it is `<`. Otherwise, if B must go before A, then `a > b`. Other-otherwise, we
            // don't care... extend this to a total ordering by saying all others are totally equivalent.
            if ordering.get(&a).is_some_and(|v| v.contains(&b)) {
                Ordering::Less
            } else if ordering.get(&b).is_some_and(|v| v.contains(&a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        if &unsorted == &sorted {
            sorted_mid_sums += sorted[sorted.len() / 2];
        } else {
            unsorted_mid_sums += sorted[sorted.len() / 2];
        }
    }

    println!("Sum of already-sorted middle elements (part 1): {}", sorted_mid_sums);
    println!("Sum of freshly-sorted middle elements (part 2): {}", unsorted_mid_sums);
}

#[derive(Debug, Clone, Copy)]
struct Rule(PageNum, PageNum);

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('|').ok_or("page ordering rule is missing '|' symbol")?;
        let a = a.trim().parse().map_err(|_| "page ordering rule contains invalid integer")?;
        let b = b.trim().parse().map_err(|_| "page ordering rule contains invalid integer")?;
        Ok(Rule(a, b))
    }
}

fn parse_update_to_vec(line: &str, vec: &mut Vec<PageNum>) -> Result<(), &'static str> {
    vec.clear();
    for s in line.split(',') {
        let p = s.trim().parse().map_err(|_| "page update contains invalid integer")?;
        vec.push(p);
    }
    Ok(())
}
