use std::collections::BTreeSet;
use std::fmt::{Debug, Display};
use std::ops::ControlFlow;
use std::sync::mpsc;

use aoc_utils::Grid;
use scoped_threadpool::Pool;

type Position = (usize, usize);

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut start_pos = None;
    let map = Grid::from_lines_map(input.lines(), |c, pos| match c {
        '#' => Cell::WALL,
        '.' => Cell::OPEN,
        '^' => {
            start_pos = Some(pos);
            Cell::OPEN
        },
        _ => panic!("invalid input: unexpected char '{c}'"),
    })
    .unwrap();

    let start_pos = start_pos.expect("invalid input: missing start character ('^')");

    // Start by taking note of all the fresh tiles we encounter as we loop.
    let mut all_tiles = Vec::new();
    let mut num_unique = 0;
    run_simulation::<(), _>(start_pos, map.clone(), |cell, pos, dir| {
        // By keeping track of the travelling direction for each tile, we'll be able to place extra wall tiles "in
        // front" of the agent's path for part 2.
        all_tiles.push((pos, dir));
        if cell == Cell::OPEN {
            num_unique += 1;
        }

        ControlFlow::Continue(())
    });

    let n_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8);
    let mut pool = Pool::new(n_threads as u32);

    // Then for each of the tiles we encountered, check if placing an obstacle directly in front of the agent would have
    // caused a loop to appear.
    let num_loops = pool.scoped(|scope| {
        let (send, recv) = mpsc::channel();
        for &(pos, dir) in &all_tiles {
            let mut map = map.clone();

            // If the tile in front of us isn't already a wall, add a wall there and then begin the simulation.
            let obs_pos = match in_front(pos, dir, &map) {
                None => continue,
                Some(p) if map[p].is_wall() || p == start_pos => continue,
                Some(p) => p,
            };

            map[obs_pos] = Cell::WALL;

            let send = send.clone();
            scope.execute(move || {
                // Run the entire simulation again from the start just to cover our bases. This could be made much more
                // efficient, but should be good enough for now.
                let loop_detected = run_simulation(start_pos, map, |cell, _, dir| {
                    // If, during the course of this simulation, we encounter a cell that we have already visited, while
                    // also going the same direction we were going before, then we have a loop.
                    if cell.has_been_visited(dir) {
                        ControlFlow::Break(())
                    } else {
                        ControlFlow::Continue(())
                    }
                })
                .is_some();

                if loop_detected {
                    send.send(obs_pos).unwrap();
                }
            });
        }

        drop(send); // Now all senders are in the threads, so the channel will hang up when the last thread finishes.
        recv.iter().collect::<BTreeSet<_>>().len()
    });

    println!("Number of unique tiles encountered (part 1): {}", num_unique);
    println!("Number of possible loops (part 2): {}", num_loops);
}

/// Performs the main loop of running a simulation of an agent in a maze.
///
/// `on_visit` accepts a closure to run once for each cell in the grid. That closure should return a [`ControlFlow`]
/// dictating whether or not the simulation should continue looping or not. If the loop is stopped by a
/// [`ControlFlow::Break`], then the whole function will return a `Some(T)` holding the value contained by the `Break`.
fn run_simulation<T, F>(mut pos: Position, mut map: Grid<Cell>, mut on_visit: F) -> Option<T>
where
    F: FnMut(Cell, Position, Direction) -> ControlFlow<T, ()>,
{
    let mut dir = Direction::Up;
    'outer: loop {
        if let ControlFlow::Break(res) = on_visit(map[pos], pos, dir) {
            break 'outer Some(res);
        }

        map[pos].visit(dir);

        // Step forwards and see if we're still in-bounds. Just in case we hit a corner, we use a loop to determine the
        // next position (may have to turn more than once before stepping forwards).
        'inner: loop {
            match in_front(pos, dir, &map) {
                // If the next step is a wall, rotate and try again.
                Some(next_pos) if map[next_pos].is_wall() => dir.turn_right(),
                // If the next spot isn't a wall, take the step and continue.
                Some(next_pos) => {
                    pos = next_pos;
                    break 'inner;
                },
                // If the next step is out of bounds, stop the whole loop.
                None => break 'outer None,
            }
        }
    }
}

/// Returns the tile directly "in front" of a given position, in the given direction, unless it is outside the bounds of
/// the given grid.
const fn in_front(pos: Position, dir: Direction, map: &Grid<Cell>) -> Option<Position> {
    let (x, y) = pos;
    match dir {
        Direction::Up if y > 0 => Some((x, y - 1)),
        Direction::Left if x > 0 => Some((x - 1, y)),
        Direction::Down if y < map.height() - 1 => Some((x, y + 1)),
        Direction::Right if x < map.width() - 1 => Some((x + 1, y)),
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell(u8);

impl Cell {
    pub const OPEN: Cell = Self(0b0000);
    pub const WALL: Cell = Self(0b1111);

    /// Checks if this cell is a wall.
    pub const fn is_wall(&self) -> bool {
        self.0 == Self::WALL.0
    }

    /// Marks this cell as having been visited while facing the given direction.
    pub const fn visit(&mut self, dir: Direction) {
        if self.is_wall() {
            panic!("cannot visit wall cell");
        } else {
            self.0 |= dir.mask();
        }
    }

    /// Checks if this cell has been visited while facing in a given direction.
    pub const fn has_been_visited(&self, dir: Direction) -> bool {
        !self.is_wall() && (self.0 & dir.mask()) > 0
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self.0 {
            0b0000 => ".",
            0b1111 => "#",
            0b0001..0b1111 => "X",
            _ => "?", // unreachable!(...unless?)
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
    /// Turns this direction 90° to the right.
    pub const fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }

    #[allow(unused)]
    /// Turns this direction 90° to the left.
    pub const fn turn_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };
    }

    /// Returns a bitmask representing this direction.
    pub const fn mask(&self) -> u8 {
        match self {
            Direction::Up => 0b0001,
            Direction::Down => 0b0010,
            Direction::Left => 0b0100,
            Direction::Right => 0b1000,
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
