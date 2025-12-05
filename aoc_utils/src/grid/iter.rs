use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};
use std::marker::PhantomData;
use std::slice;

use super::{Grid, GridIndex, Pos, index1d};

/// An iterator over all the positions of a grid.
///
/// This struct is usually created by the [`Grid::positions`] method:
///
/// ```
/// # use aoc_utils::grid::Grid;
/// let grid = Grid::<u32>::empty(10, 10);
///
/// for pos in grid.positions() {
///     println!("Cell {pos:?} = {:?}", &grid[pos]);
/// }
/// ```
///
/// However, there may be times when it is preferable to create an instance manually with [`Positions<Idx>::new`], since
/// that will allow you to choose the specific [`GridIndex`] type the iterator will return.
///
/// ```
/// # use aoc_utils::grid::{Grid, GridIndex, Positions};
/// let grid = Grid::<u32>::empty(10, 10);
///
/// # #[derive(Clone, Copy)]
/// struct Vec2 { x: f32, y: f32 }
/// impl GridIndex for Vec2 {
///     /* ... */
///     # fn x(&self) -> usize { self.x as usize }
///     # fn y(&self) -> usize { self.y as usize }
///     # fn from_xy(x: usize, y: usize) -> Self { Vec2 { x: x as f32, y: y as f32 } }
/// }
///
/// for vec in Positions::<Vec2>::new(grid.size()) {
///     println!("Cell at (x, y) of ({:.2}, {:.2}) = {:?}", vec.x, vec.y, &grid[vec]);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Positions<Idx: GridIndex> {
    width: usize,
    front: Pos,
    back: Pos,
    _marker: PhantomData<Idx>,
}

impl<Idx: GridIndex> Positions<Idx> {
    /// Creates a new iterator over all the (x, y) positions within the given bounds.
    pub fn new((w, h): (usize, usize)) -> Self {
        Self {
            width: w,
            front: (0, 0),
            // x is one past the right edge; y is at the bottom level.
            // if h is 0, the ExactSizeIterator impl will give a zero length and no iteration will happen.
            back: (w, h.saturating_sub(1)),
            _marker: PhantomData,
        }
    }
}

#[inline]
fn next_pos((x, y): Pos, w: usize) -> Pos {
    if x < w - 1 { (x + 1, y) } else { (0, y + 1) }
}

#[inline]
fn prev_pos((x, y): Pos, w: usize) -> Pos {
    if x > 0 { (x - 1, y) } else { (w - 1, y - 1) }
}

impl<Idx: GridIndex> Iterator for Positions<Idx> {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            None
        } else {
            // This is an ExactSizeIterator; if len() > 0, we know it'll be safe to increment these counters.
            let curr = self.front.to_tuple();
            self.front = next_pos(curr, self.width);
            Some(Idx::from_xy(curr.0, curr.1))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

impl<Idx: GridIndex> ExactSizeIterator for Positions<Idx> {
    fn len(&self) -> usize {
        // The back pointer points one beyond the next element.
        let a = index1d(self.front, self.width);
        let b = index1d(self.back, self.width);
        b - a
    }
}

impl<Idx: GridIndex> DoubleEndedIterator for Positions<Idx> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            None
        } else {
            // The `back` pointer points one beyond the next item, so we need to step backwards first.
            // Since this is an ExactSizeIterator, we know we don't need any bounds checking here since
            self.back = prev_pos(self.back, self.width);
            Some(Idx::from_xy(self.back.0, self.back.1))
        }
    }
}

impl<Idx: GridIndex> FusedIterator for Positions<Idx> {}

/// An iterator over the values of a [Grid].
#[derive(Debug, Clone)]
pub struct Values<'a, T> {
    inner: slice::Iter<'a, T>,
}

/// An iterator over mutable references inside of a [Grid].
#[derive(Debug)]
pub struct ValuesMut<'a, T> {
    inner: slice::IterMut<'a, T>,
}

/// An iterator over both the positions and values of a [Grid].
#[derive(Debug, Clone)]
pub struct Entries<'a, T> {
    pos_iter: Positions<Pos>,
    buf_iter: slice::Iter<'a, T>,
}

/// An iterator over both the positions and values of a [Grid], providing mutable references to the values.
#[derive(Debug)]
pub struct EntriesMut<'a, T> {
    pos_iter: Positions<Pos>,
    buf_iter: slice::IterMut<'a, T>,
}

impl<'a, T> Values<'a, T> {
    pub(super) fn new(grid: &'a Grid<T>) -> Self {
        Self { inner: grid.buf.iter() }
    }
}

impl<'a, T> ValuesMut<'a, T> {
    pub(super) fn new(grid: &'a mut Grid<T>) -> Self {
        Self { inner: grid.buf.iter_mut() }
    }
}

impl<'a, T> Entries<'a, T> {
    pub(super) fn new(grid: &'a Grid<T>) -> Self {
        Self {
            pos_iter: grid.positions(),
            buf_iter: grid.buf.iter(),
        }
    }
}

impl<'a, T> EntriesMut<'a, T> {
    pub(super) fn new(grid: &'a mut Grid<T>) -> Self {
        Self {
            pos_iter: grid.positions(),
            buf_iter: grid.buf.iter_mut(),
        }
    }
}

// Just so I don't have to type everything twice.
macro_rules! impl_iter {
    ($name:ident, $inner:ty) => {
        impl<'a, T> Iterator for $name<'a, T> {
            type Item = <$inner as Iterator>::Item;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.inner.size_hint()
            }
        }

        impl<'a, T> DoubleEndedIterator for $name<'a, T> {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.inner.next_back()
            }
        }

        impl<'a, T> ExactSizeIterator for $name<'a, T> {
            fn len(&self) -> usize {
                self.inner.len()
            }
        }

        impl<'a, T> FusedIterator for $name<'a, T> {}
    };
}

macro_rules! impl_entries {
    ($name:ident, $inner:ty) => {
        impl<'a, T> Iterator for $name<'a, T> {
            type Item = (<Positions<Pos> as Iterator>::Item, <$inner as Iterator>::Item);

            fn next(&mut self) -> Option<Self::Item> {
                let pos_next = self.pos_iter.next();
                let buf_next = self.buf_iter.next();
                match (pos_next, buf_next) {
                    (Some(p), Some(b)) => Some((p, b)),
                    (None, None) => None,
                    _ => unreachable!("grid::Positions and inner Iter should be exactly the same length"),
                }
            }
        }

        impl<'a, T> ExactSizeIterator for $name<'a, T> {
            fn len(&self) -> usize {
                let length = self.buf_iter.len();
                debug_assert!(
                    length == self.pos_iter.len(),
                    "grid::Positions and inner Iter should be exactly the same length"
                );
                length
            }
        }

        impl<'a, T> DoubleEndedIterator for $name<'a, T> {
            fn next_back(&mut self) -> Option<Self::Item> {
                let pos_next = self.pos_iter.next_back();
                let buf_next = self.buf_iter.next_back();
                match (pos_next, buf_next) {
                    (Some(p), Some(b)) => Some((p, b)),
                    (None, None) => None,
                    _ => unreachable!("grid::Positions and inner Iter should be exactly the same length"),
                }
            }
        }

        impl<'a, T> FusedIterator for $name<'a, T> {}
    };
}

impl_iter!(Values, slice::Iter<'a, T>);
impl_iter!(ValuesMut, slice::IterMut<'a, T>);

impl_entries!(Entries, slice::Iter<'a, T>);
impl_entries!(EntriesMut, slice::IterMut<'a, T>);
