use std::fmt::Debug;

use aoc_utils::grid::{Grid, GridIndex, Pos};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    /// This cell contains empty space, but may have one or more tachyon beams passing through it.
    Space(u64),
    /// A tachyon beam splitter.
    Splitter,
}

impl Cell {
    pub fn beam_count(self) -> Option<u64> {
        match self {
            Cell::Space(b) => Some(b),
            Cell::Splitter => None,
        }
    }

    pub fn beam_count_mut(&mut self) -> Option<&mut u64> {
        match self {
            Cell::Space(b) => Some(b),
            Cell::Splitter => None,
        }
    }
}

fn main() {
    let input = aoc_utils::puzzle_input();
    let (mut grid, start_pos) = parse_input(input);

    if aoc_utils::verbosity() > 0 {
        println!("Starting grid:");
        println!("{grid:?}");
    }

    // Start by setting the single cell at the starting position to a beam.
    grid[start_pos] = Cell::Space(1);

    let mut splitter_hits = 0usize;
    for y in (start_pos.y() + 1)..grid.height() {
        // For each level in the grid:
        for x in 0..grid.width() {
            // Is there a beam above us?
            if let Cell::Space(src) = grid[(x, y - 1)]
                && src > 0
            {
                // If so, then either:
                // - This cell is a splitter, and the beam should get added to either side.
                // - This cell is empty space, and it should get added to this cell directly.
                match &mut grid[(x, y)] {
                    // We want to add to however many beams are already there (i.e., from surrounding iterations)
                    Cell::Space(cur) => *cur += src,
                    Cell::Splitter => {
                        splitter_hits += 1;
                        for pos in [(x - 1, y), (x + 1, y)] {
                            let neighbour = grid
                                .get_mut(pos)
                                .expect("splitters should not be against map edges")
                                .beam_count_mut()
                                .expect("splitters should be next to empty space");
                            *neighbour += src;
                        }
                    },
                }
            }
        }
    }

    // Now, at the end, we simply need to tally up the total beams in the bottom row:
    let total_branches = (0..grid.width())
        .map(|x| {
            grid[(x, grid.height() - 1)]
                .beam_count()
                .expect("there should be no splitters in the last row") as usize
        })
        .sum::<usize>();

    if aoc_utils::verbosity() > 0 {
        println!("\nFinal grid:");
        println!("{grid:?}");
    }

    println!("Number of tachyon beam splits (part 1): {splitter_hits}");
    println!("Number of possible timelines (part 2): {total_branches}");
}

/// Reads and validates puzzle input.
///
/// Returns the grid and the initial beam position. Initial beam position is pre-seeded with a value of 1.
fn parse_input(input: &str) -> (Grid<Cell>, Pos) {
    let mut start_pos = None;

    let grid = Grid::from_lines_map(input.lines(), |c, pos| match c {
        '.' => Cell::Space(0),
        '^' => Cell::Splitter,
        'S' if start_pos.is_none() => {
            start_pos = Some(pos);
            Cell::Space(1)
        },
        'S' => panic!("invalid puzzle input: multiple 'S' chars"),
        _ => panic!("invalid puzzle input: unknown char {c}"),
    })
    .unwrap();

    let start_pos = start_pos.expect("invalid puzzle input: missing starting position ('S')");
    (grid, start_pos)
}


impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BLACK: &str = "\x1b[38;5;238m"; // Technically dark gray
        const WHITE: &str = "\x1b[38;5;15m";
        const GREEN: &str = "\x1b[38;5;2m";
        const RESET: &str = "\x1b[0m";
        match self {
            Cell::Space(0) => write!(f, "{BLACK}.{RESET}"),
            Cell::Space(_) => write!(f, "{GREEN}|{RESET}"),
            Cell::Splitter => write!(f, "{WHITE}^{RESET}"),
        }
    }
}
