use std::collections::BTreeSet;
use std::fmt::Display;
use std::ops::ControlFlow;
use std::sync::mpsc;

use aoc_utils::grid::{Dir4, Direction, Pos};
use aoc_utils::Grid;

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

    let mut pool = aoc_utils::threadpool();

    // Then for each of the tiles we encountered, check if placing an obstacle directly in front of the agent would have
    // caused a loop to appear.
    let num_loops = pool.scoped(|scope| {
        let (send, recv) = mpsc::channel();
        for &(check_pos, dir) in &all_tiles {
            let mut map = map.clone();

            // If the tile in front of us isn't already a wall, add a wall there and then begin the simulation.
            let obs_pos = match dir.checked_add(check_pos, map.size()) {
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
fn run_simulation<T, F>(mut pos: Pos, mut map: Grid<Cell>, mut on_visit: F) -> Option<T>
where
    F: FnMut(Cell, Pos, Dir4) -> ControlFlow<T, ()>,
{
    let mut dir = Dir4::Up;
    loop {
        if let ControlFlow::Break(res) = on_visit(map[pos], pos, dir) {
            break Some(res);
        }

        map[pos].visit(dir);

        // Step forwards and see if we're about to hit a wall or if we've stepped out of bounds or not. When turning
        // right, let the loop restart so `on_visit` can re-run for the new direction.
        match dir.checked_add(pos, map.size()) {
            Some(next_pos) if map[next_pos].is_wall() => dir = dir.right(),
            Some(next_pos) => pos = next_pos,
            None => break None,
        }
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
    pub const fn visit(&mut self, dir: Dir4) {
        if self.is_wall() {
            panic!("cannot visit wall cell");
        } else {
            self.0 |= dir_mask(dir);
        }
    }

    /// Checks if this cell has been visited while facing in a given direction.
    pub const fn has_been_visited(&self, dir: Dir4) -> bool {
        !self.is_wall() && (self.0 & dir_mask(dir)) > 0
    }
}

const fn dir_mask(dir: Dir4) -> u8 {
    match dir {
        Dir4::Up => 0b0001,
        Dir4::Down => 0b0010,
        Dir4::Left => 0b0100,
        Dir4::Right => 0b1000,
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
