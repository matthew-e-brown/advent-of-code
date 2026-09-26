mod cover;
mod input;

use self::cover::raw::{Matrix, build};
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

        // [TODO] Do I even need a whole separate "high-level" API? Or does the improved builder API make it simple
        // enough to just make the IndexMaps out here... Honestly, I think it might...

        // For columns, we need:
        // - One required column for each present shape.
        // - One optional column for each of the tiles in the board.
        let num_cols = shapes.len() + (max_width * max_height);
        let mut col_labels = indexmap::IndexSet::<Criteria>::with_capacity(num_cols);
        let mut row_labels = indexmap::IndexSet::<PresentPlacement>::new();

        // [TODO] `with_capacity`?
        let mut builder = Matrix::builder();

        for i in 0..shapes.len() {
            // We will adjust the column counts later.
            col_labels.insert(Criteria::Present(i));
            builder.push_column(build::Column::required());
            // [TODO] `builder.column().required()` would be better; .column() would push a new one into the list and
            // return a `&mut` (right now this specific method just returns `()`).
        }

        for y in 0..max_height {
            for x in 0..max_width {
                col_labels.insert(Criteria::Tile(x, y));
                builder.push_column(build::Column::optional());
            }
        }

        let mut builder = builder.finish_columns();

        // Now, for all possible positions of all possible
        for (i, shape) in shapes.iter().enumerate() {
            for transform in Transform::VARIANTS {
                let shape = shape.with_transform(transform);

                let mut y = 0;
                while y + shape.height() < max_height {
                    let mut x = 0;
                    while x + shape.width() < max_width {
                        // (i, (x, y), transform) now give us all the information we need to label the choice. Now we
                        // just need to figure out which columns it intersects with.
                        row_labels.insert(PresentPlacement { index: i, position: (x, y), transform });

                        let columns = shape.points().into_iter().map(|&(px, py)| {
                            // - There are `shape.len()` points before tile (0, 0).
                            // - Then the tiles (x, 0) live at indices (shapes.len() + x);
                            // - Then, the tiles (x, y) live at (shapes.len() + x + (max_width * y)).
                            let tx = x + px;
                            let ty = y + py;
                            shapes.len() + tx + (ty * max_width)
                        });

                        builder.push_row(columns);

                        x += 1;
                    }
                    y += 1;
                }
            }
        }

        let mut problem = builder.build();

        // Now, for each region, do a prepared search where we configure the columns first.
        for (i, region) in possible_regions.iter().enumerate() {
            let result = problem.prepared_search(|idx| match col_labels[idx] {
                Criteria::Present(i) => region.counts()[i],
                Criteria::Tile(x, y) => region.is_in_bounds(x, y).then_some(1).unwrap_or(0),
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
    Present(usize),
    Tile(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PresentPlacement {
    index: usize,
    position: (usize, usize),
    transform: Transform,
}
