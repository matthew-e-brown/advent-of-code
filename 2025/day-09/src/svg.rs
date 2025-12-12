use std::fmt::Write;

use crate::shapes::{Point, Polygon};

const SVG_DOCTYPE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
"#;

const SVG_STYLES: &str = r#"<style>rect { fill: #cf1313; } polygon { fill: #396326; stroke: #396326; }</style>"#;

/// Renders a [`Polygon`] as an SVG image.
pub fn render_polygon(polygon: &Polygon) -> String {
    let points = polygon.points();

    // Pick some coordinate to be our initial view box corners
    let mut min_x = None;
    let mut max_x = None;
    let mut min_y = None;
    let mut max_y = None;

    let mut polygon = String::from(r##"<polygon stroke-width="1" points=""##);
    let mut p_group = String::from(r##"<g>"##);

    p_group += "\n";
    for &point in points {
        let Point { x, y } = point;

        min_x = if min_x.is_none_or(|m| x < m) { Some(x) } else { min_x };
        max_x = if max_x.is_none_or(|m| x > m) { Some(x) } else { max_x };
        min_y = if min_y.is_none_or(|m| y < m) { Some(y) } else { min_y };
        max_y = if max_y.is_none_or(|m| y > m) { Some(y) } else { max_y };

        write!(&mut polygon, "{},{} ", x, y).unwrap();
        write!(&mut p_group, r##"  <rect width="1" height="1" "##).unwrap();
        // This is kinda messy, but basically: w
        #[rustfmt::skip]
        {
            if x == 0 { write!(&mut p_group, "x=\"-0.5\" ") } else { write!(&mut p_group, "x=\"{}.5\" ", x - 1) }.unwrap();
            if y == 0 { write!(&mut p_group, "y=\"-0.5\" ") } else { write!(&mut p_group, "y=\"{}.5\" ", y - 1) }.unwrap();
        };
        writeln!(&mut p_group, "/>").unwrap();
    }
    polygon.pop(); // Remove extra space before closing quote
    polygon += "\" />";
    p_group += "</g>";

    let mut result = String::from(SVG_DOCTYPE);
    write!(&mut result, r##"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox=""##).unwrap();

    // viewBox:
    let max_x = max_x.unwrap_or(100) as i64;
    let max_y = max_y.unwrap_or(100) as i64;
    let mut min_x = min_x.unwrap_or(0) as i64;
    let mut min_y = min_y.unwrap_or(0) as i64;
    let mut w = max_x - min_x;
    let mut h = max_y - min_y;

    // Extend the dimensions of the viewBox by 10% on either side (and by at least two for small puzzles):
    let dx = ((w as f64 * 0.10) as i64).max(2);
    let dy = ((h as f64 * 0.10) as i64).max(2);

    w += dx * 2;
    h += dy * 2;
    min_x -= dx;
    min_y -= dy;

    writeln!(&mut result, "{min_x} {min_y} {w} {h}\">").unwrap();

    writeln!(&mut result, "{SVG_STYLES}").unwrap();
    writeln!(&mut result, "{polygon}").unwrap();
    writeln!(&mut result, "{p_group}").unwrap();
    writeln!(&mut result, "</svg>").unwrap();

    result
}
