use std::ops::{Index, IndexMut};

use thiserror::Error;

/// A 2D grid of characters, for providing easy access to indexing operations.
#[derive(Clone)]
pub struct CharGrid {
    w: usize,
    h: usize,
    buf: Box<[char]>,
}

#[derive(Error, Debug, Clone)]
pub enum ParseGridError {
    #[error("all columns of an input grid must have the same size: expected width {exp}, found {acc}.")]
    ColumnSize { exp: usize, acc: usize },
}

impl CharGrid {
    pub const fn width(&self) -> usize {
        self.w
    }

    pub const fn height(&self) -> usize {
        self.h
    }

    pub const fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn from_lines<I, S>(lines: I) -> Result<Self, ParseGridError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut lines = lines.into_iter();
        let Some(first_line) = lines.next() else {
            return Ok(CharGrid { w: 0, h: 0, buf: Box::new([]) });
        };

        let first_line = first_line.as_ref();
        let w = first_line.len();
        let mut buf = Vec::with_capacity(w * w); // Assume square to start with; will shrink to boxed_slice later.

        buf.extend(first_line.chars());
        let mut h = 1;

        for line in lines {
            let line = line.as_ref();
            if line.len() == w {
                buf.extend(line.chars());
                h += 1;
            } else {
                return Err(ParseGridError::ColumnSize { exp: w, acc: line.len() });
            }
        }

        let buf = buf.into_boxed_slice();
        Ok(CharGrid { w, h, buf })
    }
}

impl Index<[usize; 2]> for CharGrid {
    type Output = char;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [x, y] = index;
        let idx = y * self.w + x;
        &self.buf[idx]
    }
}

impl IndexMut<[usize; 2]> for CharGrid {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [x, y] = index;
        let idx = y * self.w + x;
        &mut self.buf[idx]
    }
}

impl Index<(usize, usize)> for CharGrid {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        let idx = y * self.w + x;
        &self.buf[idx]
    }
}

impl IndexMut<(usize, usize)> for CharGrid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let idx = y * self.w + x;
        &mut self.buf[idx]
    }
}
