use std::convert::Infallible;
use std::fmt::{self, Debug};
use std::iter;
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
    #[error("all rows of an input grid must have the same width: expected width {exp}, found {acc}.")]
    RowSize { exp: usize, acc: usize },
}

#[derive(Error, Debug, Clone)]
pub enum TryParseGridError<E> {
    /// An error occurred parsing the input grid itself (i.e., not ).
    #[error(transparent)]
    GridError(#[from] ParseGridError),

    /// An error occurred from within a map function while attempting to parse grid input.
    #[error("map function returned Err while parsing input grid: {0}")]
    MapFnError(E),
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

// [FIXME] There doesn't seem to be a simple (read: not-`unsafe`) way to allocate **exactly** the right amount of bytes;
// for creating new boxed slices. Vec::with_capacity is allowed to allocate more than asked to, if the compiler/OS
// decides to. Might be worth looking into eventually, maybe once `std::alloc::Allocator` gets stabilized and is easier
// to work with.

impl<T: Clone> Grid<T> {
    /// Creates a new grid by filling it with clones of an element.
    pub fn from_elem(w: usize, h: usize, val: T) -> Self {
        let cap = w * h;
        if cap == 0 {
            return Grid { w, h, buf: Box::new([]) };
        }

        let mut buf = Vec::with_capacity(cap);

        // Move first value directly into the buffer without cloning unnecessarily:
        buf.push(val);

        // Then clone that one into the rest of the vector:
        for _ in 1..cap {
            buf.push(buf[0].clone());
        }

        Grid {
            w,
            h,
            buf: buf.into_boxed_slice(),
        }
    }
}

impl<T: Default> Grid<T> {
    /// Creates a new empty grid filled with the default value for `T`.
    pub fn empty(w: usize, h: usize) -> Self {
        let mut buf = Vec::<T>::with_capacity(w * h);
        buf.fill_with(Default::default);
        Grid {
            w,
            h,
            buf: buf.into_boxed_slice(),
        }
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

    /// Creates a new grid of the given size, filled with the result of calling `func` once for every (x, y) position of
    /// the grid.
    pub fn from_fn<F>(w: usize, h: usize, mut func: F) -> Self
    where
        F: FnMut((usize, usize)) -> T,
    {
        let mut buf = Vec::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                buf.push(func((x, y)));
            }
        }

        Grid {
            w,
            h,
            buf: buf.into_boxed_slice(),
        }
    }

    /// Creates a new grid by running each character of the source input through a mapping function.
    ///
    /// The mapping function is passed both the source character and the (x, y) position at which it appears.
    pub fn from_lines_map<I, S, F>(lines: I, mut map_fn: F) -> Result<Self, ParseGridError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        F: FnMut(char, (usize, usize)) -> T,
    {
        match Self::try_from_lines_map::<Infallible, I, S, _>(lines, move |x, p| Ok(map_fn(x, p))) {
            Ok(grid) => Ok(grid),
            Err(TryParseGridError::GridError(err)) => Err(err),
            Err(TryParseGridError::MapFnError(_)) => unreachable!(), // map_fn never returns Err
        }
    }

    /// Creates a new grid by attempting to call the provided
    pub fn try_from_lines_map<E, I, S, F>(lines: I, mut map_fn: F) -> Result<Self, TryParseGridError<E>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        F: FnMut(char, (usize, usize)) -> Result<T, E>,
    {
        let mut lines = lines.into_iter();

        // Grab the first line first before starting the loop so we can eagerly allocate space based on its size.
        let Some(first_line) = lines.next() else {
            return Ok(Grid { w: 0, h: 0, buf: Box::new([]) });
        };

        let w = first_line.as_ref().len();
        let mut buf = Vec::with_capacity(w * w); // Assume square to start with; will shrink to boxed_slice later.
        let mut h = 0;

        let all_lines = iter::once(first_line).chain(lines);
        for line in all_lines {
            let line = line.as_ref();
            if line.len() == w {
                buf.reserve(line.len()); // NB: *not* `reserve_exact`
                for (x, c) in line.chars().enumerate() {
                    let res = map_fn(c, (x, h)).map_err(TryParseGridError::MapFnError)?;
                    buf.push(res);
                }
                h += 1;
            } else {
                return Err(ParseGridError::RowSize { exp: w, acc: line.len() }.into());
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
