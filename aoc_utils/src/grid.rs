use std::convert::Infallible;
use std::fmt::{self, Debug, Write};
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
    /// An error occurred parsing the input grid itself (as opposed to one caused by a map function).
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

    /// Checks whether or not the given position is within the bounds of this grid's size.
    pub fn contains<Idx: GridIndex>(&self, pos: Idx) -> bool {
        let (x, y) = pos.to_tuple();
        x < self.w && y < self.h
    }

    /// Gets a reference to the item at the given position in this grid. Returns `None` if `pos` is out of bounds.
    pub fn get<Idx: GridIndex>(&self, pos: Idx) -> Option<&T> {
        let (x, y) = pos.to_tuple();
        if self.contains((x, y)) {
            // SAFETY: Just checked bounds.
            Some(unsafe { self.get_unchecked((x, y)) })
        } else {
            None
        }
    }

    /// Gets a mutable reference to the item at the given position in this grid. Returns `None` if `pos` is out of
    /// bounds.
    pub fn get_mut<Idx: GridIndex>(&mut self, pos: Idx) -> Option<&mut T> {
        let (x, y) = pos.to_tuple();
        if self.contains((x, y)) {
            // SAFETY: Just checked bounds.
            Some(unsafe { self.get_unchecked_mut((x, y)) })
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

    /// Checks the positions around the given cell, checking each one against the bounds of the grid. If the given cell
    /// is out-of-bounds, `None` is returned.
    pub fn neighbours<Idx: GridIndex>(&self, pos: Idx) -> Option<Neighbours<Idx>> {
        let (x, y) = pos.to_tuple();
        let (w, h) = self.size();
        if self.contains((x, y)) { Some(Neighbours::new(pos, w, h)) } else { None }
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
    pub fn from_lines_map<I, S, F>(lines: I, mut f: F) -> Result<Self, ParseGridError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        F: FnMut(char, (usize, usize)) -> T,
    {
        match Self::try_from_lines_map::<Infallible, I, S, _>(lines, move |x, p| Ok(f(x, p))) {
            Ok(grid) => Ok(grid),
            Err(TryParseGridError::GridError(err)) => Err(err),
            Err(TryParseGridError::MapFnError(_)) => unreachable!(), // map_fn never returns Err
        }
    }

    /// Creates a new grid by attempting to call the provided mapping function on each character of the source input.
    ///
    /// The mapping function is passed both the source character and the (x, y) position at which it appears.
    pub fn try_from_lines_map<E, I, S, F>(lines: I, mut f: F) -> Result<Self, TryParseGridError<E>>
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
                return Err(ParseGridError::RowSize { exp: w, acc: line.len() }.into());
            }
        }

        let buf = buf.into_boxed_slice();
        Ok(Grid { w, h, buf })
    }

    /// Creates a new [Grid] with the same size as this one by applying a mapping function to each element.
    pub fn map<B, F>(&self, f: F) -> Grid<B>
    where
        F: FnMut(&T) -> B,
    {
        let buf = self.buf.iter().map(f).collect::<Box<[B]>>();
        Grid { buf, w: self.w, h: self.h }
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
                if let Some(width) = f.width() {
                    write!(f, "{:width$?}", &self[[x, y]], width = width)?;
                    if x < self.w - 1 && width > 0 {
                        f.write_char(' ')?;
                    }
                } else {
                    write!(f, "{:?}", &self[[x, y]])?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "Size: {}×{}", self.w, self.h)?;
        Ok(())
    }
}

/// A trait representing objects that can be used to index a [two-dimensional grid][Grid].
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

    /// Normalizes this index as an array to make it easier to destructure the `x` and `y` components.
    fn to_array(self) -> [usize; 2] {
        [self.x(), self.y()]
    }
}

/// Given the width of a [Grid], converts a two-dimensional [GridIndex] into a one-dimensional buffer offset.
#[inline]
fn index1d<Idx: GridIndex>(pos: Idx, w: usize) -> usize {
    let (x, y) = pos.to_tuple();
    y * w + x
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

// =====================================================================================================================
// =====================================================================================================================
// =====================================================================================================================

/// A representation of the neighbours around a particular cell in a [Grid].
///
/// This struct is created by the [`neighbours`][Grid::neighbours] method on the [`Grid`] struct.
#[derive(Debug, Clone, Copy)]
pub struct Neighbours<Idx: GridIndex> {
    pos: Idx,
    mask: u8,
}

impl<Idx: GridIndex> Neighbours<Idx> {
    const MASK_N: u8 = 0b1000;
    const MASK_E: u8 = 0b0100;
    const MASK_S: u8 = 0b0010;
    const MASK_W: u8 = 0b0001;

    // Diagonals are simply ANDs of their two composite directions:
    const MASK_NE: u8 = 0b1100;
    const MASK_SE: u8 = 0b0110;
    const MASK_SW: u8 = 0b0011;
    const MASK_NW: u8 = 0b1001;

    const OFFSET_N: u8 = 3;
    const OFFSET_E: u8 = 2;
    const OFFSET_S: u8 = 1;
    const OFFSET_W: u8 = 0;

    pub(self) fn new(pos: Idx, w: usize, h: usize) -> Self {
        let (x, y) = pos.to_tuple();

        let n = (y > 0) as u8;
        let e = (x < w - 1) as u8;
        let s = (y < h - 1) as u8;
        let w = (x > 0) as u8;
        let mask = (n << Self::OFFSET_N) | (e << Self::OFFSET_E) | (s << Self::OFFSET_S) | (w << Self::OFFSET_W);

        Neighbours { pos, mask }
    }

    /// Returns the position of the cell.
    pub fn pos(&self) -> Idx {
        self.pos
    }

    /// Returns the position north of the cell (up), assuming it is in-bounds.
    pub fn n(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_N > 0).then(|| Idx::from_xy(x, y - 1))
    }

    /// Returns the position east of the cell (to the right), assuming it is in-bounds.
    pub fn e(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_E > 0).then(|| Idx::from_xy(x + 1, y))
    }

    /// Returns the position south of the cell (down), assuming it is in-bounds.
    pub fn s(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_S > 0).then(|| Idx::from_xy(x, y + 1))
    }

    /// Returns the position west of the cell (to the left), assuming it is in-bounds.
    pub fn w(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_W > 0).then(|| Idx::from_xy(x - 1, y))
    }

    /// Returns the position north-east of the cell (up and to the right), assuming it is in-bounds.
    pub fn ne(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_NE > 0).then(|| Idx::from_xy(x + 1, y - 1))
    }

    /// Returns the position south-east of the cell (down and to the right), assuming it is in-bounds.
    pub fn se(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_SE > 0).then(|| Idx::from_xy(x + 1, y + 1))
    }

    /// Returns the position south-west of the cell (down and to the left), assuming it is in-bounds.
    pub fn sw(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_SW > 0).then(|| Idx::from_xy(x - 1, y + 1))
    }

    /// Returns the position north-west of the cell (up and to the left), assuming it is in-bounds.
    pub fn nw(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_NW > 0).then(|| Idx::from_xy(x - 1, y - 1))
    }

    /// Returns an iterator over the positions of the four adjacent positions around the cell. Any out-of-bounds
    /// neighbours are excluded from iteration.
    pub fn iter_adjacent(&self) -> iter::IterAdjacent<Idx> {
        iter::IterAdjacent(iter::InnerIter::new(*self))
    }

    /// Returns an iterator over the positions of all eight positions that surround the cell. Any out-of-bounds corners
    /// are excluded from iteration.
    pub fn iter_around(&self) -> iter::IterAround<Idx> {
        iter::IterAround(iter::InnerIter::new(*self))
    }

    /// Returns an iterator over the positions of the four corners around the cell. Any out-of-bounds corners are
    /// excluded from iteration.
    pub fn iter_corners(&self) -> iter::IterCorners<Idx> {
        iter::IterCorners(iter::InnerIter::new(*self))
    }

    /// Returns the number of cells adjacent to this one (excluding diagonals) which have a valid neighbour (i.e., those
    /// that are within the bounds of the [Grid]).
    pub fn num_adjacent(&self) -> u8 {
        (self.mask & Self::MASK_N > 0) as u8
            + (self.mask & Self::MASK_E > 0) as u8
            + (self.mask & Self::MASK_S > 0) as u8
            + (self.mask & Self::MASK_W > 0) as u8
    }

    /// Returns the number of cells diagonally adjacent to this one (excluding N, E, S, W) which have a valid neighbour
    /// (i.e., those that are within the bounds of the [Grid]).
    pub fn num_corners(&self) -> u8 {
        (self.mask & Self::MASK_NE > 0) as u8
            + (self.mask & Self::MASK_SE > 0) as u8
            + (self.mask & Self::MASK_SW > 0) as u8
            + (self.mask & Self::MASK_NW > 0) as u8
    }

    /// Returns the number of cells around this one (including diagonals) which have a valid neighbour (i.e., those that
    /// are within the bounds of the [Grid]).
    pub fn num_around(&self) -> u8 {
        self.num_adjacent() + self.num_corners()
    }
}

pub mod iter {
    use std::marker::PhantomData;

    use super::{GridIndex, Neighbours};

    /// An iterator over the four adjacent neighbours of a cell in a [Grid][super::Grid].
    ///
    /// This struct is created by the [`iter_adjacent`][Neighbours::iter_adjacent] method on the [`Neighbours`] struct.
    #[derive(Debug, Clone, Copy)]
    pub struct IterAdjacent<Idx: GridIndex>(pub(super) InnerIter<Idx, AdjacentOrder>);

    /// An iterator over the eight surrounding neighbours of a cell in a [Grid][super::Grid].
    ///
    /// This struct is created by the [`iter_around`][Neighbours::iter_around] method on the [`Neighbours`] struct.
    #[derive(Debug, Clone, Copy)]
    pub struct IterAround<Idx: GridIndex>(pub(super) InnerIter<Idx, AroundOrder>);

    /// An iterator over the four corner-neighbours of a cell in a [Grid][super::Grid].
    ///
    /// This struct is created by the [`iter_corners`][Neighbours::iter_corners] method on the [`Neighbours`] struct.
    #[derive(Debug, Clone, Copy)]
    pub struct IterCorners<Idx: GridIndex>(pub(super) InnerIter<Idx, CornerOrder>);

    // -----------------------------------------------------------------------------------------------------------------

    impl<Idx: GridIndex> Iterator for IterAdjacent<Idx> {
        type Item = Idx;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    impl<Idx: GridIndex> Iterator for IterAround<Idx> {
        type Item = Idx;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    impl<Idx: GridIndex> Iterator for IterCorners<Idx> {
        type Item = Idx;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    // -----------------------------------------------------------------------------------------------------------------

    /// Trait used to control the order in which an [`InnerIter`] yields its elements.
    pub(super) trait Ordering<Idx: GridIndex> {
        /// The total number of directions yielded by this ordering.
        const COUNT: u8;

        /// Retrieves the `i`th neighbour in sequence from the given [`Neighbours`], according to this particular
        /// ordering.
        fn get(neighbours: &Neighbours<Idx>, i: u8) -> Option<Idx>;
    }

    /// The implementation behind the iterators for [`Neighbours`]. This struct is parameterized over ZSTs that
    /// implement [`Ordering`] to allow for multiple different orderings to reuse the same [`Iterator`] implementation.
    #[derive(Debug, Clone, Copy)]
    pub(super) struct InnerIter<Idx: GridIndex, O: Ordering<Idx>> {
        neighbours: Neighbours<Idx>,
        index: u8,
        order: PhantomData<O>,
    }

    impl<Idx: GridIndex, O: Ordering<Idx>> InnerIter<Idx, O> {
        pub fn new(neighbours: Neighbours<Idx>) -> Self {
            Self {
                neighbours,
                index: 0,
                order: PhantomData,
            }
        }
    }

    impl<Idx: GridIndex, O: Ordering<Idx>> Iterator for InnerIter<Idx, O> {
        type Item = Idx;

        fn next(&mut self) -> Option<Self::Item> {
            while self.index < O::COUNT {
                let value = O::get(&self.neighbours, self.index);
                self.index += 1;
                if value.is_some() {
                    return value;
                }
            }

            None
        }
    }

    // -----------------------------------------------------------------------------------------------------------------

    /// An [Ordering] that yields the four N, E, S, W neighbours, in that order.
    #[derive(Debug, Clone, Copy)]
    pub(super) struct AdjacentOrder;

    /// An [Ordering] that yields all eight surrounding neighbours, in N, NE, E, SE, S, SW, W, NW order.
    #[derive(Debug, Clone, Copy)]
    pub(super) struct AroundOrder;

    /// An [Ordering] that yields diagonally-adjacent neighbours, in NE, SE, SW, NW order.
    #[derive(Debug, Clone, Copy)]
    pub(super) struct CornerOrder;

    impl<Idx: GridIndex> Ordering<Idx> for AdjacentOrder {
        const COUNT: u8 = 4;

        fn get(neighbours: &Neighbours<Idx>, i: u8) -> Option<Idx> {
            match i {
                0 => neighbours.n(),
                1 => neighbours.e(),
                2 => neighbours.s(),
                3 => neighbours.w(),
                _ => unreachable!(),
            }
        }
    }

    impl<Idx: GridIndex> Ordering<Idx> for CornerOrder {
        const COUNT: u8 = 4;

        fn get(neighbours: &Neighbours<Idx>, i: u8) -> Option<Idx> {
            match i {
                0 => neighbours.ne(),
                1 => neighbours.se(),
                2 => neighbours.sw(),
                3 => neighbours.nw(),
                _ => unreachable!(),
            }
        }
    }

    impl<Idx: GridIndex> Ordering<Idx> for AroundOrder {
        const COUNT: u8 = 8;

        fn get(neighbours: &Neighbours<Idx>, i: u8) -> Option<Idx> {
            match i {
                0 => neighbours.n(),
                1 => neighbours.ne(),
                2 => neighbours.e(),
                3 => neighbours.se(),
                4 => neighbours.s(),
                5 => neighbours.sw(),
                6 => neighbours.w(),
                7 => neighbours.nw(),
                _ => unreachable!(),
            }
        }
    }
}
