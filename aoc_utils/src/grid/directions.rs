//! Directions within a [two dimensional grid][super::Grid].
//!
//! Grids are fairly common in Advent of Code puzzles. Of all the puzzles that related to two-dimensional grids, a great
//! number of them involve considering "directions". Which way is the robot currently facing? Which neighbours of a
//! given cell meet some condition?
//!
//! The primary purpose of [`Dir4`] and [`Dir8`] from this module are to avoid having to redefine a new enum and any
//! associated helper methods every single time a new puzzle involves directions.

use std::fmt::{self, Debug, Display};
use std::ops::Neg;
use std::str::FromStr;

use auto_ops::impl_op_ex;
use thiserror::Error;

use super::GridIndex;

/// Types that represent an `x`- and/or `y`-offset in a 2D grid.
pub trait Direction: Copy + Into<Dir8> {
    /// An iterator that yields all the individual directions for this [`Direction`].
    type Iter: Iterator<Item = Self>;

    /// Returns a new instance of [`Self::Iter`][Direction::Iter].
    fn iter() -> Self::Iter;

    /// Gets the horizontal component of this direction.
    ///
    /// Positive values represent "right" or "east".
    fn x_offset(&self) -> Offset;

    /// Gets the vertical component of this direction.
    ///
    /// Positive values represent "down" or "south".
    fn y_offset(&self) -> Offset;

    /// Converts this direction into a [`Dir8`], which is the highest "resolution" direction.
    fn into_dir8(self) -> Dir8 {
        self.into()
    }

    /// Adds this direction to the given position, returning `Some` as long as the resulting position is within the
    /// givin `(w, h)` limits.
    fn checked_add<Idx: GridIndex>(self, pos: Idx, limits: (usize, usize)) -> Option<Idx> {
        let (w, h) = limits;
        let x = pos.x().checked_add_signed(self.x_offset().as_isize())?;
        let y = pos.y().checked_add_signed(self.y_offset().as_isize())?;
        (x < w && y < h).then(|| Idx::from_xy(x, y))
    }

    /// Adds this direction to the given position `n` times, returning `Some` as long as the resulting position is
    /// within the givin `(w, h)` limits.
    ///
    /// # Panics
    ///
    /// This function panics if `n` is greater than [`isize::MAX`].
    fn checked_add_n<Idx: GridIndex>(self, pos: Idx, n: usize, limits: (usize, usize)) -> Option<Idx> {
        let (w, h) = limits;
        let n = isize::try_from(n).expect("n should be <= isize::MAX");
        let x = pos.x().checked_add_signed(self.x_offset().as_isize() * n)?;
        let y = pos.y().checked_add_signed(self.y_offset().as_isize() * n)?;
        (x < w && y < h).then(|| Idx::from_xy(x, y))
    }
}

/// An offset of either +1, -1, or 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Offset {
    Positive = 1,
    Zero = 0,
    Negative = -1,
}

impl Offset {
    /// Gets this [Offset] as an [`i8`] with value `-1`, `1`, or `0`.
    pub const fn as_i8(self) -> i8 {
        match self {
            Offset::Positive => 1,
            Offset::Zero => 0,
            Offset::Negative => -1,
        }
    }

    /// Gets this [Offset] as an [`i16`] with value `-1`, `1`, or `0`.
    pub const fn as_i16(self) -> i16 {
        self.as_i8() as i16
    }

    /// Gets this [Offset] as an [`i32`] with value `-1`, `1`, or `0`.
    pub const fn as_i32(self) -> i32 {
        self.as_i8() as i32
    }

    /// Gets this [Offset] as an [`i64`] with value `-1`, `1`, or `0`.
    pub const fn as_i64(self) -> i64 {
        self.as_i8() as i64
    }

    /// Gets this [Offset] as an [`isize`] with value `-1`, `1`, or `0`.
    pub const fn as_isize(self) -> isize {
        self.as_i8() as isize
    }

    /// Returns `true` if this [Offset] is [`Offset::Negative`].
    pub const fn is_neg(&self) -> bool {
        std::matches!(self, Offset::Negative)
    }

    /// Returns `true` if this [Offset] is [`Offset::Positive`].
    pub const fn is_pos(&self) -> bool {
        std::matches!(self, Offset::Positive)
    }

    /// Returns `true` if this [Offset] is [`Offset::Zero`].
    pub const fn is_zero(&self) -> bool {
        std::matches!(self, Offset::Zero)
    }
}

/// A direction which points either up, down, left, or right (north, south, east, west).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

/// A direction which points in any of the eight major cardinal directions.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir8 {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction for Dir4 {
    type Iter = Dir4Iter;

    fn iter() -> Self::Iter {
        Dir4::iter()
    }

    fn x_offset(&self) -> Offset {
        match self {
            Dir4::Left => Offset::Negative,
            Dir4::Right => Offset::Positive,
            Dir4::Up | Dir4::Down => Offset::Zero,
        }
    }

    fn y_offset(&self) -> Offset {
        match self {
            Dir4::Up => Offset::Negative,
            Dir4::Down => Offset::Positive,
            Dir4::Left | Dir4::Right => Offset::Zero,
        }
    }
}

impl Direction for Dir8 {
    type Iter = Dir8Iter;

    fn iter() -> Self::Iter {
        Dir8::iter()
    }

    fn x_offset(&self) -> Offset {
        match self {
            Dir8::Up | Dir8::Down => Offset::Zero,
            Dir8::Left | Dir8::UpLeft | Dir8::DownLeft => Offset::Negative,
            Dir8::Right | Dir8::UpRight | Dir8::DownRight => Offset::Positive,
        }
    }

    fn y_offset(&self) -> Offset {
        match self {
            Dir8::Left | Dir8::Right => Offset::Zero,
            Dir8::Up | Dir8::UpLeft | Dir8::UpRight => Offset::Negative,
            Dir8::Down | Dir8::DownLeft | Dir8::DownRight => Offset::Positive,
        }
    }
}

impl From<Dir4> for Dir8 {
    fn from(value: Dir4) -> Self {
        match value {
            Dir4::Up => Dir8::Up,
            Dir4::Down => Dir8::Down,
            Dir4::Left => Dir8::Left,
            Dir4::Right => Dir8::Right,
        }
    }
}

impl Debug for Dir4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir4::Up => write!(f, "Dir4(↑)"),
            Dir4::Down => write!(f, "Dir4(↓)"),
            Dir4::Left => write!(f, "Dir4(←)"),
            Dir4::Right => write!(f, "Dir4(→)"),
        }
    }
}

impl Display for Dir4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir4::Up => write!(f, "↑"),
            Dir4::Down => write!(f, "↓"),
            Dir4::Left => write!(f, "←"),
            Dir4::Right => write!(f, "→"),
        }
    }
}

impl Debug for Dir8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir8::Up => write!(f, "Dir8(↑)"),
            Dir8::UpRight => write!(f, "Dir8(↗)"),
            Dir8::Right => write!(f, "Dir8(→)"),
            Dir8::DownRight => write!(f, "Dir8(↘)"),
            Dir8::Down => write!(f, "Dir8(↓)"),
            Dir8::DownLeft => write!(f, "Dir8(↙)"),
            Dir8::Left => write!(f, "Dir8(←)"),
            Dir8::UpLeft => write!(f, "Dir8(↖)"),
        }
    }
}

impl Display for Dir8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir8::Up => write!(f, "↑"),
            Dir8::UpRight => write!(f, "↗"),
            Dir8::Right => write!(f, "→"),
            Dir8::DownRight => write!(f, "↘"),
            Dir8::Down => write!(f, "↓"),
            Dir8::DownLeft => write!(f, "↙"),
            Dir8::Left => write!(f, "←"),
            Dir8::UpLeft => write!(f, "↖"),
        }
    }
}

impl Dir4 {
    /// Returns an iterator over all four directions in clockwise order, starting from the top.
    pub const fn iter() -> Dir4Iter {
        Dir4Iter::new()
    }

    /// Returns the direction that is 90° to the right of this one.
    pub const fn right(self) -> Self {
        match self {
            Dir4::Up => Dir4::Right,
            Dir4::Right => Dir4::Down,
            Dir4::Down => Dir4::Left,
            Dir4::Left => Dir4::Up,
        }
    }

    /// Returns the direction that is 90° to the left of this one.
    pub const fn left(self) -> Self {
        match self {
            Dir4::Up => Dir4::Left,
            Dir4::Right => Dir4::Up,
            Dir4::Down => Dir4::Right,
            Dir4::Left => Dir4::Down,
        }
    }

    /// Returns the direction pointing in the opposite direction as this one (180°).
    pub const fn behind(self) -> Self {
        match self {
            Dir4::Up => Dir4::Down,
            Dir4::Right => Dir4::Left,
            Dir4::Down => Dir4::Up,
            Dir4::Left => Dir4::Right,
        }
    }

    /// Gets the direction that is `n` turns (in 90° increments) to the right of this one.
    pub const fn right_n(self, n: i32) -> Self {
        if n == 0 {
            self
        } else if n < 0 {
            self.left_n(n.abs())
        } else {
            let r = n.abs().rem_euclid(4);
            match (self, r) {
                (Dir4::Up, 0) => Dir4::Up,
                (Dir4::Up, 1) => Dir4::Right,
                (Dir4::Up, 2) => Dir4::Down,
                (Dir4::Up, 3) => Dir4::Left,
                (Dir4::Right, 0) => Dir4::Right,
                (Dir4::Right, 1) => Dir4::Down,
                (Dir4::Right, 2) => Dir4::Left,
                (Dir4::Right, 3) => Dir4::Up,
                (Dir4::Down, 0) => Dir4::Down,
                (Dir4::Down, 1) => Dir4::Left,
                (Dir4::Down, 2) => Dir4::Up,
                (Dir4::Down, 3) => Dir4::Right,
                (Dir4::Left, 0) => Dir4::Left,
                (Dir4::Left, 1) => Dir4::Up,
                (Dir4::Left, 2) => Dir4::Right,
                (Dir4::Left, 3) => Dir4::Down,
                _ => unreachable!(),
            }
        }
    }

    /// Gets the direction that is `n` turns (in 90° increments) to the left of this one.
    pub const fn left_n(self, n: i32) -> Self {
        if n == 0 {
            self
        } else if n < 0 {
            self.right_n(n.abs())
        } else {
            let r = n.abs().rem_euclid(4);
            match (self, r) {
                (Dir4::Up, 0) => Dir4::Up,
                (Dir4::Up, 1) => Dir4::Left,
                (Dir4::Up, 2) => Dir4::Down,
                (Dir4::Up, 3) => Dir4::Right,
                (Dir4::Left, 0) => Dir4::Left,
                (Dir4::Left, 1) => Dir4::Down,
                (Dir4::Left, 2) => Dir4::Right,
                (Dir4::Left, 3) => Dir4::Up,
                (Dir4::Down, 0) => Dir4::Down,
                (Dir4::Down, 1) => Dir4::Right,
                (Dir4::Down, 2) => Dir4::Up,
                (Dir4::Down, 3) => Dir4::Left,
                (Dir4::Right, 0) => Dir4::Right,
                (Dir4::Right, 1) => Dir4::Up,
                (Dir4::Right, 2) => Dir4::Left,
                (Dir4::Right, 3) => Dir4::Down,
                _ => unreachable!(),
            }
        }
    }
}

impl Dir8 {
    /// Returns an iterator over all eight directions in clockwise order, starting from the top.
    pub const fn iter() -> Dir8Iter {
        Dir8Iter::new()
    }

    /// Returns the direction that is 45° (one step) to the right of this one.
    pub const fn right45(self) -> Self {
        match self {
            Dir8::Up => Dir8::UpRight,
            Dir8::UpRight => Dir8::Right,
            Dir8::Right => Dir8::DownRight,
            Dir8::DownRight => Dir8::Down,
            Dir8::Down => Dir8::DownLeft,
            Dir8::DownLeft => Dir8::Left,
            Dir8::Left => Dir8::UpLeft,
            Dir8::UpLeft => Dir8::Up,
        }
    }

    /// Returns the direction that is 90° (two steps) to the right of this one.
    pub const fn right90(self) -> Self {
        match self {
            Dir8::Up => Dir8::Right,
            Dir8::UpRight => Dir8::DownRight,
            Dir8::Right => Dir8::Down,
            Dir8::DownRight => Dir8::DownLeft,
            Dir8::Down => Dir8::Left,
            Dir8::DownLeft => Dir8::UpLeft,
            Dir8::Left => Dir8::Up,
            Dir8::UpLeft => Dir8::UpRight,
        }
    }

    /// Returns the direction that is 45° (one step) to the left of this one.
    pub const fn left45(self) -> Self {
        match self {
            Dir8::Up => Dir8::UpRight,
            Dir8::UpRight => Dir8::Right,
            Dir8::Right => Dir8::DownRight,
            Dir8::DownRight => Dir8::Down,
            Dir8::Down => Dir8::DownLeft,
            Dir8::DownLeft => Dir8::Left,
            Dir8::Left => Dir8::UpLeft,
            Dir8::UpLeft => Dir8::Left,
        }
    }

    /// Returns the direction that is 90° (two steps) to the left of this one.
    pub const fn left90(self) -> Self {
        match self {
            Dir8::Up => Dir8::Left,
            Dir8::UpRight => Dir8::UpLeft,
            Dir8::Right => Dir8::Up,
            Dir8::DownRight => Dir8::UpRight,
            Dir8::Down => Dir8::Right,
            Dir8::DownLeft => Dir8::DownRight,
            Dir8::Left => Dir8::Down,
            Dir8::UpLeft => Dir8::DownLeft,
        }
    }

    /// Returns the direction pointing in the opposite direction as this one (180°).
    pub const fn behind(self) -> Self {
        match self {
            Dir8::Up => Dir8::Down,
            Dir8::UpRight => Dir8::DownLeft,
            Dir8::Right => Dir8::Left,
            Dir8::DownRight => Dir8::UpLeft,
            Dir8::Down => Dir8::Up,
            Dir8::DownLeft => Dir8::UpRight,
            Dir8::Left => Dir8::Right,
            Dir8::UpLeft => Dir8::DownRight,
        }
    }

    /// Gets the direction that is `n` turns (in 45° increments) to the right of this one.
    pub const fn right_n(self, n: i32) -> Self {
        if n == 0 {
            self
        } else if n < 0 {
            self.left_n(n.abs())
        } else {
            // Could maybe have done this with a macro... but it's a one-time thing and was easy to do with multi-cursor
            // mode. Why bother?
            let r = n.abs().rem_euclid(8);
            match (self, r) {
                (Dir8::Up, 0) => Dir8::Up,
                (Dir8::Up, 1) => Dir8::UpRight,
                (Dir8::Up, 2) => Dir8::Right,
                (Dir8::Up, 3) => Dir8::DownRight,
                (Dir8::Up, 4) => Dir8::Down,
                (Dir8::Up, 5) => Dir8::DownLeft,
                (Dir8::Up, 6) => Dir8::Left,
                (Dir8::Up, 7) => Dir8::UpLeft,
                (Dir8::UpRight, 0) => Dir8::Right,
                (Dir8::UpRight, 1) => Dir8::DownRight,
                (Dir8::UpRight, 2) => Dir8::Down,
                (Dir8::UpRight, 3) => Dir8::DownLeft,
                (Dir8::UpRight, 4) => Dir8::Left,
                (Dir8::UpRight, 5) => Dir8::UpLeft,
                (Dir8::UpRight, 6) => Dir8::Up,
                (Dir8::UpRight, 7) => Dir8::UpRight,
                (Dir8::Right, 0) => Dir8::Right,
                (Dir8::Right, 1) => Dir8::DownRight,
                (Dir8::Right, 2) => Dir8::Down,
                (Dir8::Right, 3) => Dir8::DownLeft,
                (Dir8::Right, 4) => Dir8::Left,
                (Dir8::Right, 5) => Dir8::UpLeft,
                (Dir8::Right, 6) => Dir8::Up,
                (Dir8::Right, 7) => Dir8::UpRight,
                (Dir8::DownRight, 0) => Dir8::DownRight,
                (Dir8::DownRight, 1) => Dir8::Down,
                (Dir8::DownRight, 2) => Dir8::DownLeft,
                (Dir8::DownRight, 3) => Dir8::Left,
                (Dir8::DownRight, 4) => Dir8::UpLeft,
                (Dir8::DownRight, 5) => Dir8::Up,
                (Dir8::DownRight, 6) => Dir8::UpRight,
                (Dir8::DownRight, 7) => Dir8::Right,
                (Dir8::Down, 0) => Dir8::Down,
                (Dir8::Down, 1) => Dir8::DownLeft,
                (Dir8::Down, 2) => Dir8::Left,
                (Dir8::Down, 3) => Dir8::UpLeft,
                (Dir8::Down, 4) => Dir8::Up,
                (Dir8::Down, 5) => Dir8::UpRight,
                (Dir8::Down, 6) => Dir8::Right,
                (Dir8::Down, 7) => Dir8::DownRight,
                (Dir8::DownLeft, 0) => Dir8::DownLeft,
                (Dir8::DownLeft, 1) => Dir8::Left,
                (Dir8::DownLeft, 2) => Dir8::UpLeft,
                (Dir8::DownLeft, 3) => Dir8::Up,
                (Dir8::DownLeft, 4) => Dir8::UpRight,
                (Dir8::DownLeft, 5) => Dir8::Right,
                (Dir8::DownLeft, 6) => Dir8::DownRight,
                (Dir8::DownLeft, 7) => Dir8::Down,
                (Dir8::Left, 0) => Dir8::Left,
                (Dir8::Left, 1) => Dir8::UpLeft,
                (Dir8::Left, 2) => Dir8::Up,
                (Dir8::Left, 3) => Dir8::UpRight,
                (Dir8::Left, 4) => Dir8::Right,
                (Dir8::Left, 5) => Dir8::DownRight,
                (Dir8::Left, 6) => Dir8::Down,
                (Dir8::Left, 7) => Dir8::DownLeft,
                (Dir8::UpLeft, 0) => Dir8::UpLeft,
                (Dir8::UpLeft, 1) => Dir8::Up,
                (Dir8::UpLeft, 2) => Dir8::UpRight,
                (Dir8::UpLeft, 3) => Dir8::Right,
                (Dir8::UpLeft, 4) => Dir8::DownRight,
                (Dir8::UpLeft, 5) => Dir8::Down,
                (Dir8::UpLeft, 6) => Dir8::DownLeft,
                (Dir8::UpLeft, 7) => Dir8::Left,
                _ => unreachable!(),
            }
        }
    }

    /// Gets the direction that is `n` turns (in 45° increments) to the left of this one.
    pub const fn left_n(self, n: i32) -> Self {
        if n == 0 {
            self
        } else if n < 0 {
            self.right_n(n.abs())
        } else {
            let r = n.abs().rem_euclid(8);
            match (self, r) {
                (Dir8::Up, 0) => Dir8::Up,
                (Dir8::Up, 1) => Dir8::UpLeft,
                (Dir8::Up, 2) => Dir8::Left,
                (Dir8::Up, 3) => Dir8::DownLeft,
                (Dir8::Up, 4) => Dir8::Down,
                (Dir8::Up, 5) => Dir8::DownRight,
                (Dir8::Up, 6) => Dir8::Right,
                (Dir8::Up, 7) => Dir8::UpRight,
                (Dir8::UpLeft, 0) => Dir8::UpLeft,
                (Dir8::UpLeft, 1) => Dir8::Left,
                (Dir8::UpLeft, 2) => Dir8::DownLeft,
                (Dir8::UpLeft, 3) => Dir8::Down,
                (Dir8::UpLeft, 4) => Dir8::DownRight,
                (Dir8::UpLeft, 5) => Dir8::Right,
                (Dir8::UpLeft, 6) => Dir8::UpRight,
                (Dir8::UpLeft, 7) => Dir8::Up,
                (Dir8::Left, 0) => Dir8::Left,
                (Dir8::Left, 1) => Dir8::DownLeft,
                (Dir8::Left, 2) => Dir8::Down,
                (Dir8::Left, 3) => Dir8::DownRight,
                (Dir8::Left, 4) => Dir8::Right,
                (Dir8::Left, 5) => Dir8::UpRight,
                (Dir8::Left, 6) => Dir8::Up,
                (Dir8::Left, 7) => Dir8::UpLeft,
                (Dir8::DownLeft, 0) => Dir8::DownLeft,
                (Dir8::DownLeft, 1) => Dir8::Down,
                (Dir8::DownLeft, 2) => Dir8::DownRight,
                (Dir8::DownLeft, 3) => Dir8::Right,
                (Dir8::DownLeft, 4) => Dir8::UpRight,
                (Dir8::DownLeft, 5) => Dir8::Up,
                (Dir8::DownLeft, 6) => Dir8::UpLeft,
                (Dir8::DownLeft, 7) => Dir8::Left,
                (Dir8::Down, 0) => Dir8::Down,
                (Dir8::Down, 1) => Dir8::DownRight,
                (Dir8::Down, 2) => Dir8::Right,
                (Dir8::Down, 3) => Dir8::UpRight,
                (Dir8::Down, 4) => Dir8::Up,
                (Dir8::Down, 5) => Dir8::UpLeft,
                (Dir8::Down, 6) => Dir8::Left,
                (Dir8::Down, 7) => Dir8::DownLeft,
                (Dir8::DownRight, 0) => Dir8::DownRight,
                (Dir8::DownRight, 1) => Dir8::Right,
                (Dir8::DownRight, 2) => Dir8::UpRight,
                (Dir8::DownRight, 3) => Dir8::Up,
                (Dir8::DownRight, 4) => Dir8::UpLeft,
                (Dir8::DownRight, 5) => Dir8::Left,
                (Dir8::DownRight, 6) => Dir8::DownLeft,
                (Dir8::DownRight, 7) => Dir8::Down,
                (Dir8::Right, 0) => Dir8::Right,
                (Dir8::Right, 1) => Dir8::UpRight,
                (Dir8::Right, 2) => Dir8::Up,
                (Dir8::Right, 3) => Dir8::UpLeft,
                (Dir8::Right, 4) => Dir8::Left,
                (Dir8::Right, 5) => Dir8::DownLeft,
                (Dir8::Right, 6) => Dir8::Down,
                (Dir8::Right, 7) => Dir8::DownRight,
                (Dir8::UpRight, 0) => Dir8::UpRight,
                (Dir8::UpRight, 1) => Dir8::Up,
                (Dir8::UpRight, 2) => Dir8::UpLeft,
                (Dir8::UpRight, 3) => Dir8::Left,
                (Dir8::UpRight, 4) => Dir8::DownLeft,
                (Dir8::UpRight, 5) => Dir8::Down,
                (Dir8::UpRight, 6) => Dir8::DownRight,
                (Dir8::UpRight, 7) => Dir8::Right,
                _ => unreachable!(),
            }
        }
    }
}

impl Neg for Dir4 {
    type Output = Dir4;
    fn neg(self) -> Self::Output {
        self.behind()
    }
}

impl Neg for Dir8 {
    type Output = Dir8;
    fn neg(self) -> Self::Output {
        self.behind()
    }
}

macro_rules! impl_dir_ops {
    ($dir:ty, $pos:ty) => {
        impl_op_ex!(+ |pos: &$pos, dir: &$dir| -> $pos {
            let x = pos.x().checked_add_signed(dir.x_offset().as_isize()).expect("attempt to add position + direction with overflow");
            let y = pos.y().checked_add_signed(dir.y_offset().as_isize()).expect("attempt to add position + direction with overflow");
            <$pos as GridIndex>::from_xy(x, y)
        });

        impl_op_ex!(- |pos: &$pos, dir: &$dir| -> $pos {
            // [FIXME] `feature(mixed_integer_ops_unsigned_sub)`: https://github.com/rust-lang/rust/issues/126043
            let x = pos.x().checked_add_signed(-dir.x_offset().as_isize()).expect("attempt to subtract position - direction with overflow");
            let y = pos.y().checked_add_signed(-dir.y_offset().as_isize()).expect("attempt to subtract position - direction with overflow");
            <$pos as GridIndex>::from_xy(x, y)
        });

        impl_op_ex!(+= |pos: &mut $pos, dir: &$dir| {
            *pos = *pos + dir;
        });

        impl_op_ex!(-= |pos: &mut $pos, dir: &$dir| {
            *pos = *pos - dir;
        });
    };
}

impl_dir_ops!(Dir4, (usize, usize));
impl_dir_ops!(Dir8, (usize, usize));
impl_dir_ops!(Dir4, [usize; 2]);
impl_dir_ops!(Dir8, [usize; 2]);

// [FIXME] Sadly, we can't add a blanket `impl` for all `GridIndex` types due to Rust's orphaning rules, hence the
// macro. This will _probably_ be possible in at least _some_ capacity, once specialization _eventually_ lands in
// stable...

/* impl<D: Direction, I: GridIndex> Add<D> for I {
    type Output = I;

    fn add(self, rhs: D) -> Self::Output {
        let x = self.x().checked_add_signed(rhs.x_offset().as_isize()).expect("attempt to add position + direction with overflow");
        let y = self.y().checked_add_signed(rhs.y_offset().as_isize()).expect("attempt to add position + direction with overflow");
        I::from_xy(x, y)
    }
}

impl<D: Direction, I: GridIndex> Sub<D> for I {
    type Output = I;

    fn sub(self, rhs: D) -> Self::Output {
        // [FIXME] `feature(mixed_integer_ops_unsigned_sub)`: https://github.com/rust-lang/rust/issues/126043
        let x = self.x().checked_add_signed(-rhs.x_offset().as_isize()).expect("attempt to subtract position - direction with overflow");
        let y = self.y().checked_add_signed(-rhs.y_offset().as_isize()).expect("attempt to subtract position - direction with overflow");
        I::from_xy(x, y)
    }
}

impl<D: Direction, I: GridIndex> AddAssign<D> for I {
    fn add_assign(&mut self, rhs: D) {
        *self = *self + rhs;
    }
}

impl<D: Direction, I: GridIndex> SubAssign<D> for I {
    fn sub_assign(&mut self, rhs: D) {
        *self = *self - rhs;
    }
} */

/// An error representing failure to convert a single `char` to a [`Dir4`] or [`Dir8`].
#[derive(Debug, PartialEq, Eq, Error)]
#[error("character is not a valid direction")]
pub struct DirFromCharError;

/// An error representing failure to parse a string into a [`Dir4`] or [`Dir8`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseDirError {
    /// The string is either too long or too short to be parsed into a direction.
    #[error("string must be exactly 1 character")]
    InvalidLength,

    /// The parsed character does not represent a direction.
    #[error("character '{}' (U+{:04X}) is not a valid direction", .0, *(.0) as u32)]
    InvalidChar(char),
}

impl TryFrom<char> for Dir4 {
    type Error = DirFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' | '↑' => Ok(Dir4::Up),
            '<' | '←' => Ok(Dir4::Left),
            '>' | '→' => Ok(Dir4::Right),
            'v' | 'V' | '↓' => Ok(Dir4::Down),
            _ => Err(DirFromCharError),
        }
    }
}

impl FromStr for Dir4 {
    type Err = ParseDirError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let c = chars.next().ok_or(ParseDirError::InvalidLength)?;
        if chars.count() > 0 {
            Err(ParseDirError::InvalidLength)
        } else {
            c.try_into().or(Err(ParseDirError::InvalidChar(c)))
        }
    }
}


/// An iterator that yields all four [up–right–down–left directions][Dir4] in clockwise order.
#[derive(Debug, Clone, Copy)]
pub struct Dir4Iter(u8, u8); // 2 counters allow for double-ended iteration

/// An iterator that yields all eight [cardinal directions][Dir8] in clockwise order, starting from north/up.
#[derive(Debug, Clone, Copy)]
pub struct Dir8Iter(u8, u8);

impl Dir4Iter {
    const fn new() -> Self {
        Dir4Iter(0, 4)
    }

    const fn get(i: u8) -> Dir4 {
        match i {
            0 => Dir4::Up,
            1 => Dir4::Right,
            2 => Dir4::Down,
            3 => Dir4::Left,
            _ => panic!("Dir4Iter index out of range"),
        }
    }
}

impl Dir8Iter {
    const fn new() -> Self {
        Dir8Iter(0, 8)
    }

    const fn get(i: u8) -> Dir8 {
        match i {
            0 => Dir8::Up,
            1 => Dir8::UpRight,
            2 => Dir8::Right,
            3 => Dir8::DownRight,
            4 => Dir8::Down,
            5 => Dir8::DownLeft,
            6 => Dir8::Left,
            7 => Dir8::UpLeft,
            _ => panic!("Dir8Iter index out of range"),
        }
    }
}

impl Iterator for Dir4Iter {
    type Item = Dir4;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= self.1 {
            None
        } else {
            let val = Self::get(self.0);
            self.0 += 1;
            Some(val)
        }
    }
}

impl DoubleEndedIterator for Dir4Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.1 <= self.0 {
            None
        } else {
            self.1 -= 1;
            Some(Self::get(self.1))
        }
    }
}

impl Iterator for Dir8Iter {
    type Item = Dir8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= self.1 {
            None
        } else {
            let val = Self::get(self.0);
            self.0 += 1;
            Some(val)
        }
    }
}

impl DoubleEndedIterator for Dir8Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.1 <= self.0 {
            None
        } else {
            self.1 -= 1;
            Some(Self::get(self.1))
        }
    }
}

impl ExactSizeIterator for Dir4Iter {
    fn len(&self) -> usize {
        (self.1 - self.0) as usize
    }
}

impl ExactSizeIterator for Dir8Iter {
    fn len(&self) -> usize {
        (self.1 - self.0) as usize
    }
}
