fn main() {
    let input = aoc_utils::puzzle_input();

    let mut floor = 0isize;
    let mut first_basement = None;

    for (i, c) in input.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Encountered unknown character '{c}'"),
        }

        if first_basement.is_none() && floor < 0 {
            first_basement = Some(i);
        }
    }

    let first_basement = first_basement.map(|i| i.to_string()).unwrap_or_else(|| "None".to_owned());

    println!("Santa's final floor (part 1): {floor}");
    println!("Position of first basement instruction (part 2): {first_basement}");
}
