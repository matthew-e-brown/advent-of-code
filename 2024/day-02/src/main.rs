use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let path = std::env::args().skip(1).next().expect("missing input file path");
    let file = File::open(path).expect("failed to open file");
    let lines = BufReader::new(file).lines().map(|line| line.expect("failed to read from file"));

    // Could do this with a simple `Iterator::count`, but I suspect I'm gonna need to do something else while
    let mut num_safe = 0;
    for line in lines {
        let report = line.split_whitespace().map(|s| s.parse::<u32>().expect("invalid puzzle input"));
        if is_safe(report) {
            num_safe += 1;
        }
    }

    println!("Number of safe reports (part 1): {num_safe}");
}


fn is_safe(mut report: impl Iterator<Item = u32>) -> bool {
    // Assumption: an empty report is safe, and a report with only one number is safe.
    let Some(r1) = report.next() else { return true };
    let Some(r2) = report.next() else { return true };

    // Check first two elements to determine if we're increasing or decreasing.
    let inc = r2 > r1;

    // Check the difference on these first two before kicking off the loop.
    let diff = r1.abs_diff(r2);
    if diff < 1 || diff > 3 {
        return false;
    }

    let mut last = r2;
    for x in report {
        if (x > last) != inc {
            return false;
        }

        let diff = x.abs_diff(last);
        if diff < 1 || diff > 3 {
            return false;
        }

        last = x;
    }

    true
}
