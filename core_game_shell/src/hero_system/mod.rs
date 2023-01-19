use core::ops::{Div, SubAssign};

use self::characteristics::CharacterPoints;

mod character;
mod characteristics;

#[derive(Clone, Copy, PartialEq, Debug, Eq, PartialOrd, Ord)]
pub struct PositiveNumber(u64);
impl PositiveNumber {
    pub fn inner(&self) -> u64 {
        self.0
    }
}
impl SubAssign for PositiveNumber {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Div<u64> for PositiveNumber {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl From<u64> for PositiveNumber {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Number(i64);
impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Self(n)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Meters(Number);
impl From<i64> for Meters {
    fn from(n: i64) -> Self {
        Self(n.into())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Kilograms(Number);
impl From<i32> for Kilograms {
    fn from(i: i32) -> Self {
        (i as i64).into()
    }
}
impl From<i64> for Kilograms {
    fn from(n: i64) -> Self {
        Self(n.into())
    }
}
impl From<u64> for Kilograms {
    fn from(n: u64) -> Self {
        Self((n as i64).into())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quantity<T> {
    pub number: PositiveNumber,
    pub item: T,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct D6;

/// A value that one has to roll under.
pub fn characteristic_roll(characteristic: CharacterPoints) -> PositiveNumber {
    (9 + (characteristic.inner() / 5)).into()
}
