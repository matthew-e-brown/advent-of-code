//! Structures and iterators relating to the neighbours around a given cell in a two-dimensional grid.

use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};

use super::directions::{Dir4Iter, Dir8Iter, Direction};
use super::{Dir4, Dir8, GridIndex};

/// Helper struct giving bounds-checked access around a particular cell in a [Grid][super::Grid].
///
/// Specifically, this struct provides helper methods for accessing and iterating over the grid indices/positions (see
/// [`GridIndex`]) which _would_ be around a given position in a grid of a given size. This struct holds no reference to
/// the original grid. It is usually constructed by the [`neighbours`][super::Grid::neighbours] method on the
/// [`Grid`][super::Grid] struct, but one may be constructed without a reference to a grid as long as you have a width
/// and a height.
#[derive(Debug, Clone, Copy)]
pub struct Neighbours<Idx: GridIndex> {
    pos: Idx,
    mask: u8,
}

// I was going to do some clever bit-manipulation stuff based on the +/-1 returned by Direction::*_offset functions,
// but... instead I just ended up using `get_mask`.
const MASK_S: u8 = 0b00_01; // vertical, positive
const MASK_N: u8 = 0b00_10; // vertical, negative
const MASK_E: u8 = 0b01_00; // horizontal, positive
const MASK_W: u8 = 0b10_00; // horizontal, negative

const MASK_NE: u8 = MASK_N | MASK_E;
const MASK_SE: u8 = MASK_S | MASK_E;
const MASK_SW: u8 = MASK_S | MASK_W;
const MASK_NW: u8 = MASK_N | MASK_W;

const OFFSET_S: u8 = 0;
const OFFSET_N: u8 = 1;
const OFFSET_E: u8 = 2;
const OFFSET_W: u8 = 3;

#[inline]
fn get_mask(dir: Dir8) -> u8 {
    match dir {
        Dir8::Up => MASK_N,
        Dir8::UpRight => MASK_NE,
        Dir8::Right => MASK_E,
        Dir8::DownRight => MASK_SE,
        Dir8::Down => MASK_S,
        Dir8::DownLeft => MASK_SW,
        Dir8::Left => MASK_W,
        Dir8::UpLeft => MASK_NW,
    }
}

impl<Idx: GridIndex> Neighbours<Idx> {
    /// Creates a new [`Neighbours`] struct.
    pub fn new(pos: Idx, (width, height): (usize, usize)) -> Self {
        let (x, y) = pos.to_tuple();

        let n = (y > 0) as u8;
        let e = (x < width - 1) as u8;
        let s = (y < height - 1) as u8;
        let w = (x > 0) as u8;
        let mask = (n << OFFSET_N) | (e << OFFSET_E) | (s << OFFSET_S) | (w << OFFSET_W);

        Neighbours { pos, mask }
    }

    /// Returns the position of the cell.
    pub const fn pos(&self) -> Idx {
        self.pos
    }

    /// Returns the position next to the cell in a given direction, assuming it is in-bounds.
    pub fn get<Dir: Direction>(&self, dir: Dir) -> Option<Idx> {
        let mask = get_mask(dir.into_dir8());
        if self.mask & mask == mask {
            // Unwrapping these checked adds is fine, since we just checked bounds with the mask.
            Some(Idx::from_xy(
                self.pos.x().checked_add_signed(dir.x_offset().as_isize()).unwrap(),
                self.pos.y().checked_add_signed(dir.y_offset().as_isize()).unwrap(),
            ))
        } else {
            None
        }
    }

    /// Returns the position north of the cell (up), assuming it is in-bounds.
    pub fn n(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & MASK_N == MASK_N).then(|| Idx::from_xy(x, y - 1))
    }

    /// Returns the position east of the cell (to the right), assuming it is in-bounds.
    pub fn e(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & MASK_E == MASK_E).then(|| Idx::from_xy(x + 1, y))
    }

    /// Returns the position south of the cell (down), assuming it is in-bounds.
    pub fn s(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & MASK_S == MASK_S).then(|| Idx::from_xy(x, y + 1))
    }

    /// Returns the position west of the cell (to the left), assuming it is in-bounds.
    pub fn w(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & MASK_W == MASK_W).then(|| Idx::from_xy(x - 1, y))
    }

    /// Returns the position north-east of the cell (up and to the right), assuming it is in-bounds.
    pub fn ne(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & MASK_NE == MASK_NE).then(|| Idx::from_xy(x + 1, y - 1))
    }

    /// Returns the position south-east of the cell (down and to the right), assuming it is in-bounds.
    pub fn se(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & MASK_SE == MASK_SE).then(|| Idx::from_xy(x + 1, y + 1))
    }

    /// Returns the position south-west of the cell (down and to the left), assuming it is in-bounds.
    pub fn sw(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & MASK_SW == MASK_SW).then(|| Idx::from_xy(x - 1, y + 1))
    }

    /// Returns the position north-west of the cell (up and to the left), assuming it is in-bounds.
    pub fn nw(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & MASK_NW == MASK_NW).then(|| Idx::from_xy(x - 1, y - 1))
    }

    /// Returns an iterator over the positions of the four adjacent positions around the cell. Any out-of-bounds
    /// neighbours are excluded from iteration.
    pub fn iter_adjacent(&self) -> IterAdjacent<Idx> {
        IterAdjacent(*self, Dir4::iter())
    }

    /// Returns an iterator over the positions of all eight positions that surround the cell. Any out-of-bounds
    /// corners are excluded from iteration.
    pub fn iter_around(&self) -> IterAround<Idx> {
        IterAround(*self, Dir8::iter())
    }

    /// Returns an iterator over the positions of the four corners around the cell. Any out-of-bounds corners are
    /// excluded from iteration.
    pub fn iter_corners(&self) -> IterCorners<Idx> {
        IterCorners(*self, Dir4::iter())
    }
}

/// An iterator over the four adjacent neighbours of a cell in a [Grid][super::Grid].
///
/// This struct is created by the [`iter_adjacent`][Neighbours::iter_adjacent] method on the [`Neighbours`] struct.
#[derive(Debug, Clone, Copy)]
pub struct IterAdjacent<Idx: GridIndex>(Neighbours<Idx>, Dir4Iter);

/// An iterator over the eight surrounding neighbours of a cell in a [Grid][super::Grid].
///
/// This struct is created by the [`iter_around`][Neighbours::iter_around] method on the [`Neighbours`] struct.
#[derive(Debug, Clone, Copy)]
pub struct IterAround<Idx: GridIndex>(Neighbours<Idx>, Dir8Iter);

/// An iterator over the four corner-neighbours of a cell in a [Grid][super::Grid].
///
/// This struct is created by the [`iter_corners`][Neighbours::iter_corners] method on the [`Neighbours`] struct.
pub struct IterCorners<Idx: GridIndex>(Neighbours<Idx>, Dir4Iter);
// ^works the same as NeighboursAdjacent, but rotates its directions by 45 degrees before indexing Neighbours struct

macro_rules! impl_iter {
    ($name:ident) => {
        impl_iter!($name, dir => dir);
    };
    ($name:ident, $dir:ident => $map:expr) => {
        impl<Idx: GridIndex> Iterator for $name<Idx> {
            type Item = Idx;

            fn next(&mut self) -> Option<Self::Item> {
                self.1.find_map(|$dir| self.0.get($map))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, Some(self.len()))
            }
        }

        impl<Idx: GridIndex> DoubleEndedIterator for $name<Idx> {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.1.rev().find_map(|$dir| self.0.get($map))
            }
        }

        impl<Idx: GridIndex> ExactSizeIterator for $name<Idx> {
            fn len(&self) -> usize {
                self.1.len()
            }
        }

        impl<Idx: GridIndex> FusedIterator for $name<Idx> {}
    };
}

impl_iter!(IterAdjacent);
impl_iter!(IterAround);
impl_iter!(IterCorners, dir => dir.into_dir8().right45());
