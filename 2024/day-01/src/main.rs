pub fn main() {
    let input = aoc_utils::puzzle_input();
    let lines = input.lines();

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
    for (a, b) in l.iter().zip(r.iter()) {
        total += (a - b).abs();
    }

    println!("Total distance (part 1): {total}");

    // Both lists are already sorted, so finding the number of occurrences of each number is easy: just do a simple
    // double-pointer scan.
    let mut j = 0;
    let mut sim_score = 0;
    for x in l {
        // Move forward until we hit a common region, skipping past anything that might not appear in `r`
        while j < r.len() && x > r[j] {
            j += 1;
        }

        // Count the number of times we see `x` in this region of `r` before hitting another element
        let mut n = 0;
        while j < r.len() && x == r[j] {
            n += 1;
            j += 1;
        }

        sim_score += x * n;

        // Kill the outer loop early if we hit the end of `r`.
        if !(j < r.len()) {
            break;
        }
    }

    println!("Similarity score (part 2): {sim_score}");
}
