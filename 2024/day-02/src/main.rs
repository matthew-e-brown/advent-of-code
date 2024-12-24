fn main() {
    let input = aoc_utils::puzzle_input();
    let lines = input.lines();

    let mut num_safe = 0;
    let mut num_almost_safe = 0;

    // Want to iterate each report multiple times, but allocating a new buffer each time would be a tad wasteful. So
    // make some scratch-space that we'll clear and refill each line.
    let mut report_buff = Vec::new();
    for line in lines {
        let nums = line.split_whitespace().map(|s| s.parse::<u32>().expect("invalid puzzle input"));
        report_buff.extend(nums);

        if is_safe(report_buff.iter().copied()) {
            num_safe += 1;
        } else {
            // We know that each line will only contain 5-8 numbers... so just brute force it, checking each index.
            // (note: `awk '{ print NF }' input.txt | sort -nr | uniq -c`)
            let mut can_be_safe = false;
            for skip in 0..report_buff.len() {
                // Create an iterator that skips over the i'th element
                let iter = report_buff
                    .iter()
                    .enumerate()
                    .filter_map(|(i, n)| (i != skip).then_some(n))
                    .copied();
                if is_safe(iter) {
                    can_be_safe = true;
                    break;
                }
            }

            if can_be_safe {
                num_almost_safe += 1;
            }
        }

        report_buff.clear();
    }

    println!("Number of safe reports (part 1): {num_safe}");
    println!("Number of mostly safe reports (part 2): {}", num_safe + num_almost_safe);
}


fn is_safe(mut report: impl Iterator<Item = u32>) -> bool {
    // Assumption: an empty report is safe, and a report with only one number is safe.
    let Some(r1) = report.next() else { return true };
    let Some(r2) = report.next() else { return true };

    // Check first two elements to determine if we're increasing or decreasing.
    let increasing = r2 > r1;

    // Check the difference on these first two before kicking off the loop.
    let d = r1.abs_diff(r2);
    if d < 1 || d > 3 {
        return false;
    }

    let mut prev = r2;
    for x in report {
        let i = x > prev;
        let d = x.abs_diff(prev);
        if i != increasing || d < 1 || d > 3 {
            return false;
        }

        prev = x;
    }

    true
}
