mod shapes;
mod svg;

use self::shapes::{Line, Point, Polygon, Rectangle};

// Attempted answers:
// - 269731960 (too low)

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
            if aoc_utils::verbosity() >= 2 {
                println!("Rectangle between {} and {} (area {}) fits inside polygon", rect.p1, rect.p2, area);
            }

            max_inside = max_inside.max(area);
        } else {
            if aoc_utils::verbosity() >= 3 {
                println!("\tRectangle between {} and {} (area {}) does not fit inside polygon", rect.p1, rect.p2, area);
            }
        }
    }

    (max_overall, max_inside)
}

/// Determines if the given rectangle "intersects" with the given polygon.
fn rect_crosses_poly(rect: &Rectangle, polygon: &Polygon) -> bool {
    // Starting from <https://stackoverflow.com/a/4833823/10549827>, there are a few simplifications we can make:
    //
    // - We know that all our rectangles are constructed entirely from points of the given polygon, so we don't need to
    //   check if any of the rectangle's points are contained within the polygon.
    // - Lines in this puzzle have an inherent thickness of 1 unit; we don't care when two lines *overlap*, only when
    //   they *intersect*. Hence why this function is called `crosses` instead of `intersects`.
    // - We know that all the lines in our puzzle are either strictly vertical or strictly horizontal.
    // - So, the only way that two lines can "intersect" is if one is horizontal and one is vertical.
    //
    // That means that our rectangle is inside of our polygon as long as *none* of the edges cross.
    let rect_lines = rect.edges();
    for poly_line in polygon.edges() {
        for &rect_line in &rect_lines {
            if line_crosses_line(&rect_line, &poly_line) {
                if aoc_utils::verbosity() >= 3 {
                    println!("{rect_line:?} intersects {poly_line:?}");
                }
                return true;
            }
        }
    }

    false
}

fn line_crosses_line(l1: &Line, l2: &Line) -> bool {
    // Figure out which of the two is vertical and which is horizontal:
    // (as mentioned above, those are the only cases where an intersection can happen)
    let (v, h) = if l1.is_vertical() && l2.is_horizontal() {
        // | vs. --
        (l1, l2)
    } else if l1.is_horizontal() && l2.is_vertical() {
        // -- vs. |
        (l2, l1)
    } else {
        return false;
    };

    let (vt, vb) = if v.a.y <= v.b.y { (v.a, v.b) } else { (v.b, v.a) }; // Vertical's top/bottom
    let (hl, hr) = if h.a.x <= h.b.x { (h.a, h.b) } else { (h.b, h.a) }; // Horizontal's left/right

    // The vertical line's x position and the horizontal line's y position are both the same regardless
    // of which start/end point we take it from:
    let vx = v.a.x;
    let hy = h.a.y;

    // If the x coordinate of the vertical line is not between the endpoints of the horizontal line,
    // or the y coordinate of the horizontal line is not between the endpoints of the vertical line,
    // there is no intersection:
    if (vx < hl.x || vx > hr.x) || (hy < vt.y || hy > vb.y) {
        return false;
    }

    // Otherwise, if the vertical line's two y coordinates are on opposite sides of the horizontal line,
    // or the horizontal line's x coordinates are on opposite sides of the vertical line,
    // we have an intersection!
    if (vt.y < hy && vb.y > hy) || (hl.x < vx && hr.x > vx) {
        return true;
    }

    false
}


/// For debugging.
fn write_svg(polygon: &Polygon) {
    println!("Writing puzzle visualization to SVG file...");

    let input_name = aoc_utils::puzzle_input_filename();
    let output_name = input_name.with_extension("svg");
    let svg = svg::render_polygon(&polygon);

    std::fs::write(output_name, svg).expect("Writing to file failed.");
    println!("Done.");
}
