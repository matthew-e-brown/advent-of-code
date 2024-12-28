use std::fmt::{self, Debug};
use std::ops::{Index, IndexMut};

use thiserror::Error;

/// A 2D grid providing easy access to indexing operations.
#[derive(Clone)]
pub struct Grid<T> {
    w: usize,
    h: usize,
    buf: Box<[T]>,
}

#[derive(Error, Debug, Clone)]
pub enum ParseGridError {
    #[error("all columns of an input grid must have the same size: expected width {exp}, found {acc}.")]
    ColumnSize { exp: usize, acc: usize },
}

impl Grid<char> {
    pub fn from_lines<I, S>(lines: I) -> Result<Self, ParseGridError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Grid::from_lines_map(lines, |c, _| c)
    }
}

impl<T> Grid<T> {
    pub const fn width(&self) -> usize {
        self.w
    }

    pub const fn height(&self) -> usize {
        self.h
    }

    pub const fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn contains<Idx: GridIndex>(&self, pos: Idx) -> bool {
        let (x, y) = pos.as_tuple();
        x < self.w && y < self.h
    }

    pub fn get<Idx: GridIndex>(&self, pos: Idx) -> Option<&T> {
        let (x, y) = pos.as_tuple();
        if self.contains((x, y)) {
            // SAFETY: Just checked bounds.
            Some(unsafe { self.get_unchecked((x, y)) })
        } else {
            None
        }
    }

    pub fn get_mut<Idx: GridIndex>(&mut self, pos: Idx) -> Option<&mut T> {
        let (x, y) = pos.as_tuple();
        if self.contains((x, y)) {
            // SAFETY: Just checked bounds.
            Some(unsafe { self.get_unchecked_mut((x, y)) })
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked<Idx: GridIndex>(&self, pos: Idx) -> &T {
        let idx = pos.index1d(self.width());
        self.buf.get_unchecked(idx)
    }

    pub unsafe fn get_unchecked_mut<Idx: GridIndex>(&mut self, pos: Idx) -> &mut T {
        let idx = pos.index1d(self.width());
        self.buf.get_unchecked_mut(idx)
    }

    /// Creates a new grid of characters, running each one through a mapping function first.
    ///
    /// The mapping function is passed both the source character and the (x, y) position at which it appears.
    pub fn from_lines_map<I, S, F>(lines: I, mut map_fn: F) -> Result<Self, ParseGridError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        F: FnMut(char, (usize, usize)) -> T,
    {
        let mut lines = lines.into_iter();
        let Some(first_line) = lines.next() else {
            return Ok(Grid { w: 0, h: 0, buf: Box::new([]) });
        };

        let first_line = first_line.as_ref();
        let w = first_line.len();
        let mut buf = Vec::with_capacity(w * w); // Assume square to start with; will shrink to boxed_slice later.

        buf.extend(first_line.chars().enumerate().map(|(x, c)| map_fn(c, (x, 0))));
        let mut h = 1;

        for line in lines {
            let line = line.as_ref();
            if line.len() == w {
                buf.extend(line.chars().enumerate().map(|(x, c)| map_fn(c, (x, h))));
                h += 1;
            } else {
                return Err(ParseGridError::ColumnSize { exp: w, acc: line.len() });
            }
        }

        let buf = buf.into_boxed_slice();
        Ok(Grid { w, h, buf })
    }
}

impl<T, I: GridIndex> Index<I> for Grid<T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let i = index.index1d(self.width());
        &self.buf[i]
    }
}

impl<T, I: GridIndex> IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let i = index.index1d(self.width());
        &mut self.buf[i]
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Grid:")?;
        for y in 0..self.h {
            for x in 0..self.w {
                write!(f, "{:?}", &self[[x, y]])?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Size: {}×{}", self.w, self.h)?;
        Ok(())
    }
}

/// A trait representing objects that can be used to index a [two-dimensional grid][Grid].
pub trait GridIndex {
    /// Gets the `x`-component of this [GridIndex].
    fn x(&self) -> usize;

    /// Gets the `y`-component of this [GridIndex].
    fn y(&self) -> usize;

    /// Normalizes this index as a tuple to make it easier to destructure the `x` and `y` components.
    fn as_tuple(&self) -> (usize, usize) {
        (self.x(), self.y())
    }

    /// Given a grid-width, converts this [GridIndex] into a single one-dimensional buffer offset.
    fn index1d(&self, w: usize) -> usize {
        self.y() * w + self.x()
    }
}

#[rustfmt::skip]
impl GridIndex for (usize, usize) {
    fn x(&self) -> usize { self.0 }
    fn y(&self) -> usize { self.1 }
}

#[rustfmt::skip]
impl GridIndex for &(usize, usize) {
    fn x(&self) -> usize { self.0 }
    fn y(&self) -> usize { self.1 }
}

#[rustfmt::skip]
impl GridIndex for (&usize, &usize) {
    fn x(&self) -> usize { *self.0 }
    fn y(&self) -> usize { *self.1 }
}

#[rustfmt::skip]
impl GridIndex for [usize; 2] {
    fn x(&self) -> usize { self[0] }
    fn y(&self) -> usize { self[1] }
}

#[rustfmt::skip]
impl GridIndex for &[usize; 2] {
    fn x(&self) -> usize { self[0] }
    fn y(&self) -> usize { self[1] }
}

#[rustfmt::skip]
impl GridIndex for [&usize; 2] {
    fn x(&self) -> usize { *self[0] }
    fn y(&self) -> usize { *self[1] }
}
