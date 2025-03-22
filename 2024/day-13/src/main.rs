use std::ops::{Add, AddAssign, Mul, MulAssign, SubAssign};

use day_13::Rational;
use regex::Regex;

fn main() {
    let input = aoc_utils::puzzle_input();
    let regex =
        Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

    let systems = regex.captures_iter(&input).map(|caps| System {
        x: Row {
            a: caps[1].parse::<i64>().unwrap().into(),
            b: caps[3].parse::<i64>().unwrap().into(),
            p: caps[5].parse::<i64>().unwrap().into(),
        },
        y: Row {
            a: caps[2].parse::<i64>().unwrap().into(),
            b: caps[4].parse::<i64>().unwrap().into(),
            p: caps[6].parse::<i64>().unwrap().into(),
        },
    });

    let mut total1 = 0;
    let mut total2 = 0;
    for (i, mut sys) in systems.enumerate() {
        let mut big = sys.into_big();

        if let Some((a, b)) = solve_system(&mut sys) {
            println!("System #{i} has solution (a = {a}, b = {b})");
            total1 += 3 * a + b;
        } else {
            println!("System #{i} has no solution.");
        }

        if let Some((a, b)) = solve_system(&mut big) {
            println!("System #{i} (big) has solution (a = {a}, b = {b})");
            total2 += 3 * a + b;
        } else {
            println!("System #{i} (big) has no solution.");
        }
    }

    println!("\nTotal tokens to win all prizes (part 1): {total1}");
    println!("Total tokens to win all prizes, 10-trillion away (part 2): {total2}");
}

/// Finds the solution to the given system.
pub fn solve_system(sys: &mut System) -> Option<(i64, i64)> {
    // 1. Divide the first row by column 1 to obtain a pivot in the first column
    let pivot = sys.x.a.recip().unwrap();
    sys.x *= pivot;
    sys.x.reduce();

    // 2. Scale the pivot to eliminate the first column in the second row
    sys.y -= sys.y.a * &sys.x;
    sys.y.reduce();

    // 3. Now pivot back upwards with the second row
    let pivot = sys.y.b.recip().unwrap();
    sys.y *= pivot;
    sys.y.reduce();

    sys.x -= sys.x.b * &sys.y;
    sys.x.reduce();

    let a = sys.x.p.to_int()?;
    let b = sys.y.p.to_int()?;
    Some((a, b))
}


#[derive(Debug, Clone)]
pub struct Row {
    pub a: Rational,
    pub b: Rational,
    pub p: Rational,
}

#[derive(Debug, Clone)]
pub struct System {
    pub x: Row,
    pub y: Row,
}

impl System {
    pub fn into_big(&self) -> System {
        System {
            x: Row { p: self.x.p + 10000000000000, ..self.x },
            y: Row { p: self.y.p + 10000000000000, ..self.y },
        }
    }
}

impl Row {
    pub fn reduce(&mut self) {
        self.a = self.a.reduced();
        self.b = self.b.reduced();
        self.p = self.p.reduced();
    }
}

impl MulAssign<Rational> for Row {
    fn mul_assign(&mut self, rhs: Rational) {
        self.a *= rhs;
        self.b *= rhs;
        self.p *= rhs;
    }
}

impl AddAssign<Row> for Row {
    fn add_assign(&mut self, rhs: Row) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.p += rhs.p;
    }
}

impl SubAssign<Row> for Row {
    fn sub_assign(&mut self, rhs: Row) {
        self.a -= rhs.a;
        self.b -= rhs.b;
        self.p -= rhs.p;
    }
}

impl Add<Row> for Row {
    type Output = Row;

    fn add(self, rhs: Row) -> Self::Output {
        Row {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            p: self.p + rhs.p,
        }
    }
}

impl Mul<Rational> for Row {
    type Output = Row;

    fn mul(self, rhs: Rational) -> Self::Output {
        Row {
            a: self.a * rhs,
            b: self.b * rhs,
            p: self.p * rhs,
        }
    }
}

impl Mul<&Row> for Rational {
    type Output = Row;

    fn mul(self, rhs: &Row) -> Self::Output {
        Row {
            a: rhs.a * self,
            b: rhs.b * self,
            p: rhs.p * self,
        }
    }
}
