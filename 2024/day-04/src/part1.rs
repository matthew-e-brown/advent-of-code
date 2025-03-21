// I definitely overcomplicated Part 1 here. Using signed integers for indexing would've drastically simplified the
// `Direction` thing. There's also a lot of overlap with the implementation of Part 2, but Part 2 ended up being way
// simpler; that tells me I was probably being silly and there's a more elegant way to do the searching for 'XMAS'. Oh
// well, I'll come back to this another day.

use std::ops::Add;
use std::sync::mpsc;

use aoc_utils::Grid;

// Thought it'd be helpful to return some more metadata from each match, thinking Part 2 would make use of it... nope.
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
struct XmasResult {
    pos: (usize, usize),
    dir: Direction,
}

pub fn main(grid: &Grid<char>) -> usize {
    let mut pool = aoc_utils::threadpool();
    let (send, recv) = mpsc::channel::<XmasResult>();

    pool.scoped(|scope| {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let pos = (x, y);
                if grid[pos] == 'X' {
                    for dir in Direction::all() {
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

fn scan(grid: &Grid<char>, mut pos: (usize, usize), dir: Direction, channel: mpsc::Sender<XmasResult>) {
    let start = pos;
    let mut curr = 'X';
    loop {
        // Move to next position, stopping if we hit the edge.
        if dir.can_add_to(pos, grid.size()) {
            pos = pos + dir;
        } else {
            break;
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


#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub const fn all() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }

    pub const fn can_add_to(&self, pos: (usize, usize), limits: (usize, usize)) -> bool {
        let (x, y) = pos;
        let (w, h) = limits;
        // Actual width and height limits are based on w-1, h-1:
        let h = h.saturating_sub(1);
        let w = w.saturating_sub(1);
        match self {
            Direction::Up if y == 0 => false,
            Direction::Down if y >= h => false,
            Direction::Left if x == 0 => false,
            Direction::Right if x >= w => false,
            Direction::UpLeft if x == 0 || y == 0 => false,
            Direction::UpRight if x >= w || y == 0 => false,
            Direction::DownLeft if x == 0 || y >= h => false,
            Direction::DownRight if x >= w || y >= h => false,
            _ => true,
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;
        match rhs {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::UpLeft => (x - 1, y - 1),
            Direction::UpRight => (x + 1, y - 1),
            Direction::DownLeft => (x - 1, y + 1),
            Direction::DownRight => (x + 1, y + 1),
        }
    }
}
