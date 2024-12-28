use std::fmt::{Debug, Display};

use aoc_utils::Grid;

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut start_pos = None;
    let mut obstruction_map = Grid::from_lines_map(input.lines(), |c, pos| match c {
        '#' => Cell::Wall,
        '.' => Cell::Open,
        '^' => {
            start_pos = Some(pos);
            Cell::Open
        },
        _ => panic!("invalid input: unexpected char '{c}'"),
    })
    .unwrap();

    // Part 1:
    let mut pos = start_pos.expect("invalid input: missing start character ('^')");
    let mut dir = Direction::Up;
    let mut n: usize = 0;

    'outer: loop {
        // If the cell we're currently on is open and not yet visited, visit and count it.
        if obstruction_map[pos] == Cell::Open {
            obstruction_map[pos] = Cell::Visited;
            n += 1;
        }

        // Just in case we hit a corner or something, use a loop to determine the next position
        'inner: loop {
            match in_front(pos, dir, &obstruction_map) {
                // If the next step is out of bounds, we can stop the whole loop; this tile has already been counted.
                None => break 'outer,
                // If the next step is a wall, rotate and try again.
                Some(next_pos) if obstruction_map[next_pos] == Cell::Wall => {
                    dir = dir.turn_right();
                },
                // Otherwise, keep looking in the same direction and move forwards.
                Some(next_pos) => {
                    pos = next_pos;
                    break 'inner;
                },
            }
        }
    }

    println!("Number of unique tiles encountered (part 1): {n}");
}

fn in_front(pos: (usize, usize), dir: Direction, grid: &Grid<Cell>) -> Option<(usize, usize)> {
    let (x, y) = pos;
    match dir {
        Direction::Up if y > 0 => Some((x, y - 1)),
        Direction::Left if x > 0 => Some((x - 1, y)),
        Direction::Down if y < grid.height() => Some((x, y + 1)),
        Direction::Right if x < grid.width() => Some((x + 1, y)),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Open,
    Wall,
    Visited,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Cell::Open => ".",
            Cell::Wall => "#",
            Cell::Visited => "X",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(&mut self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    #[allow(unused)]
    pub fn turn_left(&mut self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Direction::Up => "↑",
            Direction::Down => "↓",
            Direction::Left => "←",
            Direction::Right => "→",
        })
    }
}
