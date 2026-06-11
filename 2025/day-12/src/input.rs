use std::cmp;
use std::fmt::{Debug, Display};
use std::str::FromStr;

// MARK: Base impl

type Point = (usize, usize);

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
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn points(&self) -> &[Point] {
        &self.points
    }
}

impl Region {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

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
            return Err("invalid present shape: shape zero size");
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

/// Transformations for [`PresentShape`]s.
impl PresentShape {
    fn transform_points<F: FnMut(Point) -> Point>(&mut self, mut f: F) {
        for point in &mut self.points {
            *point = f(*point)
        }
    }

    fn sort_points(&mut self) {
        self.points.sort_unstable_by(cmp_points);
    }

    pub fn trim_shape(&mut self) {
        // - Find the minimum and maximum x- and y-coordinates.
        // - If the maxima are less than width/height, then width/height can be lowered.
        // - If the minima are greater than 0, all points can be shifted towards zero by that amount.
        let (min_x, max_x, min_y, max_y) = self
            .points
            .iter()
            .copied()
            .fold(None, |acc, (x, y)| match acc {
                None => Some((x, x, y, y)),
                Some((min_x, max_x, min_y, max_y)) => Some((min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))),
            })
            .expect("all shapes should have at least one point");

        self.width = max_x + 1;
        self.height = max_y + 1;
        if min_x > 0 || min_y > 0 {
            self.transform_points(|(x, y)| (x - min_x, y - min_y));
            self.width -= min_x;
            self.height -= min_y;
        }

        // In theory, the above operation should keep things in the correct order...
        // But it can't hurt to make sure!
        self.sort_points();
    }

    pub fn rotate_cw(&self, n: usize) -> Self {
        let mut shape = self.clone();
        for _ in 0..n {
            let height = shape.height;
            shape.transform_points(|(x, y)| (height - y - 1, x));
            std::mem::swap(&mut shape.width, &mut shape.height);
        }
        shape.sort_points();
        shape
    }

    pub fn rotate_ccw(&self, n: usize) -> Self {
        let mut shape = self.clone();
        for _ in 0..n {
            let width = shape.width;
            shape.transform_points(|(x, y)| (y, width - x - 1));
            std::mem::swap(&mut shape.width, &mut shape.height);
        }
        shape.sort_points();
        shape
    }

    pub fn flip_vertical(&self) -> Self {
        let mut shape = self.clone();
        shape.transform_points(|(x, y)| (x, self.height - y - 1));
        shape.sort_points();
        shape
    }

    pub fn flip_horizontal(&self) -> Self {
        let mut shape = self.clone();
        shape.transform_points(|(x, y)| (self.width - x - 1, y));
        shape.sort_points();
        shape
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

    #[allow(unused)]
    pub fn with_point_str(mut self, s: &'a str) -> Self {
        self.point_str = s;
        self
    }

    #[allow(unused)]
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
