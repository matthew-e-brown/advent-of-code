use std::collections::BTreeSet;
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

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let mut pool = aoc_utils::threadpool();

    pool.scoped(|scope| {
        for pos in trailheads {
            let tx1 = tx1.clone();
            let tx2 = tx2.clone();
            let map = &map;
            scope.execute(move || {
                let (score, rating) = scan_trailhead(map, pos);
                tx1.send(score).unwrap();
                tx2.send(rating).unwrap();
            });
        }
    });

    drop(tx1);
    drop(tx2);

    let score_sum = rx1.iter().fold(0, |acc, cur| acc + cur);
    let rating_sum = rx2.iter().fold(0, |acc, cur| acc + cur);
    println!("Sum of all trailhead scores (part 1): {score_sum}");
    println!("Sum of all trailhead ratings (part 2): {rating_sum}");
}

/// Performs a depth-first search of the given `map` for mountain peaks (values of 9) that are accessible from the given
/// `start_pos`.
///
/// Returns both the position's _score_ (the number of unique peaks accessible from `start_pos`) and its _rating_ (the
/// number of unique trails that start at `start_pos`).
fn scan_trailhead(map: &Grid<u32>, start_pos: Position) -> (usize, usize) {
    /// Performs a depth-first search down a trail and looks for any 9s, inserting them into `found` when found. Returns
    /// the number of unique trails that successfully found a 9 (the trail's rating) from this starting position.
    fn dfs(map: &Grid<u32>, pos: Position, found: &mut BTreeSet<Position>) -> usize {
        let val = map[pos];
        if val == 9 {
            // Base case: if this is a 9, it has access to one 9 through one unique path.
            found.insert(pos);
            1
        } else {
            // If this is not a 9, then we ask all neighbours how many unique paths to 9s they all have. For example, if
            // we have 3 neighbours---left, right, and down---and they have 2, 1, and 3 unique trails starting from each
            // of them, then this tile has a total of 6 unique trails starting from it. Any tiles that fail to find a 9
            // from their position will have a zero sum. NB: we don't bother with an "explored" queue here, since we
            // always want to fully explore all paths; otherwise, we don't know how many unique trails they may
            // contribute.
            map.neighbours(pos)
                .unwrap()
                .iter_adjacent()
                .filter_map(|p| (map[p] == val + 1).then(|| dfs(map, p, found)))
                .sum()
        }
    }

    let mut found = BTreeSet::new();
    let rating = dfs(map, start_pos, &mut found);
    let score = found.len();
    (score, rating)
}
