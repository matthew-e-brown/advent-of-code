use std::borrow::Borrow;
use std::collections::{BTreeSet, VecDeque};
use std::sync::mpsc;

use aoc_utils::Grid;

type Position = (usize, usize);

fn main() {
    let input = aoc_utils::puzzle_input();

    let mut trailheads = Vec::new();
    let map = Grid::try_from_lines_map(input.lines(), |c, pos| {
        // assert!('0' <= c && c <= '9', "puzzle input should only contain digits");
        if !('0' <= c && c <= '9') {
            return Err("puzzle input should only contain digits");
        }

        if c == '0' {
            trailheads.push(pos);
        }

        Ok(c as u32 - '0' as u32)
    })
    .unwrap();

    let (tx, rx) = mpsc::channel();
    let mut pool = aoc_utils::threadpool();

    pool.scoped(|scope| {
        for pos in trailheads {
            let tx = tx.clone();
            let map = &map;
            scope.execute(move || {
                let score = scan_trailhead(map, pos);
                tx.send(score).unwrap();
            });
        }
    });

    drop(tx);

    let score_sum = rx.iter().fold(0, |acc, cur| acc + cur);
    println!("Sum of all trailhead scores (part 1): {score_sum}");
}

/// Performs a breadth-first search for mountain peaks (values of 9) from the given grid starting at the given position.
/// Returns the number of uniq
fn scan_trailhead(map: &Grid<u32>, start_pos: Position) -> usize {
    let mut found = BTreeSet::new(); // Maintain a set of unique peaks for this specific trailhead.
    let mut queue = VecDeque::new(); // Standard BFS queue.
    let mut explored = BTreeSet::new(); // Standard BFS "explored" set.

    queue.push_back(start_pos);
    explored.insert(start_pos);

    while let Some(pos) = queue.pop_front() {
        let val = map[pos];
        if val == 9 {
            found.insert(pos);
        } else {
            for next in find_surrounding(map, pos, val + 1) {
                // insert returns true if this is a fresh insert:
                if explored.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }

    found.len()
}

/// Returns an iterator over all positions around a given position `pos`, accounting for the boundaries of the given
/// grid.
fn surrounding<T>(grid: &Grid<T>, pos: Position) -> impl Iterator<Item = Position> {
    let (x, y) = pos;
    let (w, h) = grid.size();
    [
        (x > 0).then(|| (x - 1, y)),
        (y > 0).then(|| (x, y - 1)),
        (x < w - 1).then(|| (x + 1, y)),
        (y < h - 1).then(|| (x, y + 1)),
    ]
    .into_iter()
    .filter_map(|x| x)
}

/// Filters the result of [`surrounding`] to just those that equal the given value.
fn find_surrounding<'a, 'b, T: Eq, B: Borrow<T>>(
    grid: &'a Grid<T>,
    pos: Position,
    val: B,
) -> impl Iterator<Item = Position> + use<'a, 'b, T, B> {
    surrounding(grid, pos).filter(move |x| &grid[x] == val.borrow())
}
