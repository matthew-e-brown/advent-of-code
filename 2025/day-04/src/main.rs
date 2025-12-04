use std::fmt::Debug;

use aoc_utils::grid::Grid;

fn main() {
    let input = aoc_utils::puzzle_input();
    let map = Grid::from_lines_map(input.lines(), |c, _| match c {
        '@' => Cell::Paper,
        '.' => Cell::Empty,
        _ => panic!("invalid puzzle input: unknown char '{c}'"),
    })
    .unwrap();

    let mut num_reachable = 0usize;
    for pos in map.positions() {
        if map[pos] == Cell::Paper {
            let neighbours = map.neighbours(pos).expect("pos should be in bounds");
            let num_papers = neighbours.iter_around().filter(|&p| map[p] == Cell::Paper).count();
            if num_papers < 4 {
                num_reachable += 1;
            }
        }
    }

    println!("Number of reachable rolls of paper (part 1): {num_reachable}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Paper,
    Empty,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paper => write!(f, "@"),
            Self::Empty => write!(f, "."),
        }
    }
}
