use std::cmp;
use std::fmt::{Debug, Display};
use std::str::FromStr;

// MARK: Base impl

pub type Point = (usize, usize);

#[derive(Clone)]
pub struct PresentShape {
    width: usize,
    height: usize,
    points: Box<[Point]>,
}

#[derive(Debug, Clone)]
pub struct Region {
    width: usize,
    height: usize,
    counts: Box<[usize]>,
}

impl PresentShape {
    /// Returns the width of this shape's bounding box.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of this shape's bounding box.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a reference to the list of points filled by this shape.
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// Returns the total number of points/tiles that are filled by this shape.
    pub fn surface_area(&self) -> usize {
        self.points().len()
    }

    /// Creates a new shape out of a list of points.
    pub fn from_points(points: impl IntoIterator<Item = Point>) -> Self {
        let mut maxes = None;
        let mut points: Box<[Point]> = points
            .into_iter()
            .inspect(|&(x, y)| {
                maxes = maxes
                    .map(|(max_x, max_y)| (cmp::max(max_x, x), cmp::max(max_y, y)))
                    .or(Some((x, y)));
            })
            .collect();
        if let Some((max_x, max_y)) = maxes {
            points.sort_unstable_by(cmp_points);
            let width = max_x + 1;
            let height = max_y + 1;
            Self { width, height, points }
        } else {
            Self { width: 0, height: 0, points }
        }
    }
}

impl Region {
    /// Returns the width of this region.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of this region.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the required amount of [`PresentShape`]
    pub fn counts(&self) -> &[usize] {
        &self.counts
    }
}

// MARK: Parsing

/// Parses the complete puzzle input file into a list of present shapes and regions.
pub fn parse_input(input: &str) -> Result<(Vec<PresentShape>, Vec<Region>), &'static str> {
    let mut lines = input.lines();
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    while let Some(line) = lines.next() {
        // Does this line look like `/^\d+:$/`?
        match line.strip_suffix(':').and_then(|n| n.parse::<usize>().ok()) {
            Some(index) => {
                // If so, double check that the indices provided actually appear in order.
                if index != shapes.len() {
                    return Err("invalid puzzle input: present shape indices are out of order");
                }

                // For the puzzle shape itself, we collect a substring that starts at the beginning of the next line and
                // ends at the first double-blank we see.
                let mut lines = lines.by_ref().take_while(|line| !line.trim().is_empty());
                let first_line = lines.next().ok_or("invalid puzzle input: expected present shape after index")?;
                let last_line = lines.last().unwrap_or(first_line);

                // Rust's `Lines` iterator doesn't really have a good way to "merge" the lines it spits out (which makes
                // sense, since they're discontinuous due to the fact that they exclude line terminators). To accomplish
                // this, we get a pointer to the start of the first line and the end of the last line, then grab the
                // subslice between them.
                let input_range = input.as_bytes().as_ptr_range();
                let ptr1 = first_line.as_bytes().as_ptr_range().start;
                let ptr2 = last_line.as_bytes().as_ptr_range().end;
                assert!(input_range.contains(&ptr1) || ptr1 == input_range.end);
                assert!(input_range.contains(&ptr2) || ptr2 == input_range.end);

                // SAFETY: just checked that ptr1 and ptr2 were within the bounds of `input_range`.
                // Note: This could be a bit more efficient with std::slice::from_ptr_range, but it's still unstable.
                let i = unsafe { ptr1.byte_offset_from_unsigned(input_range.start) };
                let j = unsafe { ptr2.byte_offset_from_unsigned(input_range.start) };

                let shape = PresentShape::from_str(&input[i..j])?;
                shapes.push(shape);
            },
            // If this line didn't look like `/^\d+:$/`, then either it's empty or it must be a region.
            None if line.trim().is_empty() => continue,
            None => {
                let region = Region::from_str(line)?;
                regions.push(region);
            },
        }
    }

    Ok((shapes, regions))
}

impl FromStr for PresentShape {
    type Err = &'static str;

    /// Converts a string into a [`PresentShape`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Should see a square grid of '#'/'.' characters.
        let mut points = Vec::new();
        let mut width = None;
        let mut height = 0;
        for line in s.lines() {
            // Is this line the same width as the last one?
            let lw = line.len();
            match width {
                Some(w) if w != lw => return Err("invalid present shape: shape is not square"),
                Some(_) => {},
                None => width = Some(lw),
            }

            for (x, c) in line.as_bytes().into_iter().enumerate() {
                match c {
                    b'#' => points.push((x, height)),
                    b'.' => {},
                    _ => return Err("invalid present shape: found invalid character"),
                }
            }

            height += 1;
        }

        let width = width.unwrap_or(0);

        if width == 0 || height == 0 {
            return Err("invalid present shape: shape has zero size");
        } else if points.len() == 0 {
            return Err("invalid present shape: shape is empty");
        }

        points.sort_unstable_by(cmp_points);
        Ok(Self {
            width,
            height,
            points: points.into_boxed_slice(),
        })
    }
}

impl FromStr for Region {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, counts) = s.split_once(':').ok_or("invalid region: expected ':'")?;

        let (width, height) = size
            .split_once('x')
            .ok_or("invalid region: expected 'x' between width and height")?;
        let width = width.parse().or(Err("invalid region: width is not a valid integer"))?;
        let height = height.parse().or(Err("invalid region: height is not a valid integer"))?;

        let counts = counts
            .trim_start()
            .split_whitespace()
            .map(|n| n.parse::<usize>().or(Err("invalid region: shape index is not a valid integer")))
            .collect::<Result<Box<[_]>, _>>()?;

        Ok(Self { width, height, counts })
    }
}

// MARK: Transforms

/// Represent all possible orientations/transformations that a [`PresentShape`] may be in.
///
/// Corresponds to the eight different members of the _dihedral group of order 8_.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Transform {
    /// No transformation.
    #[default]
    Identity,
    /// A 90° clockwise rotation.
    RotateCW,
    /// A 180° rotation.
    Rotate180,
    /// A 90° counterclockwise rotation. Equivalent to a 270° clockwise rotation.
    RotateCCW,
    /// A reflection along the vertical axis (which moves points horizontally, from left to right).
    ReflectV,
    /// A reflection along the line between the top-right and bottom-left corners.
    ///
    /// Equivalent to a horizontal reflection followed by a 90° clockwise rotation.
    ReflectNE,
    /// A reflection along the horizontal axis (which moves points vertically, from top to bottom).
    ///
    /// Equivalent to a horizontal reflection followed by a 180° clockwise rotation.
    ReflectH,
    /// A reflection along the line between the top-left and bottom-right corners.
    ///
    /// Equivalent to a horizontal reflection followed by a 270° clockwise rotation.
    ReflectSE,
}

impl Transform {
    /// An array containing all variants of this enum.
    pub const VARIANTS: [Self; 8] = [
        Transform::Identity,
        Transform::RotateCW,
        Transform::Rotate180,
        Transform::RotateCCW,
        Transform::ReflectV,
        Transform::ReflectNE,
        Transform::ReflectH,
        Transform::ReflectSE,
    ];
}

/// Transformations for [`PresentShape`]s.
impl PresentShape {
    /// Creates a new [`PresentShape`] by applying a [`Transform`] to this one.
    pub fn with_transform(&self, transform: Transform) -> Self {
        let mut shape = self.clone();
        shape.transform(transform);
        shape
    }

    /// Applies a [`Transform`] to this shape.
    pub fn transform(&mut self, transform: Transform) {
        let w = self.width;
        let h = self.height;
        match transform {
            Transform::Identity => {},
            Transform::RotateCW => self.do_transform(|(x, y)| (h - y - 1, x)),
            Transform::Rotate180 => self.do_transform(|(x, y)| (w - x - 1, h - y - 1)),
            Transform::RotateCCW => self.do_transform(|(x, y)| (y, w - x - 1)),
            Transform::ReflectV => self.do_transform(|(x, y)| (w - x - 1, y)),
            Transform::ReflectNE => self.do_transform(|(x, y)| (h - y - 1, w - x - 1)),
            Transform::ReflectH => self.do_transform(|(x, y)| (x, h - y - 1)),
            Transform::ReflectSE => self.do_transform(|(x, y)| (y, x)),
        }
    }

    /// Accepts a function that converts one `(x, y)` point into another and uses it to actually modify this present's
    /// shape.
    fn do_transform<F: FnMut(Point) -> Point>(&mut self, mut f: F) {
        // Keep track of where all the points end up as we transform them so we can keep the width and height consistent
        // as we go.
        let mut maxes = None;
        for point in &mut self.points {
            let (new_x, new_y) = f(*point);
            maxes = maxes
                .map(|(max_x, max_y)| (cmp::max(max_x, new_x), cmp::max(max_y, new_y)))
                .or(Some((new_x, new_y)));
            *point = (new_x, new_y);
        }

        // In theory, it's possible that the shape has zero points. Our `FromStr` impl doesn't allow it, but our
        // `from_points` does.
        if let Some((max_x, max_y)) = maxes {
            self.width = max_x + 1;
            self.height = max_y + 1;
            // Also take the chance to make sure our points stay sorted.
            self.points.sort_unstable_by(cmp_points);
        } else {
            self.width = 0;
            self.height = 0;
        }
    }
}

/// Compares two [`Point`]s for sorting.
///
/// Comparison is done first by `y`-coordinate, then by `x`-coordinate. This order is useful because it means that
/// points will appear in the correct order when looping through the `x`/`y` positions of a shape.
fn cmp_points(&(x1, y1): &Point, &(x2, y2): &Point) -> cmp::Ordering {
    y1.cmp(&y2).then(x1.cmp(&x2))
}

// MARK: Printing

impl PresentShape {
    fn print(&self) -> ShapePrinter<'_> {
        ShapePrinter::new(self)
    }
}

struct ShapePrinter<'a> {
    shape: &'a PresentShape,
    point_str: &'a str,
    blank_str: &'a str,
    between_rows: Option<&'a str>,
}

impl<'a> ShapePrinter<'a> {
    fn new(shape: &'a PresentShape) -> Self {
        Self {
            shape,
            point_str: "#",
            blank_str: ".",
            between_rows: None,
        }
    }

    #[expect(unused)]
    pub fn with_point_str(mut self, s: &'a str) -> Self {
        self.point_str = s;
        self
    }

    #[expect(unused)]
    pub fn with_blank_str(mut self, s: &'a str) -> Self {
        self.blank_str = s;
        self
    }

    pub fn with_str_between_rows(mut self, s: &'a str) -> Self {
        self.between_rows = Some(s);
        self
    }
}

impl<'a> Display for ShapePrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Note: this only works because the points are sorted in y->x order.s
        let mut points = self.shape.points().into_iter().copied();
        let mut next_point = points.next();

        for y in 0..self.shape.height() {
            for x in 0..self.shape.width() {
                match next_point {
                    Some(pt) if pt == (x, y) => {
                        f.write_str(self.point_str)?;
                        next_point = points.next();
                    },
                    _ => f.write_str(self.blank_str)?,
                }
            }

            if let Some(s) = self.between_rows
                && y < self.shape.height() - 1
            {
                f.write_str(s)?;
            }
        }

        Ok(())
    }
}

impl Display for PresentShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.print().with_str_between_rows("\n");
        f.write_fmt(format_args!("{str}"))
    }
}

impl Debug for PresentShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // Alternate = print multi-line.
            // 3x3 shape, with added tabs and newlines = +2 for every row:
            // ```
            // \t##.
            // \t##.
            // \t#..
            // ```
            let str = self.print().with_str_between_rows("\n\t");
            f.write_fmt(format_args!("PresentShape(\n\t{str}\n)"))
        } else {
            // Non-alternate: print all in one line. `PresentShape([##.|##.|#..])`.
            let str = self.print().with_str_between_rows("|");
            f.debug_tuple("PresentShape").field(&format_args!("[{str}]")).finish()
        }
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that shapes can be read to/from string format accurately and losslessly.
    mod inout {
        use super::*;

        struct TestCase {
            source: &'static str,
            expected_width: usize,
            expected_height: usize,
            expected_points: &'static [Point],
        }

        impl TestCase {
            const fn new(source: &'static str, w: usize, h: usize, points: &'static [Point]) -> Self {
                TestCase {
                    source,
                    expected_width: w,
                    expected_height: h,
                    expected_points: points,
                }
            }
        }

        /// Shapes from the day 12 example problem.
        ///
        /// ```txt
        /// 0:     1:     2:     3:     4:     5:
        /// ###    ###    .##    ##.    ###    ###
        /// ##.    ##.    ###    ###    #..    .#.
        /// ##.    .##    ##.    ##.    ###    ###
        /// ```
        #[rustfmt::skip]
        const CASES: &[TestCase] = &[
            TestCase::new("###\n##.\n##.", 3, 3, &[
                (0,0), (1,0), (2,0),
                (0,1), (1,1),
                (0,2), (1,2),
            ]),
            TestCase::new("###\n##.\n.##", 3, 3, &[
                (0,0), (1,0), (2,0),
                (0,1), (1,1),
                       (1,2), (2,2),
            ]),
            TestCase::new(".##\n###\n##.", 3, 3, &[
                       (1,0), (2,0),
                (0,1), (1,1), (2,1),
                (0,2), (1,2),
            ]),
            TestCase::new("##.\n###\n##.", 3, 3, &[
                (0,0), (1,0),
                (0,1), (1,1), (2,1),
                (0,2), (1,2),
            ]),
            TestCase::new("###\n#..\n###", 3, 3, &[
                (0,0), (1,0), (2,0),
                (0,1),
                (0,2), (1,2), (2,2),
            ]),
            TestCase::new("###\n.#.\n###", 3, 3, &[
                (0,0), (1,0), (2,0),
                       (1,1),
                (0,2), (1,2), (2,2),
            ]),
        ];

        #[test]
        fn parse() {
            for case in CASES {
                let shape = PresentShape::from_str(case.source).unwrap();
                assert!(shape.width() == case.expected_width, "shape width parsed incorrectly");
                assert!(shape.height() == case.expected_height, "shape height parsed incorrectly");
                assert!(shape.points() == case.expected_points, "shape points-list parsed incorrectly");
            }
        }

        #[test]
        fn print() {
            #[rustfmt::skip]
            let shape = PresentShape::from_points([

            ]);

            todo!();
        }

        #[test]
        fn round_trip() {
            for case in CASES {
                let shape = PresentShape::from_str(case.source).unwrap();
                let string = shape.to_string();
                assert!(case.source == string);
            }
        }
    }

    /// Tests that [transformations][Transform] work correctly.
    ///
    /// Since [`inout`] verifies that parsing and printing work correctly, we can safely use `to_`/`from_str` to test
    /// other parts.
    mod transforms {
        use super::*;

        // [TODO] More tests!
    }
}
