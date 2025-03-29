use std::marker::PhantomData;

use super::GridIndex;

/// A representation of the neighbours around a particular cell in a [Grid][super::Grid].
///
/// This struct is created by the [`neighbours`][super::Grid::neighbours] method on the [`Grid`][super::Grid] struct.
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

    pub(super) fn new(pos: Idx, w: usize, h: usize) -> Self {
        let (x, y) = pos.to_tuple();

        let n = (y > 0) as u8;
        let e = (x < w - 1) as u8;
        let s = (y < h - 1) as u8;
        let w = (x > 0) as u8;
        let mask = (n << Self::OFFSET_N) | (e << Self::OFFSET_E) | (s << Self::OFFSET_S) | (w << Self::OFFSET_W);

        Neighbours { pos, mask }
    }

    /// Returns the position of the cell.
    pub const fn pos(&self) -> Idx {
        self.pos
    }

    /// Returns the position north of the cell (up), assuming it is in-bounds.
    pub fn n(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_N == Self::MASK_N).then(|| Idx::from_xy(x, y - 1))
    }

    /// Returns the position east of the cell (to the right), assuming it is in-bounds.
    pub fn e(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_E == Self::MASK_E).then(|| Idx::from_xy(x + 1, y))
    }

    /// Returns the position south of the cell (down), assuming it is in-bounds.
    pub fn s(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_S == Self::MASK_S).then(|| Idx::from_xy(x, y + 1))
    }

    /// Returns the position west of the cell (to the left), assuming it is in-bounds.
    pub fn w(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_W == Self::MASK_W).then(|| Idx::from_xy(x - 1, y))
    }

    /// Returns the position north-east of the cell (up and to the right), assuming it is in-bounds.
    pub fn ne(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_NE == Self::MASK_NE).then(|| Idx::from_xy(x + 1, y - 1))
    }

    /// Returns the position south-east of the cell (down and to the right), assuming it is in-bounds.
    pub fn se(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_SE == Self::MASK_SE).then(|| Idx::from_xy(x + 1, y + 1))
    }

    /// Returns the position south-west of the cell (down and to the left), assuming it is in-bounds.
    pub fn sw(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_SW == Self::MASK_SW).then(|| Idx::from_xy(x - 1, y + 1))
    }

    /// Returns the position north-west of the cell (up and to the left), assuming it is in-bounds.
    pub fn nw(&self) -> Option<Idx> {
        let (x, y) = self.pos.to_tuple();
        (self.mask & Self::MASK_NW == Self::MASK_NW).then(|| Idx::from_xy(x - 1, y - 1))
    }

    /// Returns an iterator over the positions of the four adjacent positions around the cell. Any out-of-bounds
    /// neighbours are excluded from iteration.
    pub fn iter_adjacent(&self) -> NeighboursAdjacent<Idx> {
        NeighboursAdjacent(InnerIter::new(*self))
    }

    /// Returns an iterator over the positions of all eight positions that surround the cell. Any out-of-bounds
    /// corners are excluded from iteration.
    pub fn iter_around(&self) -> NeighboursAround<Idx> {
        NeighboursAround(InnerIter::new(*self))
    }

    /// Returns an iterator over the positions of the four corners around the cell. Any out-of-bounds corners are
    /// excluded from iteration.
    pub fn iter_corners(&self) -> NeighboursCorners<Idx> {
        NeighboursCorners(InnerIter::new(*self))
    }

    /// Returns the number of cells adjacent to this one (excluding diagonals) which have a valid neighbour (i.e.,
    /// those that are within the bounds of the [Grid][super::Grid]).
    pub fn num_adjacent(&self) -> u8 {
        (self.mask & Self::MASK_N == Self::MASK_N) as u8
            + (self.mask & Self::MASK_E == Self::MASK_E) as u8
            + (self.mask & Self::MASK_S == Self::MASK_S) as u8
            + (self.mask & Self::MASK_W == Self::MASK_W) as u8
    }

    /// Returns the number of cells diagonally adjacent to this one (excluding N, E, S, W) which have a valid
    /// neighbour (i.e., those that are within the bounds of the [Grid][super::Grid]).
    pub fn num_corners(&self) -> u8 {
        (self.mask & Self::MASK_NE == Self::MASK_NE) as u8
            + (self.mask & Self::MASK_SE == Self::MASK_SE) as u8
            + (self.mask & Self::MASK_SW == Self::MASK_SW) as u8
            + (self.mask & Self::MASK_NW == Self::MASK_NW) as u8
    }

    /// Returns the number of cells around this one (including diagonals) which have a valid neighbour (i.e., those
    /// that are within the bounds of the [Grid][super::Grid]).
    pub fn num_around(&self) -> u8 {
        self.num_adjacent() + self.num_corners()
    }
}

// -----------------------------------------------------------------------------------------------------------------

/// An iterator over the four adjacent neighbours of a cell in a [Grid][super::Grid].
///
/// This struct is created by the [`iter_adjacent`][Neighbours::iter_adjacent] method on the [`Neighbours`] struct.
#[derive(Debug, Clone, Copy)]
pub struct NeighboursAdjacent<Idx: GridIndex>(InnerIter<Idx, AdjacentOrder>);

/// An iterator over the eight surrounding neighbours of a cell in a [Grid][super::Grid].
///
/// This struct is created by the [`iter_around`][Neighbours::iter_around] method on the [`Neighbours`] struct.
#[derive(Debug, Clone, Copy)]
pub struct NeighboursAround<Idx: GridIndex>(InnerIter<Idx, AroundOrder>);

/// An iterator over the four corner-neighbours of a cell in a [Grid][super::Grid].
///
/// This struct is created by the [`iter_corners`][Neighbours::iter_corners] method on the [`Neighbours`] struct.
#[derive(Debug, Clone, Copy)]
pub struct NeighboursCorners<Idx: GridIndex>(InnerIter<Idx, CornerOrder>);

// -----------------------------------------------------------------------------------------------------------------

impl<Idx: GridIndex> Iterator for NeighboursAdjacent<Idx> {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<Idx: GridIndex> Iterator for NeighboursAround<Idx> {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<Idx: GridIndex> Iterator for NeighboursCorners<Idx> {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// -----------------------------------------------------------------------------------------------------------------

/// The implementation behind the iterators for [`Neighbours`]. This struct is parameterized over ZSTs that
/// implement [`Ordering`] to allow for multiple different orderings to reuse the same [`Iterator`] implementation.
#[derive(Debug, Clone, Copy)]
struct InnerIter<Idx: GridIndex, O: Ordering> {
    neighbours: Neighbours<Idx>,
    index: u8,
    order: PhantomData<O>,
}

/// Trait used to control the order in which an [`InnerIter`] yields its elements.
trait Ordering {
    /// The total number of directions yielded by this ordering.
    const COUNT: u8;

    /// Retrieves the `i`th neighbour in sequence from the given [`Neighbours`], according to this particular
    /// ordering.
    fn get<Idx: GridIndex>(neighbours: &Neighbours<Idx>, i: u8) -> Option<Idx>;
}

impl<Idx: GridIndex, O: Ordering> InnerIter<Idx, O> {
    pub fn new(neighbours: Neighbours<Idx>) -> Self {
        Self {
            neighbours,
            index: 0,
            order: PhantomData,
        }
    }
}

impl<Idx: GridIndex, O: Ordering> Iterator for InnerIter<Idx, O> {
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
struct AdjacentOrder;

/// An [Ordering] that yields all eight surrounding neighbours, in N, NE, E, SE, S, SW, W, NW order.
#[derive(Debug, Clone, Copy)]
struct AroundOrder;

/// An [Ordering] that yields diagonally-adjacent neighbours, in NE, SE, SW, NW order.
#[derive(Debug, Clone, Copy)]
struct CornerOrder;

impl Ordering for AdjacentOrder {
    const COUNT: u8 = 4;

    fn get<Idx: GridIndex>(neighbours: &Neighbours<Idx>, i: u8) -> Option<Idx> {
        match i {
            0 => neighbours.n(),
            1 => neighbours.e(),
            2 => neighbours.s(),
            3 => neighbours.w(),
            _ => unreachable!(),
        }
    }
}

impl Ordering for CornerOrder {
    const COUNT: u8 = 4;

    fn get<Idx: GridIndex>(neighbours: &Neighbours<Idx>, i: u8) -> Option<Idx> {
        match i {
            0 => neighbours.ne(),
            1 => neighbours.se(),
            2 => neighbours.sw(),
            3 => neighbours.nw(),
            _ => unreachable!(),
        }
    }
}

impl Ordering for AroundOrder {
    const COUNT: u8 = 8;

    fn get<Idx: GridIndex>(neighbours: &Neighbours<Idx>, i: u8) -> Option<Idx> {
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
