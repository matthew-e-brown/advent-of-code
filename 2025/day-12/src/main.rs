mod cover;
mod input;

use self::cover::CoverProblem;
use self::cover::build::Constraint;
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

        // [TODO] `with_capacity`?
        let mut builder = CoverProblem::build();
        builder.reserve(shapes.len() + (max_width * max_height));

        // For columns, we need:
        // - One required column for each present shape.
        // - One optional column for each of the tiles in the board.
        for i in 0..shapes.len() {
            // We will adjust the column counts later
            builder.try_push_constraint(Constraint::required(Criteria::Present(i))).unwrap();
        }

        for y in 0..max_height {
            for x in 0..max_width {
                builder.try_push_constraint(Constraint::optional(Criteria::Tile(x, y))).unwrap();
            }
        }

        let mut builder = builder.finish_constraints();

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
                        let placement = PresentPlacement { index: i, position: (x, y), transform };
                        let subset = shape.points().into_iter().map(|&(px, py)| Criteria::Tile(x + px, y + py));

                        builder.try_push_subset(placement, subset).unwrap();

                        x += 1;
                    }
                    y += 1;
                }
            }
        }

        let mut problem = builder.build();

        // Now, for each region, do a prepared search where we configure the columns first.
        for (i, region) in possible_regions.iter().enumerate() {
            let result = problem.prepared_search(|idx| match &col_labels[idx] {
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
struct PresentPlacement {
    index: usize,
    position: (usize, usize),
    transform: Transform,
}
