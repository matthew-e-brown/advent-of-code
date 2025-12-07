use aoc2025_06::{Operator, Worksheet};

fn main() {
    let input = aoc_utils::puzzle_input();
    let sheet = Worksheet::from_input(input).expect("puzzle input should be valid");

    let mut grand_total1 = 0;
    let mut grand_total2 = 0;
    for p in 0..sheet.len() {
        let reducer = match sheet.operator(p) {
            Operator::Add => std::ops::Add::add,
            Operator::Mul => std::ops::Mul::mul,
        };

        grand_total1 += sheet.terms_across(p).reduce(reducer).unwrap();
        grand_total2 += sheet.terms_down(p).reduce(reducer).unwrap();
    }

    println!("Grand total of cephalopod's problem answers (part 1): {grand_total1}");
    println!("Grand total of cephalopod's problem answers (part 2): {grand_total2}");
}
