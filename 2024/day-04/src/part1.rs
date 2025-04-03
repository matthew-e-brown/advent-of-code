// I definitely overcomplicated Part 1 here. Using signed integers for indexing would've drastically simplified the
// `Direction` thing. There's also a lot of overlap with the implementation of Part 2, but Part 2 ended up being way
// simpler; that tells me I was probably being silly and there's a more elegant way to do the searching for 'XMAS'. Oh
// well, I'll come back to this another day.

use std::sync::mpsc;

use aoc_utils::grid::{Dir8, Direction};
use aoc_utils::Grid;

// Thought it'd be helpful to return some more metadata from each match, thinking Part 2 would make use of it... nope.
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct XmasResult {
    pos: (usize, usize),
    dir: Dir8,
}

pub fn main(grid: &Grid<char>) -> usize {
    let mut pool = aoc_utils::threadpool();
    let (send, recv) = mpsc::channel::<XmasResult>();

    pool.scoped(|scope| {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let pos = (x, y);
                if grid[pos] == 'X' {
                    for dir in Dir8::iter() {
                        let send = send.clone();
                        scope.execute(move || scan(grid, pos, dir, send));
                    }
                }
            }
        }
    });

    // Drop the original sender to force the channel to hang up.
    drop(send);

    let mut num = 0;
    while let Ok(_) = recv.recv() {
        num += 1;
    }

    num
}

fn scan(grid: &Grid<char>, mut pos: (usize, usize), dir: Dir8, channel: mpsc::Sender<XmasResult>) {
    let start = pos;
    let mut curr = 'X';
    loop {
        // Move to next position, stopping if we hit the edge.
        match dir.checked_add(pos, grid.size()) {
            Some(p) => pos = p,
            None => break,
        }

        // Look for next character in the sequence; if we hit anything out-of-place, break from the loop.
        let next = grid[pos];
        match (curr, next) {
            ('X', 'M') | ('M', 'A') | ('A', 'S') => curr = next,
            _ => break,
        }

        if curr == 'S' {
            channel.send(XmasResult { pos: start, dir }).unwrap();
            return;
        }
    }
}
