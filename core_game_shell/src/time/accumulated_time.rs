use super::Seconds;
use core::ops::AddAssign;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccumulatedTime(Seconds);
impl AddAssign<Seconds> for AccumulatedTime {
    fn add_assign(&mut self, rhs: Seconds) {
        self.0 = rhs + self.0;
    }
}
impl From<f32> for AccumulatedTime {
    fn from(f: f32) -> Self {
        Self(f.into())
    }
}
impl AccumulatedTime {
    /// Returns the seconds value in a F32.
    pub fn to_f32_seconds(&self) -> f32 {
        self.0.inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assign() {
        let mut a: AccumulatedTime = 1.0.into();
        let s: Seconds = 2.0.into();
        a += s;
        assert_eq!(AccumulatedTime(3.0.into()), a)
    }

    #[test]
    fn from_f32() {
        let a: Seconds = 1.0.into();
        let actual: AccumulatedTime = 1.0.into();
        assert_eq!(AccumulatedTime(1.0.into()), actual)
    }
}
