use std::fs::File;
use std::io::Read;

use regex::Regex;

fn main() {
    let input = {
        let path = std::env::args().skip(1).next().expect("missing input file path");
        let mut file = File::open(path).expect("failed to open file");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("failed to read from file");
        buf
    };

    println!("Sum of all mul(X,Y) expressions (part 1): {}", part1(&input));
    println!("Sum of just the enabled mul expressions (part 2): {}", part2(&input));
}


fn part1(input: &str) -> u32 {
    let mut sum = 0;

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for captures in mul_regex.captures_iter(input) {
        // Can unwrap the parsing because capture group is guaranteed to contain digits.
        let x = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let y = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        sum += x * y;
    }

    sum
}


fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut mul_enabled = true;

    let func_regex = Regex::new(r"(?<mul>mul\((\d+),(\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    for captures in func_regex.captures_iter(input) {
        if mul_enabled && captures.name("mul").is_some() {
            // If `mul` group exists, digits are guaranteed to.
            let x = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let y = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
            sum += x * y;
        } else if captures.name("do").is_some() {
            mul_enabled = true;
        } else if captures.name("dont").is_some() {
            mul_enabled = false;
        }
    }

    sum
}
