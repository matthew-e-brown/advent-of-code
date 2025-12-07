use std::fmt::Debug;

use aoc_utils::grid::{Dir4, Grid, GridIndex, Pos};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Beam,
    Splitter,
}

fn main() {
    let input = aoc_utils::puzzle_input();
    let (mut grid, start_pos) = parse_input(input);

    if aoc_utils::verbosity() > 0 {
        println!("Starting grid:");
        println!("{grid:?}");
    }

    // Start by setting the single cell at the starting position to a beam.
    grid[start_pos] = Cell::Beam;

    let mut splitter_hits = 0usize;
    for y in (start_pos.y() + 1)..grid.height() {
        // For each level in the grid:
        for x in 0..grid.width() {
            let pos = (x, y);
            // - Is there a beam above us? If so, it should extend downwards.
            // - If we are a splitter, it should extend to our left and our right.
            if grid[pos + Dir4::Up] == Cell::Beam {
                if grid[pos] == Cell::Splitter {
                    grid[pos + Dir4::Left] = Cell::Beam;
                    grid[pos + Dir4::Right] = Cell::Beam;
                    splitter_hits += 1;
                } else if grid[pos] == Cell::Empty {
                    grid[pos] = Cell::Beam;
                }
            }
        }
    }

    if aoc_utils::verbosity() > 0 {
        println!("\nFinal grid:");
        println!("{grid:?}");
    }

    println!("Total number of tachyon beam splits (part 1): {splitter_hits}");
}

/// Reads and validates puzzle input.
///
/// Returns the grid and the initial beam position.
fn parse_input(input: &str) -> (Grid<Cell>, Pos) {
    let mut start_pos = None;

    let grid = Grid::from_lines_map(input.lines(), |c, pos| match c {
        '.' => Cell::Empty,
        '^' => Cell::Splitter,
        'S' if start_pos.is_none() => {
            start_pos = Some(pos);
            Cell::Empty
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
            Cell::Empty => write!(f, "{BLACK}.{RESET}"),
            Cell::Beam => write!(f, "{GREEN}|{RESET}"),
            Cell::Splitter => write!(f, "{WHITE}^{RESET}"),
        }
    }
}
