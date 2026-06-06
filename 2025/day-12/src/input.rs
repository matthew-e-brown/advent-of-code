use std::error::Error;

use aoc_utils::grid::Grid;

#[derive(Debug, Clone)]
pub struct PresentShape {
    grid: Grid<bool>,
    area: usize,
}

#[derive(Debug, Clone)]
pub struct Region {
    size: (usize, usize),
    shapes: Vec<usize>,
}

pub fn parse_input(input: &str) -> Result<(Vec<PresentShape>, Vec<Region>), Box<dyn Error>> {
    // Loop until we see the first thing that isn't a valid present shape; then break and move on to the regions.
    let mut lines = input.lines();
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    while let Some(line) = lines.next() {
        // Does this line look like `/\d+:$/`?
        match line.strip_suffix(':').and_then(|n| n.parse::<usize>().ok()) {
            // If so, is this index the right one, or are we trying to make shapes out of order?
            Some(i) if i != shapes.len() => {
                return Err("invalid present shape index: indices must be contiguous".into());
            },
            Some(_) => {
                // Grab all lines up to the next empty one and try to parse them as a present shape.
                let lines = lines.by_ref().take_while(|line| !line.trim().is_empty());
                let shape = PresentShape::from_lines(lines)?;
                shapes.push(shape);
            },
            // If not, then this has to be a region.
            None => {
                regions.push(Region::from_line(line)?);
            },
        }
    }

    if shapes.len() == 0 {
        return Err("puzzle input contains no shapes".into());
    }

    if shapes.len() == 0 {
        return Err("puzzle input contains no regions".into());
    }

    Ok((shapes, regions))
}

impl PresentShape {
    /// Returns the total surface area of this present shape.
    pub fn area(&self) -> usize {
        self.area
    }

    /// Returns the width of this present shape's bounding box.
    pub fn width(&self) -> usize {
        self.grid.width()
    }

    /// Returns the height of this present shape's bounding box.
    pub fn height(&self) -> usize {
        self.grid.height()
    }

    /// Returns a reference to a grid representing this present shape's layout.
    pub fn layout(&self) -> &Grid<bool> {
        &self.grid
    }

    fn from_lines<I, S>(lines: I) -> Result<PresentShape, Box<dyn Error>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut area = 0;
        let grid = Grid::try_from_lines_map(lines, |c, _| match c {
            '.' => Ok(false),
            '#' => {
                area += 1;
                Ok(true)
            },
            _ => Err("present layout contains invalid character"),
        })?;

        if grid.width() == 0 {
            return Err("present layout is empty".into());
        }

        Ok(PresentShape { grid, area })
    }

    /// Gets this present shape's bounding box, described by two tuples: an (x, y) for the top-left corner, and a (w, h)
    /// for the size.
    pub fn bounding_box(&self) -> ((usize, usize), (usize, usize)) {
        let (mut xmin, mut xmax) = (None, None);
        let (mut ymin, mut ymax) = (None, None);

        for (x, y) in self.grid.positions() {
            if self.grid[(x, y)] {
                xmin = if xmin.is_none_or(|min| x < min) { Some(x) } else { xmin };
                xmax = if xmax.is_none_or(|max| x > max) { Some(x) } else { xmax };
                ymin = if ymin.is_none_or(|min| y < min) { Some(y) } else { ymin };
                ymax = if ymax.is_none_or(|max| y > max) { Some(y) } else { ymax };
            }
        }

        let xmin = xmin.unwrap();
        let xmax = xmax.unwrap();
        let ymin = ymin.unwrap();
        let ymax = ymax.unwrap();

        let xy = (xmin, ymin);
        let wh = (xmax - xmin, ymax - ymin);
        (xy, wh)
    }

    /// Trims this present shape's size down to be as small as possible.
    pub fn trim_edges(&mut self) {
        // Determine a new (x, y, w, h) for the grid, then copy everything over and drop the old grid.
        let ((xmin, ymin), (w, h)) = self.bounding_box();
        if w < self.width() || h < self.height() {
            let new_grid = Grid::from_fn(w, h, |(x, y)| self.grid[(xmin + x, ymin + y)]);
            self.grid = new_grid;
        }
    }
}

impl Region {
    /// Returns this region's width and height as a tuple.
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    /// Returns this region's width.
    pub fn width(&self) -> usize {
        self.size.0
    }

    /// Returns this region's height.
    pub fn height(&self) -> usize {
        self.size.1
    }

    /// Returns the indices of the present shapes required to fit in this region.
    pub fn shapes(&self) -> &[usize] {
        &self.shapes[..]
    }

    fn from_line(s: &str) -> Result<Self, &'static str> {
        let (size, indices) = s.split_once(':').ok_or("invalid region: expected ':'")?;

        let (w, h) = size
            .split_once('x')
            .ok_or("invalid region: expected 'x' between width and height")?;
        let w = w.parse().map_err(|_| "invalid region: width is not a valid integer")?;
        let h = h.parse().map_err(|_| "invalid region: height is not a valid integer")?;

        let indices = indices
            .trim_start()
            .split_whitespace()
            .map(|n| {
                n.parse::<usize>()
                    .map_err(|_| "invalid region: shape index is not a valid integer")
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { size: (w, h), shapes: indices })
    }
}
