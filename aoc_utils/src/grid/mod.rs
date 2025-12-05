//! Data structures and types for solving grid-based puzzles.
//!
//! Two-dimensional grids are very common in Advent of Code puzzles. Having a [single common implementation][Grid] for
//! all puzzles greatly reduces code repetition. By having the implementation here, the fun programming challenge of
//! implementing the data structure remains, but the puzzles themselves can stay focused on the actual logic.

// [TODO] Add support for signed indexes to make it easier to handle falling out of bounds in a grid.
// [TODO] Add proper tests.

pub mod directions;
pub mod iter;
pub mod neighbours;

use std::convert::Infallible;
use std::fmt::{self, Debug, Write};
use std::ops::{Index, IndexMut};

use thiserror::Error;

pub use self::directions::{Dir4, Dir8, Direction};
use self::iter::{Entries, EntriesMut, Positions, Values, ValuesMut};
pub use self::neighbours::Neighbours;

/// A 2D position used to index a [Grid].
pub type Pos = (usize, usize);

/// A 2D grid providing easy access to indexing operations.
#[derive(Clone)]
pub struct Grid<T> {
    w: usize,
    h: usize,
    buf: Box<[T]>,
}

/// A trait representing objects that can be used to index a [two-dimensional grid][Grid].
///
/// Most commonly, a [`(usize, usize)` tuple][Pos] is used for grid indexing; this trait allows for arbitrary types to
/// be used instead.
pub trait GridIndex: Copy {
    /// Gets the `x`-component of this [GridIndex].
    fn x(&self) -> usize;

    /// Gets the `y`-component of this [GridIndex].
    fn y(&self) -> usize;

    /// Creates a new instance of this [GridIndex] from `x`- and `y`-components.
    fn from_xy(x: usize, y: usize) -> Self;

    /// Normalizes this index as a tuple to make it easier to destructure the `x` and `y` components.
    fn to_tuple(self) -> (usize, usize) {
        (self.x(), self.y())
    }
}

/// Given the width of a [Grid], converts a two-dimensional [GridIndex] into a one-dimensional buffer offset.
#[inline]
fn index1d<Idx: GridIndex>(pos: Idx, w: usize) -> usize {
    pos.y() * w + pos.x()
}

#[rustfmt::skip]
impl GridIndex for (usize, usize) {
    fn x(&self) -> usize { self.0 }
    fn y(&self) -> usize { self.1 }
    fn from_xy(x: usize, y: usize) -> Self { (x, y) }
}

#[rustfmt::skip]
impl GridIndex for [usize; 2] {
    fn x(&self) -> usize { self[0] }
    fn y(&self) -> usize { self[1] }
    fn from_xy(x: usize, y: usize) -> Self { [x, y] }
}

#[derive(Error, Debug, Clone)]
pub enum ParseGridError {
    /// The input being parsed into a grid was not square.
    #[error("all rows of grid input must have the same width: line {0} is a different length")]
    RowSize(usize),
}

#[derive(Error, Debug, Clone)]
pub enum TryParseGridError<E> {
    /// The input being parsed into a grid was not square.
    #[error("all rows of grid input must have the same width: line {0} is a different length")]
    RowSize(usize),

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

// [FIXME] There doesn't seem to be a simple (read: non-unsafe) way to allocate **exactly** the right amount of bytes
// for creating new boxed slices. Vec::with_capacity is allowed to allocate extra space if the compiler/OS decides to,
// which would result in a reallocation when converting to a boxed slice. Might be worth looking into eventually, maybe
// once `std::alloc::Allocator` gets stabilized and is easier to work with.

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
        buf.resize_with(w * h, Default::default);
        Grid {
            w,
            h,
            buf: buf.into_boxed_slice(),
        }
    }
}

impl<T> Grid<T> {
    /// Returns the width of this grid.
    pub const fn width(&self) -> usize {
        self.w
    }

    /// Returns the height of this grid.
    pub const fn height(&self) -> usize {
        self.h
    }

    /// Returns both the width and height of this grid, as a tuple.
    pub const fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    /// Returns an iterator over all (x, y) positions in this grid.
    pub fn positions(&self) -> Positions<Pos> {
        Positions::new(self.size())
    }

    /// Returns an iterator of references to the values in this grid's cells.
    pub fn values(&self) -> Values<'_, T> {
        Values::new(self)
    }

    /// Returns an iterator of mutable references to the values in this grid's cells.
    pub fn values_mut(&mut self) -> ValuesMut<'_, T> {
        ValuesMut::new(self)
    }

    /// Returns an iterator that yields references to this grid's values alongside their (x, y) positions in the grid.
    pub fn entries(&self) -> Entries<'_, T> {
        Entries::new(self)
    }

    /// Returns an iterator that yields mutable references to this grid's values alongside their (x, y) positions in the
    /// grid.
    pub fn entries_mut(&mut self) -> EntriesMut<'_, T> {
        EntriesMut::new(self)
    }

    /// Checks whether or not the given position is within the bounds of this grid's size.
    pub fn contains<Idx: GridIndex>(&self, pos: Idx) -> bool {
        pos.x() < self.w && pos.y() < self.h
    }

    /// Gets a reference to the item at the given position in this grid. Returns `None` if `pos` is out of bounds.
    pub fn get<Idx: GridIndex>(&self, pos: Idx) -> Option<&T> {
        if self.contains(pos) {
            // SAFETY: Just checked bounds.
            Some(unsafe { self.get_unchecked(pos) })
        } else {
            None
        }
    }

    /// Gets a mutable reference to the item at the given position in this grid. Returns `None` if `pos` is out of
    /// bounds.
    pub fn get_mut<Idx: GridIndex>(&mut self, pos: Idx) -> Option<&mut T> {
        if self.contains(pos) {
            // SAFETY: Just checked bounds.
            Some(unsafe { self.get_unchecked_mut(pos) })
        } else {
            None
        }
    }

    /// Gets a reference to the item at the given position in this grid, without first performing a bounds check.
    pub unsafe fn get_unchecked<Idx: GridIndex>(&self, pos: Idx) -> &T {
        let idx = index1d(pos, self.width());
        unsafe { self.buf.get_unchecked(idx) }
    }

    /// Gets a mutable reference to the item at the given position in this grid, without first performing a bounds
    /// check.
    pub unsafe fn get_unchecked_mut<Idx: GridIndex>(&mut self, pos: Idx) -> &mut T {
        let idx = index1d(pos, self.width());
        unsafe { self.buf.get_unchecked_mut(idx) }
    }

    /// Checks whether or not the cell in the given direction from the starting position is within the bounds of this
    /// grid.
    pub fn has_neighbour<Idx: GridIndex, Dir: Direction>(&self, pos: Idx, dir: Dir) -> bool {
        dir.checked_add(pos, self.size()).is_some()
    }

    /// Gets a reference to the item in front of the given position, in the given direction.
    ///
    /// To access the neighbouring _positions themselves_ (not just references), see [`Grid::neighbours`].
    pub fn get_neighbour<Idx: GridIndex, Dir: Direction>(&self, pos: Idx, dir: Dir) -> Option<&T> {
        let pos = dir.checked_add(pos, self.size())?;
        // SAFETY: `checked_add` checks bounds.
        Some(unsafe { self.get_unchecked(pos) })
    }

    /// Gets a mutable reference to the item in front of the given position, in the given direction.
    ///
    /// To access the neighbouring _positions themselves_ (not just references), see [`Grid::neighbours`].
    pub fn get_neighbour_mut<Idx: GridIndex, Dir: Direction>(&mut self, pos: Idx, dir: Dir) -> Option<&mut T> {
        let pos = dir.checked_add(pos, self.size())?;
        // SAFETY: `checked_add` checks bounds.
        Some(unsafe { self.get_unchecked_mut(pos) })
    }

    /// Gets a reference to the [neighbouring positions][Neighbours] around the given position, or `None` if `pos` is
    /// out of bounds.
    pub fn neighbours<Idx: GridIndex>(&self, pos: Idx) -> Option<Neighbours<Idx>> {
        self.contains(pos).then(|| Neighbours::new(pos, self.size()))
    }

    /// Creates a new grid of the given size by calling `f` once for every (x, y) position of the grid.
    pub fn from_fn<F>(w: usize, h: usize, mut f: F) -> Self
    where
        F: FnMut((usize, usize)) -> T,
    {
        let mut buf = Vec::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                buf.push(f((x, y)));
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
    pub fn from_lines_map<I, S>(lines: I, mut f: impl FnMut(char, (usize, usize)) -> T) -> Result<Self, ParseGridError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        match Self::try_from_lines_map::<I, S, Infallible>(lines, move |x, p| Ok(f(x, p))) {
            Ok(grid) => Ok(grid),
            Err(TryParseGridError::RowSize(n)) => Err(ParseGridError::RowSize(n)),
            Err(TryParseGridError::MapFnError(_)) => unreachable!(), // map_fn never returns Err
        }
    }

    /// Creates a new grid by attempting to call the provided mapping function on each character of the source input.
    ///
    /// The mapping function is passed both the source character and the (x, y) position at which it appears.
    pub fn try_from_lines_map<I, S, E>(
        lines: I,
        mut f: impl FnMut(char, (usize, usize)) -> Result<T, E>,
    ) -> Result<Self, TryParseGridError<E>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut lines = lines.into_iter();

        // Grab the first line first before starting the loop so we can eagerly allocate space based on its size.
        let Some(first_line) = lines.next() else {
            return Ok(Grid { w: 0, h: 0, buf: Box::new([]) });
        };

        let w = first_line.as_ref().len();
        let mut buf = Vec::with_capacity(w * w); // Assume square to start with; will shrink to boxed_slice later.
        let mut h = 0;

        let all_lines = std::iter::once(first_line).chain(lines);
        for line in all_lines {
            let line = line.as_ref();
            if line.len() == w {
                buf.reserve(line.len()); // NB: *not* `reserve_exact`
                for (x, c) in line.chars().enumerate() {
                    let res = f(c, (x, h)).map_err(TryParseGridError::MapFnError)?;
                    buf.push(res);
                }
                h += 1;
            } else {
                return Err(TryParseGridError::RowSize(h + 1));
            }
        }

        let buf = buf.into_boxed_slice();
        Ok(Grid { w, h, buf })
    }

    /// Creates a new [Grid] with the same size as this one by applying a mapping function to each element.
    pub fn map<B, F>(&self, mut f: F) -> Grid<B>
    where
        F: FnMut(&T, (usize, usize)) -> B,
    {
        let buf = self.positions().map(|pos| f(&self[pos], pos)).collect::<Box<[B]>>();
        Grid { buf, w: self.w, h: self.h }
    }
}

impl<T, I: GridIndex> Index<I> for Grid<T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let i = index1d(index, self.width());
        &self.buf[i]
    }
}

impl<T, I: GridIndex> IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let i = index1d(index, self.width());
        &mut self.buf[i]
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Pos, &'a T);
    type IntoIter = Entries<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Entries::new(self)
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = (Pos, &'a mut T);
    type IntoIter = EntriesMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        EntriesMut::new(self)
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Grid:")?;
        for y in 0..self.height() {
            for x in 0..self.width() {
                if let Some(width) = f.width() {
                    write!(f, "{:width$?}", &self[(x, y)], width = width)?;
                    if x < self.width() - 1 && width > 0 {
                        f.write_char(' ')?;
                    }
                } else {
                    write!(f, "{:?}", &self[(x, y)])?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "Size: {}×{}", self.w, self.h)?;
        Ok(())
    }
}
