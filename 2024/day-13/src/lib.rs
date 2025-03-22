use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A rational number (a fraction).
#[derive(Clone, Copy)]
pub struct Rational {
    n: i32,
    d: i32,
}

impl Rational {
    /// Gets the numerator of this [Rational] number.
    pub fn n(&self) -> i32 {
        self.n
    }

    /// Gets the denominator of this [Rational] number.
    pub fn d(&self) -> i32 {
        self.d
    }

    /// Creates a new [Rational] number with a denominator of 1.
    pub fn int(n: i32) -> Rational {
        Rational { n, d: 1 }
    }

    /// Creates a new [Rational] number, as long as the denominator is not zero.
    ///
    /// The number is not reduced.
    pub fn new(mut n: i32, mut d: i32) -> Option<Rational> {
        if d == 0 {
            None
        } else {
            // If the denominator is negative, flip signs to keep it positive.
            if d < 0 {
                n *= -1;
                d *= -1;
            }

            Some(Rational { n, d })
        }
    }

    /// Returns the reciprocal of this [Rational] number, unless it is zero.
    pub fn recip(self) -> Option<Rational> {
        Rational::new(self.d, self.n)
    }

    /// Returns a reduced version of this [Rational] number.
    pub fn reduced(self) -> Rational {
        let gcd = gcd(self.n, self.d);
        debug_assert!(gcd != 0, "gcd should never be zero");

        let n = self.n / gcd;
        let d = self.d / gcd;
        Rational::new(n, d).unwrap()
    }

    /// Returns the integer representation of this [Rational] number, if it can be reduced to one.
    pub fn to_int(self) -> Option<i32> {
        if let Rational { n, d: 1 } = self.reduced() {
            Some(n)
        } else {
            None
        }
    }

    /// Cross-multiplies this [Rational] number with another, returning their numerators.
    fn cross(&self, rhs: &Rational) -> (i32, i32) {
        let a = self.n * rhs.d;
        let b = rhs.n * self.d;
        (a, b)
    }

    /// Puts the two given [Rational] numbers over their lowest common denominator.
    pub fn common_denom(lhs: &mut Rational, rhs: &mut Rational) {
        if lhs.d == rhs.d {
            return;
        }

        let gcd = gcd(lhs.d, rhs.d);
        let l_factor = rhs.d / gcd;
        let r_factor = lhs.d / gcd;

        lhs.n *= l_factor;
        lhs.d *= l_factor;
        rhs.n *= r_factor;
        rhs.d *= r_factor;
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Display::fmt(&self.n, f)?;
        if !(self.n == 0 || self.d == 1) {
            f.write_str("/")?;
            Display::fmt(&self.d, f)?;
        }

        Ok(())
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(&self.n, f)?;
        if !(self.n == 0 || self.d == 1) {
            f.write_str("/")?;
            Debug::fmt(&self.d, f)?;
        }

        Ok(())
    }
}

impl From<i32> for Rational {
    fn from(value: i32) -> Self {
        Rational::new(value, 1).unwrap()
    }
}

/// Computes the greatest common denominator between two numbers.
///
/// Implementation yoinked from
/// [Wikipedia](https://en.wikipedia.org/w/index.php?title=Binary_GCD_algorithm&oldid=1272402879#Implementation).
fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    // Convert a/b to u32 before doing bitwise operations on them.
    // NB: gcd(a, b) = gcd(-a, b) = gcd(a, -b) = gcd(-a, -b)
    let mut a = a.unsigned_abs();
    let mut b = b.unsigned_abs();

    let i = a.trailing_zeros();
    let j = b.trailing_zeros();
    let k = u32::min(i, j);
    a >>= i;
    b >>= j;

    let d = loop {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        b -= a;
        if b == 0 {
            break a << k;
        }

        b >>= b.trailing_zeros();
    };

    // a/b both originally came from i32, and we know that gcd(a, b) must be smaller than a and b. so the gcd must also
    // fit into an i32.
    i32::try_from(d).unwrap()
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        // Cross-multiply numerators and compare equality:
        let (a, b) = self.cross(other);
        a == b
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (a, b) = self.cross(other);
        a.partial_cmp(&b)
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (a, b) = self.cross(other);
        a.cmp(&b)
    }
}

impl Add<Rational> for Rational {
    type Output = Rational;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        Rational::common_denom(&mut self, &mut rhs);
        Rational::new(self.n + rhs.n, self.d).unwrap()
    }
}

impl Sub<Rational> for Rational {
    type Output = Rational;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        Rational::common_denom(&mut self, &mut rhs);
        Rational::new(self.n - rhs.n, self.d).unwrap()
    }
}

impl Mul<Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self::Output {
        Rational::new(self.n * rhs.n, self.d * rhs.d).unwrap()
    }
}

impl Div<Rational> for Rational {
    type Output = Rational;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.n != 0, "cannot divide by zero!");
        Rational::new(self.n * rhs.d, self.d * rhs.n).unwrap()
    }
}

impl Add<i32> for Rational {
    type Output = Rational;

    fn add(self, rhs: i32) -> Self::Output {
        Rational::new(self.n + rhs * self.d, self.d).unwrap()
    }
}

impl Sub<i32> for Rational {
    type Output = Rational;

    fn sub(self, rhs: i32) -> Self::Output {
        Rational::new(self.n - rhs * self.d, self.d).unwrap()
    }
}

impl Mul<i32> for Rational {
    type Output = Rational;

    fn mul(self, rhs: i32) -> Self::Output {
        Rational::new(self.n * rhs, self.d).unwrap()
    }
}

impl Div<i32> for Rational {
    type Output = Rational;

    fn div(self, rhs: i32) -> Self::Output {
        Rational::new(self.n / rhs, self.d).unwrap()
    }
}

impl Add<Rational> for i32 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        Rational::new((self * rhs.d) + rhs.n, rhs.d).unwrap()
    }
}

impl Sub<Rational> for i32 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        Rational::new((self * rhs.d) - rhs.n, rhs.d).unwrap()
    }
}

impl Mul<Rational> for i32 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        Rational::new(self * rhs.n, rhs.d).unwrap()
    }
}

impl Div<Rational> for i32 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        assert!(rhs.n != 0, "cannot divide by zero!");
        Rational::new(self * rhs.d, rhs.n).unwrap()
    }
}

impl AddAssign<Rational> for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Rational> for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<Rational> for Rational {
    fn mul_assign(&mut self, rhs: Rational) {
        self.n *= rhs.n;
        self.d *= rhs.d;
    }
}

impl DivAssign<Rational> for Rational {
    fn div_assign(&mut self, rhs: Rational) {
        self.n *= rhs.d;
        self.d *= rhs.n;
    }
}
