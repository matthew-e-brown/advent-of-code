use std::collections::HashSet;

use aoc_utils::grid::Dir4;

fn main() {
    let input = aoc_utils::puzzle_input();
    let dirs = input
        .chars()
        .map::<Dir4, _>(|c| c.try_into().expect("puzzle input should be valid"));

    let mut p1_set = HashSet::<(isize, isize)>::new();
    let mut p2_set = HashSet::<(isize, isize)>::new();
    p1_set.insert((0, 0));
    p2_set.insert((0, 0));

    let mut santa_pos1 = (0, 0);
    let mut santa_pos2 = (0, 0);
    let mut robot_pos2 = (0, 0);
    let mut p2_santa = true;

    for dir in dirs {
        add_dir(&mut santa_pos1, dir);
        p1_set.insert(santa_pos1);

        if p2_santa {
            add_dir(&mut santa_pos2, dir);
            p2_set.insert(santa_pos2);
        } else {
            add_dir(&mut robot_pos2, dir);
            p2_set.insert(robot_pos2);
        }

        p2_santa = !p2_santa;
    }

    println!("Number of houses visited by just Santa (part 1): {}", p1_set.len());
    println!("Number of houses visited by Santa and Robo-Santa (part 2): {}", p2_set.len());
}

// [TODO] Change `aoc_utils::grid::GridIndex` to use signed positions (preferably both signed/unsigned), then simply use
// those implementations.
fn add_dir(pos: &mut (isize, isize), dir: Dir4) {
    let (mut x, mut y) = *pos;
    match dir {
        Dir4::Up => y -= 1,
        Dir4::Down => y += 1,
        Dir4::Left => x -= 1,
        Dir4::Right => x += 1,
    }
    *pos = (x, y);
}
