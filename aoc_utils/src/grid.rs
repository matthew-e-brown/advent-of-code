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

impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [x, y] = index;
        let idx = y * self.w + x;
        &self.buf[idx]
    }
}

impl<T> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [x, y] = index;
        let idx = y * self.w + x;
        &mut self.buf[idx]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        let idx = y * self.w + x;
        &self.buf[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let idx = y * self.w + x;
        &mut self.buf[idx]
    }
}
