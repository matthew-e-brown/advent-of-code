use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;

use aoc_utils::Grid;
use aoc_utils::grid::{Dir4, Pos};

fn main() {
    let input = aoc_utils::puzzle_input();
    let (mut map1, pos1, moves) = parse_input(input);
    let (mut map2, pos2) = widen_input(&map1, pos1);

    if aoc_utils::verbosity() >= 1 {
        println!("Maps before simulation:\n{:#?}\n", [&map1 as &dyn std::fmt::Debug, &map2 as _]);
    }

    let gps_sum1 = simulate(&mut map1, pos1, moves.iter().copied());
    let gps_sum2 = simulate(&mut map2, pos2, moves.iter().copied());

    if aoc_utils::verbosity() >= 1 {
        println!("Maps after simulation:\n{:#?}\n", [&map1 as &dyn std::fmt::Debug, &map2 as _]);
    }

    println!("Sum of all boxes' GPS coordinates (part 1): {gps_sum1}");
    println!("Sum of all boxes' GPS coordinates (part 2): {gps_sum2}");
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

/// Abstraction over both types of cell.
trait Cell: Sized + Copy + Debug {
    /// Returns a new instance of an "open" cell.
    fn open() -> Self;

    /// Checks if this cell is open space.
    fn is_open(&self) -> bool;

    /// Checks if this cell is a solid wall.
    fn is_wall(&self) -> bool;

    /// Checks if this cell is a box.
    fn is_box(&self) -> bool;

    /// Checks if this cell should be included in calculating a [GPS score][gps_score].
    fn include_in_gps(&self) -> bool;

    /// If the object in this cell takes up multiple cells, this returns an iterator over all of its pieces.
    fn all_pieces(&self, pos: Pos) -> impl IntoIterator<Item = Pos> + Clone;
}

#[rustfmt::skip]
impl Cell for Cell1 {
    fn open() -> Self { Cell1::Open }

    fn is_open(&self) -> bool { matches!(self, Self::Open) }
    fn is_wall(&self) -> bool { matches!(self, Self::Wall) }
    fn is_box(&self) -> bool { matches!(self, Self::Box) }
    fn include_in_gps(&self) -> bool { matches!(self, Self::Box) }

    fn all_pieces(&self, pos: Pos) -> impl IntoIterator<Item = Pos> + Clone {
        [pos]
    }
}

#[rustfmt::skip]
impl Cell for Cell2 {
    fn open() -> Self { Cell2::Open }

    fn is_open(&self) -> bool { matches!(self, Self::Open) }
    fn is_wall(&self) -> bool { matches!(self, Self::Wall) }
    fn is_box(&self) -> bool { matches!(self, Self::BoxL | Self::BoxR) }
    fn include_in_gps(&self) -> bool { matches!(self, Self::BoxL) }

    fn all_pieces(&self, pos: Pos) -> impl IntoIterator<Item = Pos> + Clone {
        let cells = match self {
            Cell2::Open => [Some(pos), None],
            Cell2::Wall => [Some(pos), None],
            Cell2::BoxL => [Some(pos), Some(pos + Dir4::Right)],
            Cell2::BoxR => [Some(pos), Some(pos + Dir4::Left)],
        };
        cells.into_iter().filter_map(|x| x)
    }
}

fn simulate<C: Cell>(map: &mut Grid<C>, start_pos: Pos, moves: impl IntoIterator<Item = Dir4>) -> usize {
    let mut robot_pos = start_pos;

    for dir in moves {
        let in_front_pos = robot_pos + dir;
        let in_front_cell = &map[in_front_pos];

        if aoc_utils::verbosity() >= 2 {
            println!("Robot pos: {robot_pos:?}, dir: {dir}. Cell in front: {in_front_cell:#?}.");
        }

        if in_front_cell.is_open() {
            // If the tile in front of us is empty, we can simply advance.
            robot_pos += dir;
        } else if in_front_cell.is_box() {
            // If there's a box in front of us, we need to check for free space on the other side of the box so we can
            // push it. Specifically, we want to push this box and all connected boxes in the current direction, so long
            // as none of them are blocked by a wall.
            //
            // To do this for boxes of arbitrary width, we basically want to do a BFS/DFS traversal (since each box may
            // push multiple other boxes). Then, once we have completed the traversal and found that none of the boxes
            // push up against any walls, we can push all of the boxes forwards.
            //
            // To actually push the boxes forwards, just like when copying in an array, we need to go in reverse:
            // furthest away boxes need to move into the empty space, then the one closer needs to advance to take its
            // place, etc. Otherwise, we could overwrite data before we push. To ensure we can re-process all the boxes
            // in order, we'll use a breadth-first search, ensuring boxes are pushed onto the discovery queue in order
            // away from the robot's position. Then, once we finish our search, we can go back over that queue
            // backwards.
            let mut can_push = true;

            let mut bfs_order = Vec::new(); // Order we visited all boxes in our search
            let mut bfs_frontier = VecDeque::new(); // Boxes we still need to search
            let mut bfs_discovered = HashSet::new(); // Boxes we have discovered

            let in_fronts = in_front_cell.all_pieces(in_front_pos);
            bfs_frontier.extend(in_fronts.clone());
            bfs_discovered.extend(in_fronts);

            while let Some(pos) = bfs_frontier.pop_front() {
                // Okay, can we push a box into this tile?
                let cell = &map[pos];

                if cell.is_wall() {
                    // If we hit a wall at *any* point during the traversal, we can stop immediately.
                    can_push = false;
                    break;
                } else if cell.is_box() {
                    // Otherwise, this box will need to get pushed upwards.
                    bfs_order.push(pos);
                    let next_pos = pos + dir;
                    let next_cell = map[next_pos];
                    for next in next_cell.all_pieces(next_pos) {
                        if bfs_discovered.insert(next) {
                            bfs_frontier.push_back(next);
                        }
                    }
                }
            }

            if can_push {
                // Now we just need to actually push everything. Make sure we go backwards!
                for pos in bfs_order.into_iter().rev() {
                    // Note that this is not quite like shifting elements of a vector; i.e., we can't just copy the tile
                    // from "below" us (assuming an upwards push). Consider this case:
                    // `````````       `````````
                    // #       #       # [][]  #
                    // # [][]  #  ==>  #  []   #
                    // # #[][] #       # #^ [] #
                    // #  ^    #       #       #
                    // `````````       `````````
                    // Here, we don't actually want to strictly copy what was below. We need the upper two boxes to
                    // shift away from the wall and other box they are resting above. To do that, we still need to
                    // iterate downwards (in reverse), but we do it by moving them up and out of the way, then replacing
                    // them with an empty spot for the next cell to move up into.
                    assert!(map[pos].is_box(), "only boxes should have been queued for shifting");
                    map[pos + dir] = map[pos];
                    map[pos] = C::open();
                }

                // Make sure we actually move the robot, too.
                robot_pos += dir;
            }
        }
    }

    gps_score(&map)
}

/// Computes the sum of all boxes' "GPS coordinates" in the given map.
fn gps_score<C: Cell>(map: &Grid<C>) -> usize {
    map.positions()
        .filter(|&pos| map[pos].include_in_gps())
        .map(|(x, y)| x + y * 100)
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
