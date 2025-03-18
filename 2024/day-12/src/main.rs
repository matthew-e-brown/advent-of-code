use std::collections::BTreeMap;
use std::num::NonZeroU32;

use aoc_utils::Grid;

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Region {
    pub char: char,
    #[allow(unused)]
    pub id: NonZeroU32,
    pub first_pos: Position,
    pub total_area: usize,
    pub total_perimeter: usize,
}

fn main() {
    let input = Grid::from_lines(aoc_utils::puzzle_input().lines()).unwrap();

    let mut next_id = 1;

    // We don't even really need to store region info, since we compute area and perimeter one whole region at a time.
    // But I suspect that I may need it for part 2, so I'll keep it for now.
    let mut region_data = BTreeMap::<NonZeroU32, Region>::new();

    // Map that denotes which region each element is apart of.
    let mut region_map = Grid::<Option<NonZeroU32>>::empty(input.width(), input.height());
    let mut discovered = Grid::<bool>::empty(input.width(), input.height());
    let mut region_stack = Vec::new();

    for pos in input.positions() {
        // If this cell has already been found by traversing from another cell, skip ahead.
        if region_map[pos].is_some() {
            continue;
        }

        // Otherwise, this is the start of a new region! Start traversing.
        let char = input[pos];
        let region_id = NonZeroU32::new(next_id).unwrap();
        let region = region_data.entry(region_id).or_insert(Region {
            char,
            id: region_id,
            first_pos: pos,
            total_area: 0,
            total_perimeter: 0,
        });

        next_id += 1;

        region_stack.push(pos);
        discovered[pos] = true;
        while let Some(pos) = region_stack.pop() {
            let neighbours = input.neighbours(pos).unwrap().iter_adjacent().filter(|&p| input[p] == char);
            let mut perimeter = 4;
            for n_pos in neighbours {
                perimeter -= 1;
                if !discovered[n_pos] {
                    region_stack.push(n_pos);
                    discovered[n_pos] = true;
                }
            }

            region_map[pos] = Some(region_id);
            region.total_area += 1;
            region.total_perimeter += perimeter;
        }
    }

    // Unwrap all the `Option<NonZeroU32>` to regular u32:
    let region_map = region_map.map(|n| n.map(|n| n.get()).unwrap_or_default());

    println!("Number of regions found: {}", region_data.len());
    println!("{:#3?}\n", region_map);

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
