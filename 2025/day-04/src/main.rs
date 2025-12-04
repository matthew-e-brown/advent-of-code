use std::fmt::Debug;

use aoc_utils::Grid;

fn main() {
    let input = aoc_utils::puzzle_input();
    let mut map = Grid::from_lines_map(input.lines(), |c, _| match c {
        '@' => Cell::Paper,
        '.' => Cell::Empty,
        _ => panic!("invalid puzzle input: unknown char '{c}'"),
    })
    .unwrap();

    let mut count1 = 0usize;
    find_reachable_papers(&mut map, |_| count1 += 1);

    let mut count2 = 0usize;
    loop {
        let mut num = 0usize;
        find_reachable_papers(&mut map, |cell| {
            *cell = Cell::Empty;
            num += 1;
        });

        if num == 0 {
            break;
        }

        count2 += num;
    }

    println!("Number of initially reachable rolls (part 1): {count1}");
    println!("Total Number of paper rolls removed (part 2): {count2}");
}

fn find_reachable_papers(map: &mut Grid<Cell>, mut cb: impl FnMut(&mut Cell)) {
    for pos in map.positions() {
        if map[pos] == Cell::Paper {
            let neighbours = map.neighbours(pos).expect("pos should be in bounds");
            let num_papers = neighbours.iter_around().filter(|&p| map[p] == Cell::Paper).count();
            if num_papers < 4 {
                cb(&mut map[pos]);
            }
        }
    }
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
