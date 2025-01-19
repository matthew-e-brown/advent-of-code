use std::collections::HashMap;

use aoc_utils::grid::ParseGridError;
use aoc_utils::Grid;

// cspell:words antinode antinodes

type Position = (usize, usize);

struct Cell {
    _antenna: Option<char>,
    antinodes: u32,
}

struct Map {
    grid: Grid<Cell>,
    antennae: HashMap<char, Vec<Position>>,
}

fn main() {
    let input = aoc_utils::puzzle_input();
    let mut map = Map::new(input.lines()).unwrap();
    let mut unique_positions = 0;

    for (&_code, positions) in &map.antennae {
        // For every antenna `a`, we want to loop over all the other antennae `b` and find the distance from `a` to `b`
        // (as a vector). Adding that distance to `b`'s position will then give us the antinode's position.
        for a_pos in positions {
            for b_pos in positions {
                if a_pos == b_pos {
                    continue;
                }

                let &(ax, ay) = a_pos;
                let &(bx, by) = b_pos;

                // a -> b = b - a.
                let dx = (bx as i32) - (ax as i32);
                let dy = (by as i32) - (ay as i32);

                // Careful for underflow:
                let x = usize::try_from((bx as i32) + dx).ok();
                let y = usize::try_from((by as i32) + dy).ok();
                let pos = x.zip(y);
                let cell = pos.and_then(|pos| map.grid.get_mut(pos));

                if let Some(cell) = cell {
                    if cell.antinodes == 0 {
                        unique_positions += 1;
                    }
                    cell.antinodes += 1;
                }
            }
        }
    }

    println!("Unique locations with an antinode (part 1): {unique_positions}");
}


impl Map {
    pub fn new<I, S>(lines: I) -> Result<Map, ParseGridError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut antennae = HashMap::new();
        let grid = Grid::from_lines_map(lines, |c, pos| {
            let antenna = if c != '.' {
                let v = antennae.entry(c).or_insert_with(|| Vec::new());
                v.push(pos);
                Some(c)
            } else {
                None
            };

            Cell { _antenna: antenna, antinodes: 0 }
        })?;

        Ok(Map { grid, antennae })
    }
}
