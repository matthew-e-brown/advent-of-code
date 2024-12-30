use std::collections::BTreeSet;
use std::fmt::{Debug, Display};
use std::sync::mpsc;

use aoc_utils::Grid;
use scoped_threadpool::Pool;

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut start_pos = None;
    let mut map = Grid::from_lines_map(input.lines(), |c, pos| match c {
        '#' => Cell::Wall,
        '.' => Cell::Open,
        '^' => {
            start_pos = Some(pos);
            Cell::Open
        },
        _ => panic!("invalid input: unexpected char '{c}'"),
    })
    .unwrap();

    let n_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8) as u32;
    let mut thread_pool = Pool::new(n_threads);
    let (send, recv) = mpsc::channel::<(usize, usize)>();

    let mut pos = start_pos.expect("invalid input: missing start character ('^')");
    let mut dir = Direction::Up;
    let mut unique_tiles: usize = 0;
    let mut threads_spawned: usize = 0;

    thread_pool.scoped(|scope| {
        loop {
            // If the cell we're currently on has *never* been visited, from any direction, count it as unique.
            if map[pos] == Cell::Open {
                unique_tiles += 1;
            }

            // If this is the first time we've visited this cell while travelling in this particular direction, spawn a
            // thread to run a separate simulation to check for potential loops.
            if map[pos].visit(dir) {
                // In my Part 1 solution, 5444 unique tiles are visited. Our map is 130x130 2-byte tiles (enums). If we
                // assume some tiles are visited twice in different directions, and say there are about 10,000
                // visitations in total, then cloning this map 10,000 times will cost 130*130*2*10000 bytes or ~322 MiB
                // in total. That's kind of a lot, but... it's not terrible, so I'm okay with this strategy.
                let map = map.clone();
                let send = send.clone();
                threads_spawned += 1;
                scope.execute(move || {
                    if check_loop(pos, dir, map) {
                        send.send(pos).unwrap();
                    }
                });
            }

            // Step forwards and see if we're still in-bounds.
            if !step(&mut pos, &mut dir, &map) {
                break;
            }
        }
    });

    drop(send); // Drop original handle so that the completion of the last thread causes a hangup
    // Collect into a set before counting so that we can count *unique* positions.
    let possible_loops = recv.iter().collect::<BTreeSet<_>>().len();

    println!("Number of unique tiles encountered (part 1): {unique_tiles}");
    println!("Number of possible loops (part 2): {possible_loops}");
    println!("(Number of threads spawned: {threads_spawned})");
}

/// Returns the tile directly "in front" of a given position, in the given direction, unless it is outside the bounds of
/// the given grid.
fn in_front(pos: (usize, usize), dir: Direction, map: &Grid<Cell>) -> Option<(usize, usize)> {
    let (x, y) = pos;
    match dir {
        Direction::Up if y > 0 => Some((x, y - 1)),
        Direction::Left if x > 0 => Some((x - 1, y)),
        Direction::Down if y < map.height() - 1 => Some((x, y + 1)),
        Direction::Right if x < map.width() - 1 => Some((x + 1, y)),
        _ => None,
    }
}

/// Takes a step "forwards" in the given grid, turning if necessary.
///
/// Returns `true` if the new position is still within the bounds of the given `map` (`pos` is not actually mutated in
/// the event of a would-be out-of-bounds move).
fn step(pos: &mut (usize, usize), dir: &mut Direction, map: &Grid<Cell>) -> bool {
    // Just in case we hit a corner or something, use a loop to determine the next position.
    loop {
        match in_front(*pos, *dir, map) {
            // If the next step is a wall, rotate and let the loop try again.
            Some(next_pos) if map[next_pos] == Cell::Wall => dir.turn_right(),
            // Otherwise, step forwards to the next position.
            Some(next_pos) => {
                *pos = next_pos;
                return true;
            },
            // If the next step is out of bounds, we can stop the whole loop.
            None => return false,
        }
    }
}

/// Runs a modified version of the simulation to see if placing an obstacle directly in front of the given position
/// would cause a loop.
fn check_loop(mut pos: (usize, usize), mut dir: Direction, mut map: Grid<Cell>) -> bool {
    // Start by placing an obstacle directly in front of the current position:
    match in_front(pos, dir, &map) {
        Some(new_pos) => map[new_pos] = Cell::Wall,
        None => return false,
    }

    // Now turn right and start looping to see if we eventually create a loop. The only two possible outcomes are (1) a
    // loop is created, which we can detect by seeing if we have visited a given cell before; and (2) the agent
    // eventually escapes the map.
    dir.turn_right();
    loop {
        // `.visit` returns true for any *new* visitations; if we get a false, we know we've been here before.
        if !map[pos].visit(dir) {
            break true;
        } else if !step(&mut pos, &mut dir, &map) {
            break false;
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    /// This cell is empty and has not yet been visited.
    Open,
    /// This cell contains an obstacle.
    Wall,
    /// This cell has been visited at least once travelling in a given direction.
    Visited(u8),
}

impl Cell {
    /// Marks the current cell as having been visited in the given direction.
    ///
    /// Returns `true` if this visitation is the first time the cell has been visited from this direction.
    pub fn visit(&mut self, dir: Direction) -> bool {
        let mask = dir.mask() & 0x0F;
        match self {
            Cell::Wall => panic!("cannot visit wall cell"),
            Cell::Open => {
                *self = Cell::Visited(mask);
                true
            },
            Cell::Visited(bits) if (*bits & mask) == 0 => {
                *bits |= mask;
                true
            },
            Cell::Visited(_) => false, // Bit is already set in bitmask, don't need to re-OR
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Cell::Open => ".",
            Cell::Wall => "#",
            Cell::Visited(_) => "X",
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
