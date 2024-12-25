use std::sync::mpsc;

use aoc_utils::CharGrid;
use scoped_threadpool::Pool;

pub fn main(grid: &CharGrid) -> usize {
    // If the grid is smaller than 3x3, there couldn't be any diagonal MAS's. Allows us to subtract from w/h without
    // worrying about underflow:
    if grid.width() < 3 || grid.height() < 3 {
        return 0;
    }

    let n_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8);
    let mut pool = Pool::new(n_threads as u32);

    let (send, recv) = mpsc::channel::<()>();

    pool.scoped(|scope| {
        // Iterate 1 away from the border
        for y in 1..(grid.height() - 1) {
            for x in 1..(grid.width() - 1) {
                if grid[(x, y)] == 'A' {
                    let send = send.clone();
                    scope.execute(move || check(grid, (x, y), send));
                }
            }
        }
    });

    drop(send); // Force channel hangup for the while loop below

    let mut num = 0;
    while let Ok(()) = recv.recv() {
        num += 1;
    }

    num
}

fn check(grid: &CharGrid, pos: (usize, usize), channel: mpsc::Sender<()>) {
    let (x, y) = pos;

    // 'MAS', starting from the top-left and going down-right, or starting from bottom right and going up-left
    let tl = grid[(x - 1, y - 1)] == 'M' && grid[(x + 1, y + 1)] == 'S';
    let br = grid[(x + 1, y + 1)] == 'M' && grid[(x - 1, y - 1)] == 'S';
    // Same for the other diagonal:
    let bl = grid[(x - 1, y + 1)] == 'M' && grid[(x + 1, y - 1)] == 'S';
    let tr = grid[(x + 1, y - 1)] == 'M' && grid[(x - 1, y + 1)] == 'S';

    // If both diagonals have either possibility, we have a match.
    if (tl || br) && (tr || bl) {
        channel.send(()).unwrap();
    }
}
