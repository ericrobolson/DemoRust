use core::ops::{Add, Mul, MulAssign, Sub};

/// Deterministic number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct DNum(i64);

impl From<i64> for DNum {
    fn from(n: i64) -> Self {
        Self(n)
    }
}
impl Add for DNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for DNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for DNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Mul<DNum> for i64 {
    type Output = DNum;

    fn mul(self, rhs: DNum) -> Self::Output {
        DNum(self * rhs.0)
    }
}
impl Mul<i64> for DNum {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl MulAssign for DNum {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl MulAssign<i64> for DNum {
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_assign_assigns() {
        let mut a: DNum = 10.into();
        let b: DNum = 120.into();
        let expected: DNum = (10 * 120).into();
        a *= b;
        assert_eq!(expected, a)
    }
    #[test]
    fn mul_assign_assigns_i64() {
        let mut a: DNum = 10.into();
        let b: i64 = 120.into();
        let expected: DNum = (10 * 120).into();
        a *= b;
        assert_eq!(expected, a)
    }

    #[test]
    fn mul_muls() {
        let a: DNum = 10.into();
        let b: DNum = 120.into();
        let expected: DNum = (10 * 120).into();
        assert_eq!(expected, a * b)
    }
    #[test]
    fn mul_muls_i64() {
        let a: DNum = 10.into();
        let b: i64 = 120.into();
        let expected: DNum = (10 * 120).into();
        assert_eq!(expected, a * b)
    }
    #[test]
    fn mul_muls_i64_start() {
        let a: i64 = 10.into();
        let b: DNum = 120.into();
        let expected: DNum = (10 * 120).into();
        assert_eq!(expected, a * b)
    }

    #[test]
    fn add_adds() {
        let a: DNum = 10.into();
        let b: DNum = 120.into();
        let expected: DNum = (10 + 120).into();
        assert_eq!(expected, a + b)
    }
    #[test]
    fn sub_subs() {
        let a: DNum = 10.into();
        let b: DNum = 120.into();
        let expected: DNum = (10 - 120).into();
        assert_eq!(expected, a - b)
    }
}
