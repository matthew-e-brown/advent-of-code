use std::fs::File;
use std::io::Read;

use regex::Regex;

fn main() {
    let text = {
        let path = std::env::args().skip(1).next().expect("missing input file path");
        let mut file = File::open(path).expect("failed to open file");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("failed to read from file");
        buf
    };

    let mut sum = 0;

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for captures in mul_regex.captures_iter(&text) {
        let x = captures.get(1).unwrap().as_str().parse::<u32>().expect("capture group always contains digits");
        let y = captures.get(2).unwrap().as_str().parse::<u32>().expect("capture group always contains digits");
        sum += x * y;
    }

    println!("Sum of all mul(X,Y) expressions: {sum}");
}
