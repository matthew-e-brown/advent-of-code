mod cover;
mod input;

use indexmap::IndexSet;

use self::cover::raw::build::{Constraint, DLXBuilder};
use self::input::Transform;

fn main() {
    let input = aoc_utils::puzzle_input();
    let (shapes, regions) = input::parse_input(input).unwrap();

    // First, start with a simple preliminary check: what's the total surface area of all required presents? Any region
    // whose area is smaller than that amount won't be able to fit them all, period; so we don't need to do anything
    // fancy.
    //
    // Once those regions have been eliminated, we can build our DLX matrix to the maximum size it will need.
    let mut possible_regions = Vec::new();

    for (i, region) in regions.into_iter().enumerate() {
        let counts = region.counts().iter().copied().enumerate();
        let need = counts.fold(0, |total, (i, n)| total + shapes[i].surface_area() * n);
        let have = region.width() * region.height();
        if have < need {
            if aoc_utils::verbosity() > 1 {
                println!("Region {i} failed preliminary area check: {region}");
            }
        } else {
            possible_regions.push(region);
        }
    }

    // If there are any regions remaining, then we can move onto the complicated stuff.
    if possible_regions.len() > 0 {
        // First, figure out what the largest width and height of all the regions are. This will determine the overall
        // size we need for our DLX matrix.
        let mut max_width = possible_regions[0].width();
        let mut max_height = possible_regions[0].height();
        for region in &possible_regions[1..] {
            max_width = max_width.max(region.width());
            max_height = max_height.max(region.height());
        }

        let mut col_labels = IndexSet::<Criteria>::new();
        let mut row_labels = IndexSet::<Choice>::new();
        let mut columns = Vec::with_capacity(shapes.len() + (max_width * max_height));

        // For columns, we need:
        // - One required column for each present shape.
        // - One optional column for each of the tiles in the board.
        for i in 0..shapes.len() {
            columns.push(Constraint::required()); // We will adjust the column counts later
            col_labels.insert(Criteria::Present(i));
        }

        for y in 0..max_height {
            for x in 0..max_width {
                columns.push(Constraint::optional());
                col_labels.insert(Criteria::Tile(x, y));
            }
        }

        let mut builder = DLXBuilder::try_from_constraints(columns).unwrap();

        // Now, for all possible positions of all possible
        for (i, shape) in shapes.iter().enumerate() {
            for transform in Transform::VARIANTS {
                let shape = shape.with_transform(transform);

                let mut y = 0;
                while y + shape.height() < max_height {
                    let mut x = 0;
                    while x + shape.width() < max_width {
                        // (i, (x, y), transform) now give us all the information we need to label the row.
                        row_labels.insert(Choice {
                            present: i,
                            position: (x, y),
                            orientation: transform,
                        });

                        // Now we just need to figure out which columns it intersects with.
                        let col_indices = shape.points().into_iter().map(|&(px, py)| {
                            let tx = x + px;
                            let ty = y + py;
                            let col = Criteria::Tile(tx, ty);
                            col_labels
                                .get_index_of(&col)
                                .expect("all tiles should have been added to the matrix")
                        });

                        builder.try_push_row(col_indices).unwrap();

                        x += 1;
                    }
                    y += 1;
                }
            }
        }

        let mut matrix = builder.finish();

        // Now, for each region, do a prepared search where we configure the columns first.
        for (i, region) in possible_regions.iter().enumerate() {
            let result = matrix.prepared_search(|idx| match &col_labels[idx] {
                &Criteria::Tile(x, y) => region.is_in_bounds(x, y).then_some(1).unwrap_or(0),
                &Criteria::Present(i) => region.counts()[i],
            });

            match result {
                Some(row_indices) => {
                    let mapped = row_indices.into_iter().map(|i| (i, row_labels[i])).collect::<Box<[_]>>();
                    println!("Region #{i}: {mapped:#?}");
                },
                None => println!("Region #{i}: No solution found."),
            }
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Criteria {
    Tile(usize, usize),
    Present(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Choice {
    present: usize,
    position: (usize, usize),
    orientation: Transform,
}
