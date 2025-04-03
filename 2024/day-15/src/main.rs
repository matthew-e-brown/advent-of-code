use std::fmt::Debug;

use aoc_utils::Grid;
use aoc_utils::grid::{Dir4, Pos};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Open,
    Wall,
    Box,
}

fn main() {
    let (mut map, mut robot_pos, moves) = parse_input();

    for dir in moves {
        let in_front = map
            .get_neighbour(robot_pos, dir)
            .expect("robot should never reach the map border");

        if aoc_utils::verbosity() >= 2 {
            println!("Robot pos: {robot_pos:?}, dir: {dir}. Tile in front: {in_front:#?}.");
        }

        match in_front {
            // If the tile in front of us is empty, we can simply advance.
            Cell::Open => robot_pos += dir,
            // If there is a wall in front of us, we do nothing.
            Cell::Wall => {},
            // If there is a box in front of us, we need to check for free space on the other side of the box.
            Cell::Box => {
                // Scan forward until we find a non-box, starting from the box in front of us.
                let mut pos = robot_pos + dir;
                while map[pos] == Cell::Box {
                    pos += dir;
                }

                if aoc_utils::verbosity() >= 2 {
                    println!("\tScanned from {:?} -> {:?}. Found tile: {:#?}.", robot_pos + dir, pos, map[pos]);
                }

                // `pos` should now be the position of a non-box. If it's a wall, we can't push the stack of boxes; if
                // it's open space, we can push the entire stack of boxes forward by one.
                if map[pos] == Cell::Open {
                    // To "push the boxes forward" by one, we can simply move the first box into this open space, then
                    // remove the one right in front of the robot.
                    map[pos] = Cell::Box;
                    map[robot_pos + dir] = Cell::Open;
                    robot_pos += dir;
                }
            },
        }
    }

    if aoc_utils::verbosity() >= 1 {
        println!("After moving:\n{map:?}");
    }

    let gps_sum = gps_score(&map);
    println!("Sum of all boxes' GPS coordinates (part 1): {gps_sum}");
}

/// Computes the sum of all boxes' "GPS coordinates" in the given map.
fn gps_score(map: &Grid<Cell>) -> usize {
    map.positions()
        .filter_map(|(x, y)| (map[(x, y)] == Cell::Box).then(|| x + y * 100))
        .sum()
}

fn parse_input() -> (Grid<Cell>, Pos, impl Iterator<Item = Dir4>) {
    let mut lines = aoc_utils::puzzle_input().lines();

    let mut robot_pos = None;
    let map_input = lines.by_ref().take_while(|line| line.len() > 0);
    let map = Grid::from_lines_map(map_input, |c, pos| match c {
        '#' => Cell::Wall,
        'O' => Cell::Box,
        '.' => Cell::Open,
        '@' => {
            robot_pos = Some(pos);
            Cell::Open
        },
        _ => panic!("Unknown char {c} found in map at position {pos:?}"),
    })
    .unwrap();
    let robot_pos = robot_pos.expect("robot position ('@') should have been found somewhere in map");

    let (rx, ry) = robot_pos;
    let (mw, mh) = map.size();
    assert!(
        (rx >= 1 && rx <= mw - 1) && (ry >= 1 && ry <= mh - 1),
        "robot should be at least one tile away from map borders"
    );

    let moves = lines
        .flat_map(|line| line.chars())
        .filter(|&c| c != '\n' && c != '\r')
        .map(|c| c.try_into().unwrap());

    (map, robot_pos, moves)
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::Open => write!(f, "Open"),
                Self::Wall => write!(f, "Wall"),
                Self::Box => write!(f, "Box"),
            }
        } else {
            match self {
                Self::Open => write!(f, "."),
                Self::Wall => write!(f, "#"),
                Self::Box => write!(f, "O"),
            }
        }
    }
}
