use aoc_utils::Grid;
use aoc_utils::grid::Neighbours;

type Position = (usize, usize);

fn main() {
    let map = Grid::from_lines(aoc_utils::puzzle_input().lines()).unwrap();

    let mut discovered = Grid::<bool>::empty(map.width(), map.height());
    let mut explored = Grid::<bool>::empty(map.width(), map.height());
    let mut region_stack = Vec::new();

    let mut total_price1 = 0;
    let mut total_price2 = 0;

    for start_pos in map.positions() {
        // If this cell has already been found by traversing from another cell, skip ahead.
        if explored[start_pos] {
            continue;
        }

        // Otherwise, this is the start of a new region: start traversing!
        region_stack.push(start_pos);
        discovered[start_pos] = true;

        let region_char = map[start_pos];
        let mut region_area = 0u32;
        let mut region_edges = 0u32;
        let mut region_perimeter = 0u32;

        while let Some(pos) = region_stack.pop() {
            explored[pos] = true;

            let neighbours = map.neighbours(pos).unwrap();

            // Each cell's contribution to the total perimeter of the region is its number of non-same cells. Start at 4
            // and count down whenever we see a same-character cell.
            let mut perimeter = 4;
            for n_pos in neighbours.iter_adjacent() {
                if map[n_pos] == region_char {
                    perimeter -= 1;
                    if !discovered[n_pos] {
                        region_stack.push(n_pos);
                        discovered[n_pos] = true;
                    }
                }
            }

            region_area += 1;
            region_edges += count_corners(&map, &neighbours) as u32;
            region_perimeter += perimeter;
        }

        total_price1 += region_area * region_perimeter;
        total_price2 += region_area * region_edges;
    }

    println!("Total price of all regions (part 1): {}", total_price1);
    println!("Total price of all regions (part 2): {}", total_price2);
}

/// Counts the corners that a given cell has.
fn count_corners(map: &Grid<char>, neighbours: &Neighbours<Position>) -> u8 {
    let char = map[neighbours.pos()];

    macro_rules! check_match {
        ($dir:ident) => {
            neighbours.$dir().is_some_and(|p| map[p] == char)
        };
    }

    macro_rules! count_bools {
        ($bool:expr) => ($bool as u8);
        ($bool:expr, $($others:expr),+) => ($bool as u8 + count_bools!($($others),+));
    }

    // There's almost certainly a more elegant way to do this, but we can also just brute-force all combinations.
    // - Outside corners: two sides, 45-degrees apart, are *not* the same char.
    // - Inside corners: two sides, 45-degrees apart, *are* the same as this char, but the one between them is not.
    let n = check_match!(n);
    let e = check_match!(e);
    let s = check_match!(s);
    let w = check_match!(w);
    let ne = check_match!(ne);
    let se = check_match!(se);
    let sw = check_match!(sw);
    let nw = check_match!(nw);

    let outside = count_bools!(!n && !e, !e && !s, !s && !w, !w && !n);
    let inside = count_bools!(n && e && !ne, s && e && !se, s && w && !sw, n && w && !nw);

    outside + inside
}
