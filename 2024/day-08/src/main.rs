use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

use aoc_utils::Grid;

// cspell:words antinode antinodes

type Position = (usize, usize);

// Went a little extra/convoluted with this one, but I wanted to have some fun with multithreading. Who knows if it
// actually made a performance impact. Oh well! :D
fn main() {
    let input = aoc_utils::puzzle_input();

    let mut antennae = HashMap::new();
    let grid = Grid::from_lines_map(input.lines(), |char, pos| {
        if char != '.' {
            let v = antennae.entry(char).or_insert_with(|| Vec::new());
            v.push(pos);
        }

        false
    })
    .unwrap();

    let grid_size = grid.size();

    let (tx1, rx1) = mpsc::channel(); // Channel for part1 positions
    let (tx2, rx2) = mpsc::channel(); // Channel for part2 positions
    let reducer1 = reducer(rx1, grid.clone());
    let reducer2 = reducer(rx2, grid);

    let mut pool = aoc_utils::threadpool();
    pool.scoped(|scope| {
        for (_code, positions) in &antennae {
            let tx1 = tx1.clone();
            let tx2 = tx2.clone();
            scope.execute(move || scan_antinodes(tx1, tx2, positions, grid_size));
        }
    });

    drop(tx1);
    drop(tx2);

    let num_pos1 = reducer1.join().unwrap();
    let num_pos2 = reducer2.join().unwrap();
    println!("Unique locations with an antinode, part 1 rules: {num_pos1}");
    println!("Unique locations with an antinode, part 2 rules: {num_pos2}");
}

/// Creates an iterator over the cartesian product of a set of items, excluding self-intersection.
fn cartesian_product<'a, T>(items: &'a [T]) -> impl Iterator<Item = (&'a T, &'a T)> + use<'a, T> {
    items
        .iter()
        .flat_map(|a| items.iter().filter(|&b| !std::ptr::eq(a, b)).map(move |b| (a, b)))
}

fn scan_antinodes(tx1: Sender<Position>, tx2: Sender<Position>, positions: &[Position], grid_size: (usize, usize)) {
    // Helper function to convert a signed position to an unsigned one, as well as checking for bounds within the
    // original grid.
    let (grid_w, grid_h) = grid_size;
    let get_idx = |x: isize, y: isize| -> Option<(usize, usize)> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        (x < grid_w && y < grid_h).then_some((x, y))
    };

    // For every antenna `a`, we want to loop over all the other antennae `b` and find the distance from `a` to `b` (as
    // a vector).
    for (a_pos, b_pos) in cartesian_product(&positions) {
        let &(ax, ay) = a_pos;
        let &(bx, by) = b_pos;

        // a -> b = b - a.
        let dx = (bx as isize) - (ax as isize);
        let dy = (by as isize) - (ay as isize);

        // Part 1: Add the distance directly to `b`'s position to find the antinode's position.
        let mut x = bx as isize;
        let mut y = by as isize;
        if let Some(pos) = get_idx(x + dx, y + dy) {
            tx1.send(pos).unwrap();
        }

        // Part 2: Start at `b` and keep adding until we leave the map (see `get_idx` helper above).
        while let Some(pos) = get_idx(x, y) {
            tx2.send(pos).unwrap();
            x = x + dx;
            y = y + dy;
        }
    }
}

/// Spawns a reducer thread that receives positions for the given [`Receiver`] and counts all of the ones which were the
/// first on that tile.
fn reducer(receiver: Receiver<Position>, mut grid: Grid<bool>) -> JoinHandle<u32> {
    std::thread::spawn(move || {
        let mut unique_positions = 0;

        for pos in receiver.iter() {
            let has_antinode = &mut grid[pos];

            if !*has_antinode {
                unique_positions += 1;
            }

            *has_antinode = true;
        }

        unique_positions
    })
}
