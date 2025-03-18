use std::collections::BTreeMap;
use std::num::NonZeroU32;

use aoc_utils::Grid;

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Region {
    pub char: char,
    pub first_pos: Position,
    pub total_area: usize,
    pub total_perimeter: usize,
}

fn main() {
    let input = Grid::from_lines(aoc_utils::puzzle_input().lines()).unwrap();

    let mut next_id = 1;
    let mut region_ids = Grid::<Option<NonZeroU32>>::empty(input.width(), input.height());
    let mut region_data = BTreeMap::<NonZeroU32, Region>::new();

    for y in 0..input.height() {
        for x in 0..input.width() {
            let pos = (x, y);
            let char = input[pos];

            // Check and see if any neighbouring cells (with the same character) have already been assigned to a region;
            // we can add this cell to that region.
            let neighbours = input.neighbours(pos).unwrap().iter_adjacent().filter(|&p| input[p] == char);

            // Either find the ID of the existing region that this cell should attach to, or come up with a new ID.
            let region_id = neighbours.clone().find_map(|pos| region_ids[pos]).unwrap_or_else(|| {
                let id = NonZeroU32::new(next_id).unwrap();
                next_id += 1;
                id
            });

            // If it's a new ID, we will have to create a new empty entry.
            let region = region_data.entry(region_id).or_insert_with(|| Region {
                char,
                first_pos: pos,
                total_area: 0,
                total_perimeter: 0,
            });

            // The number of same-character neighbours we *don't* have is our in-region perimeter for this cell.
            region.total_perimeter += 4 - neighbours.count();
            region.total_area += 1;

            // Be sure to note which region this cell is attached to.
            region_ids[pos] = Some(region_id);
        }
    }

    println!("Number of regions found: {}", region_data.len());
    println!("{:#2?}\n", region_ids.map(|n| n.map(|n| n.get()).unwrap_or_default()));

    let mut total_price = 0;
    for (&id, region) in region_data.iter() {
        let price = region.total_area * region.total_perimeter;
        println!(
            "Region {id} with char {}, found starting at position {:?} has total price {} * {} = {}",
            region.char, region.first_pos, region.total_area, region.total_perimeter, price,
        );
        total_price += price;
    }

    println!("\nTotal price of all regions (part 1): {}", total_price);
}


// #[derive(Debug, Clone, Copy)]
// struct Region {
//     /// The character for this region.
//     pub char: char,
//     /// The position of the primary (first) cell in this region.
//     pub pos: Position,
// }

// struct Map {
//     grid: Grid<Cell>,
// }
