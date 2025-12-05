use std::cmp;
use std::collections::BinaryHeap;
use std::fmt::{Debug, Display};

fn main() {
    let input = aoc_utils::puzzle_input();
    let mut lines = input.lines();

    let mut width = 1usize; // For pretty-printing: format numbers based on the largest width

    // First, build the database of fresh fruit. We have a bunch of ranges which potentially overlap, and it's probably
    // a good idea to merge them into a bunch of non-overlapping sets.
    let ranges = lines.by_ref().take_while(|line| line.len() > 0).map(|line| {
        let (a, b) = line.split_once('-').expect("puzzle input should have dash-separated ranges");
        width = width.max(a.len()).max(b.len());
        let a = a.parse::<u64>().expect("puzzle input should contain valid u64s");
        let b = b.parse::<u64>().expect("puzzle input should contain valid u64s");
        Range::new(a, b)
    });

    let ranges = build_ranges(ranges);
    let fruits = lines.map(|line| line.parse::<u64>().expect("puzzle input should contain valid u64s"));

    // Now that the ranges are all merged, we should be able to do a simple binary search for range starts.

    let mut fresh_count = 0usize;
    for fruit in fruits {
        if let Some(i) = search_ranges(&ranges, fruit) {
            fresh_count += 1;
            if aoc_utils::verbosity() > 0 {
                println!("Fruit {fruit:width$}: fits into range #{i:3} ({:width$})", &ranges[i]);
            }
        } else {
            if aoc_utils::verbosity() > 1 {
                println!("Fruit {fruit:width$}: not fresh");
            }
        }
    }

    // Aha! Thinking ahead pays off. Part 2 is dead simple now that we've already sorted and merged our ranges! :D
    let mut total_fresh = 0usize;
    for &range in &ranges {
        // Range size is +1 because they're inclusive.
        total_fresh += (range.end - range.start + 1) as usize;
    }

    println!("Number of input fresh fruits from input (part 1): {fresh_count}");
    println!("Total number of fresh fruit across all ranges (part 2): {total_fresh}");
}

/// An inclusive range.
///
/// Rust's [`RangeInclusive`][std::ops::RangeInclusive] struct is a little bit awkward to work with, on account of being
/// generic over any type. It exposes `.start()` and `.end()` methods, but has no way to mutate those values. It's meant
/// mostly for looping through.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(w) = f.width() {
            write!(f, "{:>w$}-{:<w$}", self.start, self.end)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(w) = f.width() {
            write!(f, "{:w$}..={:w$}", self.start, self.end)
        } else {
            write!(f, "{}..={}", self.start, self.end)
        }
    }
}

/// Merges a series of ranges into a smaller set of non-overlapping ranges.
fn build_ranges(ranges: impl Iterator<Item = Range>) -> Vec<Range> {
    // https://stackoverflow.com/a/5276786/10549827
    // - Sort the ranges by starting time.
    // - Iterate over them and merge anything that overlaps.

    /// Defines an ordering over a [`Range`] based on the range's start.
    /// Also sorts ranges in ascending order instead of descending order inside a [`BinaryHeap`].
    #[derive(PartialEq, Eq)]
    struct SortHelper(Range);

    impl Ord for SortHelper {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.0.start.cmp(&other.0.start).then(self.0.end.cmp(&other.0.end)).reverse()
        }
    }

    impl PartialOrd for SortHelper {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    // Collect into BinaryHeap to sort, then merge in a vector:
    let mut ranges = ranges.map(SortHelper).collect::<BinaryHeap<SortHelper>>();
    let mut merged = Vec::<Range>::new();
    while let Some(SortHelper(range)) = ranges.pop() {
        match merged.last_mut() {
            Some(current) if range.start <= current.end => current.end = current.end.max(range.end),
            Some(_) | None => merged.push(range),
        }
    }

    merged
}

/// Searches a (**sorted**) set of non-overlapping ranges to see if the given item falls within one of them.
fn search_ranges(ranges: &[Range], x: u64) -> Option<usize> {
    // The ranges are sorted by their starting point first.
    match ranges.binary_search_by(|range| range.start.cmp(&x)) {
        // If we find the desired value directly, we're obviously done.
        Ok(i) => Some(i),
        // Otherwise, `binary_search_by` will return an "index where a matching element could be inserted while
        // maintaining sorted order."
        // - If `x` is smaller than the smallest starting point, that index will be 0; `x` is not in any range.
        // - Otherwise, `i` would place `x` right after the range with the closest starting point that is less than `x`;
        //   if `x` fits within range `i - 1`, then we have a hit. If it doesn't, no hit.
        // - There is no special edge-case for when `i` is `range.len()`; the same `i - 1` check works there, too.
        Err(0) => None,
        Err(i) if x <= ranges[i - 1].end => Some(i - 1),
        Err(_) => None,
    }
}
