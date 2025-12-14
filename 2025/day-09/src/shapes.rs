use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub l: u32,
    pub r: u32,
    pub t: u32,
    pub b: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polygon {
    /// The points that make up this polygon.
    ///
    /// The first point is repeated at the end of the list to make this a closed polygon.
    points: Vec<Point>,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = s.split(',');

        let x = bits.next().ok_or("Point is missing x coordinate")?;
        let y = bits.next().ok_or("Point is missing y coordinate")?;
        if bits.next().is_some() {
            return Err("Point contains more than 2 coordinates");
        }

        let x = x.parse().map_err(|_| "Point x is not a valid number")?;
        let y = y.parse().map_err(|_| "Point y is not a valid number")?;
        Ok(Point { x, y })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("{},{}", self.x, self.y);
        f.pad(&str)
    }
}

impl Line {
    /// Creates a new line from start and end [points][Point].
    pub const fn new(a: Point, b: Point) -> Line {
        Line { a, b }
    }

    /// Checks if this line is horizontal.
    pub const fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    /// Checks if this line is vertical.
    pub const fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line({} to {})", self.a, self.b)
    }
}

impl Rectangle {
    /// Creates a new rectangle from two corner [points][Point].
    pub const fn new(p1: Point, p2: Point) -> Rectangle {
        let Point { x: x1, y: y1 } = p1;
        let Point { x: x2, y: y2 } = p2;
        let [l, r] = if x1 <= x2 { [x1, x2] } else { [x2, x1] };
        let [t, b] = if y1 <= y2 { [y1, y2] } else { [y2, y1] };
        Rectangle { l, r, t, b }
    }

    pub const fn width(&self) -> u32 {
        self.r - self.l + 1
    }

    pub const fn height(&self) -> u32 {
        self.b - self.t + 1
    }

    pub const fn area(&self) -> u64 {
        (self.width() as u64) * (self.height() as u64)
    }
}

impl Polygon {
    /// Returns the total number of points in this polygon.
    pub fn num_points(&self) -> usize {
        // Without the repeated first point at the end
        self.points.len() - 1
    }

    /// Reads a series of *x,y* points and verifies that they are in the correct order to form a polygon.
    pub fn from_points(iter: impl IntoIterator<Item = Point>) -> Polygon {
        let mut iter = iter.into_iter();

        // Parse the first before starting the loop:
        let p0 = iter.next().expect("Polygon should contain at least 3 points");
        let mut points = vec![p0];
        let mut i = 0;
        while let Some(point) = iter.next() {
            let prev = points[i];

            if point.x != prev.x && point.y != prev.y {
                panic!("Polygon can only be formed from 90° angles");
            }

            points.push(point);
            i += 1;
        }

        // Close the polygon
        points.push(p0);

        assert!(points.len() >= 3, "Polygon should contain at least 3 points");
        Polygon { points }
    }

    /// Gets a list of all the points in this polygon.
    pub fn points(&self) -> &[Point] {
        &self.points[..self.num_points()]
    }

    /// Returns an iterator of all of the line segments that form this (closed) polygon.
    pub fn edges(&self) -> impl Iterator<Item = Line> {
        // NB: *not* `self.points()` -- we want the repeated final point so the iterator closes the polygon.
        self.points.windows(2).map(|pts| Line::new(pts[0], pts[1]))
    }
}
