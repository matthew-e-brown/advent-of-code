use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let path = std::env::args().skip(1).next().expect("missing input file path");
    let file = File::open(path).expect("failed to open file");

    let lines = BufReader::new(file).lines().map(|line| line.expect("failed to read from file"));

    // Read both lists into vectors
    let mut l = Vec::new();
    let mut r = Vec::new();
    for line in lines {
        let mut chunks = line.split_whitespace();
        let a = chunks.next().and_then(|s| s.parse::<i32>().ok()).expect("invalid puzzle input");
        let b = chunks.next().and_then(|s| s.parse::<i32>().ok()).expect("invalid puzzle input");
        l.push(a);
        r.push(b);
    }

    // Sort them both
    l.sort();
    r.sort();

    // Find the differences
    let mut total = 0;
    for (a, b) in l.into_iter().zip(r.into_iter()) {
        total += (a - b).abs();
    }

    println!("Total distance: {total}")
}
