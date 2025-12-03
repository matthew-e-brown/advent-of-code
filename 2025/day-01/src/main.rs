fn main() {
    let input = aoc_utils::puzzle_input();
    let lines = input.lines();

    let mut dial: i64 = 50;
    let mut password = 0usize;

    for line in lines {
        let rotation = parse_rotation(line);

        dial = (dial + rotation).rem_euclid(100);
        if dial == 0 {
            password += 1;
        }
    }

    println!("Password (part 1): {password}");
}

fn parse_rotation(line: &str) -> i64 {
    assert!(line.len() > 0 && line.is_char_boundary(1), "invalid puzzle input");
    let amt = line[1..].parse::<i64>().expect("invalid puzzle input");
    match line.as_bytes()[0] {
        b'L' => -amt,
        b'R' => amt,
        _ => panic!("invalid puzzle input"),
    }
}
