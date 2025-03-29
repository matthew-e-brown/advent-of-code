use std::fmt::{self, Debug, Display};
use std::ops::Neg;
use std::str::FromStr;

use auto_ops::impl_op_ex;
use thiserror::Error;

use crate::grid::{GridIndex, Pos};


/// Types that represent an `x`- and/or `y`-offset in a 2D grid.
pub trait Direction {
    /// Gets the horizontal component of this direction.
    ///
    /// Positive values represent "right" or "east".
    fn x_offset(&self) -> isize;

    /// Gets the vertical component of this direction.
    ///
    /// Positive values represent "down" or "south".
    fn y_offset(&self) -> isize;
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
    fn x_offset(&self) -> isize {
        match self {
            Dir4::Left => -1,
            Dir4::Right => 1,
            Dir4::Up | Dir4::Down => 0,
        }
    }

    fn y_offset(&self) -> isize {
        match self {
            Dir4::Up => -1,
            Dir4::Down => 1,
            Dir4::Left | Dir4::Right => 0,
        }
    }
}

impl Direction for Dir8 {
    fn x_offset(&self) -> isize {
        match self {
            Dir8::Up | Dir8::Down => 0,
            Dir8::Left | Dir8::UpLeft | Dir8::DownLeft => -1,
            Dir8::Right | Dir8::UpRight | Dir8::DownRight => 1,
        }
    }

    fn y_offset(&self) -> isize {
        match self {
            Dir8::Left | Dir8::Right => 0,
            Dir8::Up | Dir8::UpLeft | Dir8::UpRight => -1,
            Dir8::Down | Dir8::DownLeft | Dir8::DownRight => 1,
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

macro_rules! impl_dir_ops {
    ($name:ident) => {
        impl_op_ex!(+ |pos: &Pos, dir: &$name| -> Pos {
            let x = match dir.x_offset() {
                0 => pos.x(),
                x @ ..0 => pos.x() - x.unsigned_abs(),
                x @ 1.. => pos.x() + x.unsigned_abs(),
            };

            let y = match dir.y_offset() {
                0 => pos.y(),
                y @ ..0 => pos.y() - y.unsigned_abs(),
                y @ 1.. => pos.y() + y.unsigned_abs(),
            };

            Pos::from_xy(x, y)
        });

        impl_op_ex!(- |pos: &Pos, dir: &$name| -> Pos {
            let x = match dir.x_offset() {
                0 => pos.0,
                x @ ..0 => pos.0 + x.unsigned_abs(), // if x is -1, subtracting it means adding +1.
                x @ 1.. => pos.0 - x.unsigned_abs(),
            };

            let y = match dir.y_offset() {
                0 => pos.1,
                y @ ..0 => pos.1 + y.unsigned_abs(),
                y @ 1.. => pos.1 - y.unsigned_abs(),
            };

            Pos::from_xy(x, y)
        });

        impl_op_ex!(+= |pos: &mut Pos, dir: &$name| {
            match dir.x_offset() {
                0 => {},
                x @ ..0 => pos.0 += x.unsigned_abs(),
                x @ 1.. => pos.0 -= x.unsigned_abs(),
            }

            match dir.y_offset() {
                0 => {},
                y @ ..0 => pos.1 += y.unsigned_abs(),
                y @ 1.. => pos.1 -= y.unsigned_abs(),
            }
        });

        impl_op_ex!(-= |pos: &mut Pos, dir: &$name| {
            match dir.x_offset() {
                0 => {},
                x @ ..0 => pos.0 -= x.unsigned_abs(),
                x @ 1.. => pos.0 += x.unsigned_abs(),
            }

            match dir.y_offset() {
                0 => {},
                y @ ..0 => pos.1 -= y.unsigned_abs(),
                y @ 1.. => pos.1 += y.unsigned_abs(),
            }
        });

        impl Neg for $name {
            type Output = $name;
            fn neg(self) -> Self::Output {
                self.behind()
            }
        }
    };
}

impl_dir_ops!(Dir4);
impl_dir_ops!(Dir8);

// Sadly, we can't add an `impl` like this due to Rust's orphaning rules; so we need to use a macro instead. This will
// _probably_ be possible in at least _some_ capacity once specialization lands in stable; [FIXME] then.

/* impl<D: Direction, I: GridIndex> Add<D> for I {
    type Output = I;

    fn add(self, rhs: D) -> Self::Output {
        let x = match rhs.x_offset() {
            0 => self.x(),
            x @ ..0 => self.x() - x.unsigned_abs(),
            x @ 1.. => self.x() + x.unsigned_abs(),
        };

        let y = match rhs.y_offset() {
            0 => self.y(),
            y @ ..0 => self.y() - y.unsigned_abs(),
            y @ 1.. => self.y() + y.unsigned_abs(),
        };

        I::from_xy(x, y)
    }
}

impl<D: Direction> Sub<D> for Pos {
    type Output = Pos;

    fn sub(self, rhs: D) -> Self::Output {
        let x = match rhs.x_offset() {
            0 => self.0,
            x @ ..0 => self.0 + x.unsigned_abs(), // if x is -1, subtracting it means adding +1.
            x @ 1.. => self.0 - x.unsigned_abs(),
        };

        let y = match rhs.y_offset() {
            0 => self.1,
            y @ ..0 => self.1 + y.unsigned_abs(),
            y @ 1.. => self.1 - y.unsigned_abs(),
        };

        (x, y)
    }
}

impl<D: Direction> AddAssign<D> for Pos {
    fn add_assign(&mut self, rhs: D) {
        match rhs.x_offset() {
            0 => {},
            x @ ..0 => self.0 += x.unsigned_abs(),
            x @ 1.. => self.0 -= x.unsigned_abs(),
        }

        match rhs.y_offset() {
            0 => {},
            y @ ..0 => self.1 += y.unsigned_abs(),
            y @ 1.. => self.1 -= y.unsigned_abs(),
        }
    }
}

impl<D: Direction> SubAssign<D> for Pos {
    fn sub_assign(&mut self, rhs: D) {
        match rhs.x_offset() {
            0 => {},
            x @ ..0 => self.0 -= x.unsigned_abs(),
            x @ 1.. => self.0 += x.unsigned_abs(),
        }

        match rhs.y_offset() {
            0 => {},
            y @ ..0 => self.1 -= y.unsigned_abs(),
            y @ 1.. => self.1 += y.unsigned_abs(),
        }
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
