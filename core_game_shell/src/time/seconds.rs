use core::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Seconds(f32);
impl Seconds {
    /// Returns the seconds value in a F32.
    pub fn inner(&self) -> f32 {
        self.0
    }
}
impl From<f32> for Seconds {
    fn from(t: f32) -> Self {
        Self(t)
    }
}
impl Add for Seconds {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_returns_inner() {
        let a = Seconds(34.0);
        assert_eq!(34.0, a.inner())
    }

    #[test]
    fn from_f32() {
        let a: Seconds = 1.0.into();
        assert_eq!(Seconds(1.0), a)
    }

    #[test]
    fn add() {
        let a: Seconds = 1.0.into();
        let b: Seconds = 2.0.into();
        assert_eq!(Seconds(3.0), a + b)
    }
}
