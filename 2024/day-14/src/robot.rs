use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, SubAssign};
use std::str::FromStr;

use aoc_utils::grid::GridIndex;

#[derive(Debug, Clone)]
pub struct Robot {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl FromStr for Robot {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = s.split_whitespace();
        let p = bits.next().unwrap();
        let v = bits.next().ok_or("missing whitespace in robot input")?;
        let pi = p.find("p=").ok_or("robot pos missing 'p='")?;
        let vi = v.find("v=").ok_or("robot vel missing 'v='")?;
        let pxy = p.get(pi + 2..).ok_or("robot p= missing vector")?;
        let vxy = v.get(vi + 2..).ok_or("robot v= missing vector")?;
        Ok(Robot {
            pos: pxy.parse()?,
            vel: vxy.parse()?,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn wrapping_clamp(&mut self, limits: &Vec2) {
        self.x = self.x.rem_euclid(limits.x);
        self.y = self.y.rem_euclid(limits.y);
    }
}

impl TryFrom<Vec2> for (usize, usize) {
    type Error = <usize as TryFrom<i32>>::Error;

    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        let x = usize::try_from(value.x)?;
        let y = usize::try_from(value.y)?;
        Ok((x, y))
    }
}

impl TryFrom<(usize, usize)> for Vec2 {
    type Error = <i32 as TryFrom<usize>>::Error;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let x = i32::try_from(value.0)?;
        let y = i32::try_from(value.1)?;
        Ok(Vec2 { x, y })
    }
}

impl FromStr for Vec2 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or("vec2 missing a comma")?;
        let x = x.parse().or(Err("failed to parse 'x'"))?;
        let y = y.parse().or(Err("failed to parse 'y'"))?;
        Ok(Vec2 { x, y })
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(&(self.x, self.y), f)
    }
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(&(self.x, self.y), f)
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self * rhs.x, y: self * rhs.y }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl GridIndex for Vec2 {
    fn x(&self) -> usize {
        usize::try_from(self.x).unwrap()
    }

    fn y(&self) -> usize {
        usize::try_from(self.y).unwrap()
    }

    fn from_xy(x: usize, y: usize) -> Self {
        (x, y).try_into().unwrap()
    }
}
