use aoc2025_06::Worksheet;

fn main() {
    let input = aoc_utils::puzzle_input();
    let sheet = Worksheet::from_input(input).expect("puzzle input should be valid");

    println!("{sheet:#?}");

    /*
    let input = aoc_utils::puzzle_input();

    let line_count = input.lines().count();
    assert!(line_count >= 3, "puzzle input should have at least 3 lines (2 lines of terms + 1 line of ops)");

    let t = line_count - 1; // Number of terms per problem (AKA, number of lines).
    let p; // Number of problems total (AKA, number of terms per line).

    // We'll collect all the terms for all problems into the same buffer; since all lines are the same length `p`,
    // subsequent terms are indexed by looking forwards `p` spots.
    let mut nums = Vec::<u64>::new();

    // Using `by_ref` like this lets us iterate through the first `t` lines and have us left with an iterator already
    // waiting at the last line to run through the operators.
    let mut lines = input.lines();
    let mut terms = lines.by_ref().take(t).map(|line| {
        // NB: *Not* a flat_map; we want to pull out `t` separate iterators and extend the vec with each of them.
        line.split_whitespace()
            .map(|term| term.parse::<u64>().expect("puzzle input should have valid u64s"))
    });

    // Grab the first one so we can count the length and reserve vector capacity.
    nums.extend(terms.next().unwrap()); // unwrap: we know there are at least `t >= 2` lines of terms
    p = nums.len();
    assert!(p > 0, "puzzle input should have at least 1 problem");

    if aoc_utils::verbosity() > 0 {
        println!("Determined {p} problems with {t} terms each.");
    }

    // Now grab the remaining `t - 1` lines of `p` terms
    nums.reserve(p * (t - 1));
    for (i, iter) in terms.enumerate() {
        nums.extend(iter);
        // Length should be `p` higher than it was last time:
        assert_eq!(nums.len(), p + p * (i + 1), "all lines of puzzle input should have the same number of terms");
    }

    // Now we can run through the operators in the last line.
    let ops = lines.next().unwrap(); // unwrap: at least one line left, `terms` was a `take(line_count - 1)`

    let mut grand_total = 0u64;
    let mut op_count = 0usize; // Probably could to assert this last line is the same length, too
    for (i, op) in ops.split_whitespace().enumerate() {
        // This is problem #i. The terms for it are at `nums[i], nums[i + p], nums[i + 2p], ..., nums[i + (t-1)p]`.
        // We just need to fetch those numbers and reduce them using the corresponding operation.
        let reducer = match op {
            "+" => std::ops::Add::add,
            "*" => std::ops::Mul::mul,
            x => panic!("puzzle input should only contain '+' and '*' operators (encountered '{x}')"),
        };

        let answer = (0..=(t - 1)).map(|j| nums[i + j * p]).reduce(reducer).unwrap();
        grand_total += answer;
        op_count += 1;
    }

    assert_eq!(op_count, p, "puzzle input should have same number of terms and operators");

    println!("Grand total of all cephalopod problem answers (part 1): {grand_total}");
    */
}
