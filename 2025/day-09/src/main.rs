mod shapes;
mod svg;

use self::shapes::{Line, Point, Polygon, Rectangle};

fn main() {
    let input = aoc_utils::puzzle_input();
    let points = input
        .lines()
        .map(|line| line.parse::<Point>().expect("Puzzle input should be valid"));

    // Look... I'm kinda tired today. Sometimes, you just gotta go for the good'ole fashioned O(n²) double-for loop. But
    // that doesn't mean we can't speed things up! I'll check all possible rectangles by splitting them up into
    // different threads.
    let polygon = Polygon::from_points(points);
    let all_rectangles = compute_rectangles(polygon.points());

    if aoc_utils::verbosity() >= 5 {
        write_svg(&polygon);
        return;
    }

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get() / 2)
        .unwrap_or(8)
        .min(all_rectangles.len());
    let job_count = all_rectangles.len() / num_threads; // All threads get at least this many jobs
    let remainder = all_rectangles.len() % num_threads; // But the first `remainder` of them get one extra

    let (max_overall, max_inside) = std::thread::scope(|scope| {
        let polygon = &polygon;
        let all_rectangles = &all_rectangles[..];

        let mut start = 0;
        let mut handles = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            let count = job_count + (i < remainder) as usize;
            let group = &all_rectangles[start..start + count];

            let handle = scope.spawn(|| find_largest_rectangles(group, polygon));
            handles.push(handle);

            start += count;
        }

        let mut max_overall = 0;
        let mut max_inside = 0;
        for (i, handle) in handles.into_iter().enumerate() {
            let (overall, inside) = handle.join().expect("thread panicked");

            if aoc_utils::verbosity() >= 1 {
                println!("Thread {i} returned values ({overall}, {inside})", i = i + 1);
            }

            max_overall = max_overall.max(overall);
            max_inside = max_inside.max(inside);
        }

        (max_overall, max_inside)
    });

    println!("Area of largest rectangle between red-tile corners (part 1): {max_overall}");
    println!("Area when limited to just red and green tiles (part 2): {max_inside}");
}


/// Gets all possible unique rectangles that can be formed by a series of *n* points.
///
/// In total, *n²* total rectangles could be formed by *n* points. However, rectangle *i,j* and *j,i* would be
/// duplicates; also, we can skip any *i,i* rectangles. This gives a total of *(n² - n)/2* rectangles
/// (*∑(i=0..n)(∑(j=i+1..n)1)*).
fn compute_rectangles(points: &[Point]) -> Vec<Rectangle> {
    let n = points.len();
    let mut rectangles = Vec::with_capacity((n * n - n) / 2);
    for i in 0..n {
        for j in i + 1..n {
            let p1 = points[i];
            let p2 = points[j];
            rectangles.push(Rectangle::new(p1, p2))
        }
    }
    rectangles
}

/// For a set of rectangles, finds:
/// 1.  The largest rectangle overall.
/// 2.  The largest rectangle that is wholly contained within the given polygon.
fn find_largest_rectangles(rectangles: &[Rectangle], polygon: &Polygon) -> (u64, u64) {
    let mut max_overall = 0;
    let mut max_inside = 0;

    for rect in rectangles {
        let area = rect.area();

        max_overall = max_overall.max(area);
        if !rect_crosses_poly(rect, polygon) {
            // From back before switching the Rectangle struct from p1,p2 to l,r,t,b:
            /* if aoc_utils::verbosity() >= 2 {
                println!("Rectangle between {} and {} (area {}) fits inside polygon", rect.p1, rect.p2, area);
            } */

            max_inside = max_inside.max(area);
        } else {
            /* if aoc_utils::verbosity() >= 3 {
                println!("\tRectangle between {} and {} (area {}) does not fit inside polygon", rect.p1, rect.p2, area);
            } */
        }
    }

    (max_overall, max_inside)
}

/// Determines if the given rectangle "intersects" with the given polygon.
fn rect_crosses_poly(rect: &Rectangle, polygon: &Polygon) -> bool {
    // The basic idea is as in <https://stackoverflow.com/a/4833823/10549827>: we just need to check if the rectangle
    // and the polygon intersect. But there are a few adjustments we need to make:
    //
    // - We know that all our rectangles are constructed entirely from points of the given polygon, so we don't need to
    //   check if any of the rectangle's points are contained within the polygon.
    // - Lines in this puzzle have an inherent thickness of 1 unit; we don't care when two lines *overlap*, only when
    //   they *intersect*. Hence why this function is called `crosses` instead of `intersects`.
    // - We know that all the lines in our puzzle are either strictly vertical or strictly horizontal, which simplifies
    //   things massively.
    for line in polygon.edges() {
        if line_crosses_rect(&line, &rect) {
            return true;
        }
    }

    false
}

/// Detects if a line crosses into a rectangle.
fn line_crosses_rect(line: &Line, rect: &Rectangle) -> bool {
    let Line { a, b } = line;

    // There is almost certainly
    if line.is_horizontal() {
        let ly = line.a.y;
        // If this horizontal line is not between the top and bottom edges of the rectangle, it cannot possibly
        // intersect. Note that this specifically excludes any lines that pass directly along the top or bottom of the
        // rectangle; those never count as crossings. Any lines that do pass directly through the top or bottom line of
        // the rectangle will cause a crossing if the vertical edge they form a corner with descends down into the
        // rectangle (or ascends up into it).
        if rect.t < ly && ly < rect.b {
            // If either of the rectangle's L/R edges are *between* the line's endpoints, we have a cross.
            let [line_l, line_r] = if a.x <= b.x { [a.x, b.x] } else { [b.x, a.x] };
            (line_l <= rect.l && rect.l < line_r) || (line_l < rect.r && rect.r <= line_r)
        } else {
            false
        }
    } else if line.is_vertical() {
        let lx = line.a.x;
        // Is this vertical line between the left and right edges of the rectangle?
        if rect.l < lx && lx < rect.r {
            // If so, then we have a cross as long as the rectangle's top edge or bottom edge is between the top and
            // bottom of the line.
            let [line_t, line_b] = if a.y <= b.y { [a.y, b.y] } else { [b.y, a.y] };
            (line_t <= rect.t && rect.t < line_b) || (line_t < rect.b && rect.b <= line_b)
        } else {
            false
        }
    } else {
        panic!("Encountered diagonal line");
    }
}

/// Writes the given polygon to an SVG file based on the current puzzle input filename.
///
/// Used for debugging/visualization.
fn write_svg(polygon: &Polygon) {
    println!("Writing puzzle visualization to SVG file...");

    let input_name = aoc_utils::puzzle_input_filename();
    let output_name = input_name.with_extension("svg");
    let svg = svg::render_polygon(&polygon);

    std::fs::write(output_name, svg).expect("Writing to file failed.");
    println!("Done.");
}
