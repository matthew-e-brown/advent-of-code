mod dlx;
mod input;

use self::dlx::Matrix;

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

        let mut builder = Matrix::<Row, Col>::builder();

        todo!();
    }
}

// Placeholders for now
struct Row {}
struct Col {}
