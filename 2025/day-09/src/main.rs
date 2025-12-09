fn main() {
    let input = aoc_utils::puzzle_input();
    let positions = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("puzzle input lines should be comma-separated");
            let x = x.parse::<u64>().expect("puzzle input should contain valid numbers");
            let y = y.parse::<u64>().expect("puzzle input should contain valid numbers");
            (x, y)
        })
        .collect::<Vec<_>>();
    assert!(positions.len() >= 2, "puzzle input should have at least two points");

    // Look... I'm kinda tired today. Sometimes, you just gotta go for the good'ole O(n²) double-for loop.
    // (of course, don't forget that we don't need to check i=j, and we only need to check j < i or j > i).

    let mut max_pair = (1, 0);
    let mut max_area = rect_area(positions[1], positions[0]);
    for i in 1..positions.len() {
        for j in 0..i {
            let area = rect_area(positions[i], positions[j]);
            if area > max_area {
                max_area = area;
                max_pair = (i, j);
            }
        }
    }

    let (i, j) = max_pair;
    let (x1, y1) = positions[i];
    let (x2, y2) = positions[j];
    println!("Largest possible rectangle area (part 1): {max_area} (between {x1},{y1} and {x2},{y2})");
}

fn rect_area((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    // Need to add one to the widths and heights of each rectangle.
    // Two x coordinates on the same level makes a rectangle with a width of 1, not zero.
    let w = x1.abs_diff(x2) + 1;
    let h = y1.abs_diff(y2) + 1;
    w * h
}
