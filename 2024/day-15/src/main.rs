use std::fmt::Debug;

use aoc_utils::Grid;
use aoc_utils::grid::{Dir4, Pos};

fn main() {
    let input = aoc_utils::puzzle_input();
    let (map1, pos1, moves) = parse_input(input);
    let (map2, pos2) = widen_input(&map1, pos1);

    println!("{:#?}", [&map1 as &dyn std::fmt::Debug, &map2 as _]);

    let gps_sum1 = simulate(map1, pos1, moves.iter().copied());

    println!("Sum of all boxes' GPS coordinates (part 1): {gps_sum1}");
}

/// A cell with a width of 1.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell1 {
    Open,
    Wall,
    Box,
}

/// A cell with a width of 2.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell2 {
    Open,
    Wall,
    BoxL,
    BoxR,
}

fn simulate(mut map: Grid<Cell1>, start_pos: Pos, moves: impl IntoIterator<Item = Dir4>) -> usize {
    let mut robot_pos = start_pos;

    for dir in moves {
        let in_front = map
            .get_neighbour(robot_pos, dir)
            .expect("robot should never reach the map border");

        if aoc_utils::verbosity() >= 2 {
            println!("Robot pos: {robot_pos:?}, dir: {dir}. Tile in front: {in_front:#?}.");
        }

        match in_front {
            // If the tile in front of us is empty, we can simply advance.
            Cell1::Open => robot_pos += dir,
            // If there is a wall in front of us, we do nothing.
            Cell1::Wall => {},
            // If there is a box in front of us, we need to check for free space on the other side of the box.
            Cell1::Box => {
                // Scan forward until we find a non-box, starting from the box in front of us.
                let mut pos = robot_pos + dir;
                while map[pos] == Cell1::Box {
                    pos += dir;
                }

                if aoc_utils::verbosity() >= 2 {
                    println!("\tScanned from {:?} -> {:?}. Found tile: {:#?}.", robot_pos + dir, pos, map[pos]);
                }

                // `pos` should now be the position of a non-box. If it's a wall, we can't push the stack of boxes; if
                // it's open space, we can push the entire stack of boxes forward by one.
                if map[pos] == Cell1::Open {
                    // To "push the boxes forward" by one, we can simply move the first box into this open space, then
                    // remove the one right in front of the robot.
                    map[pos] = Cell1::Box;
                    map[robot_pos + dir] = Cell1::Open;
                    robot_pos += dir;
                }
            },
        }
    }

    if aoc_utils::verbosity() >= 1 {
        println!("After moving:\n{map:?}");
    }

    gps_score(&map)
}

/// Computes the sum of all boxes' "GPS coordinates" in the given map.
fn gps_score(map: &Grid<Cell1>) -> usize {
    map.positions()
        .filter_map(|(x, y)| (map[(x, y)] == Cell1::Box).then(|| x + y * 100))
        .sum()
}

fn parse_input(input: &str) -> (Grid<Cell1>, Pos, Vec<Dir4>) {
    let mut lines = input.lines();

    let mut robot_pos = None;
    let map_input = lines.by_ref().take_while(|line| line.len() > 0);
    let map = Grid::from_lines_map(map_input, |c, pos| match c {
        '#' => Cell1::Wall,
        'O' => Cell1::Box,
        '.' => Cell1::Open,
        '@' => {
            robot_pos = Some(pos);
            Cell1::Open
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
        .filter(|c| !c.is_whitespace())
        .map(|c| c.try_into().unwrap())
        .collect();

    (map, robot_pos, moves)
}

/// Takes the input grid from part 1 and widens it for part 2.
fn widen_input(grid: &Grid<Cell1>, robot_pos: Pos) -> (Grid<Cell2>, Pos) {
    // Thanks to truncating division, dividing by two will always give the original index.
    // Wide grid at index 3 is the left half of what was originally cell 3/2 = 1.
    // We can determine if we're currently in the left or right half by checking the remainder.
    let wide_grid = Grid::from_fn(grid.width() * 2, grid.height(), |(x, y)| match grid[(x / 2, y)] {
        Cell1::Open => Cell2::Open,
        Cell1::Wall => Cell2::Wall,
        Cell1::Box => {
            if x % 2 == 0 {
                Cell2::BoxL
            } else {
                Cell2::BoxR
            }
        },
    });

    let (rx, ry) = robot_pos;
    let wide_pos = (rx * 2, ry);
    (wide_grid, wide_pos)
}

impl Debug for Cell1 {
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

impl Debug for Cell2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Self::Open => write!(f, "Open"),
                Self::Wall => write!(f, "Wall"),
                Self::BoxL => write!(f, "BoxL"),
                Self::BoxR => write!(f, "BoxR"),
            }
        } else {
            match self {
                Cell2::Open => write!(f, "."),
                Cell2::Wall => write!(f, "#"),
                Cell2::BoxL => write!(f, "["),
                Cell2::BoxR => write!(f, "]"),
            }
        }
    }
}
