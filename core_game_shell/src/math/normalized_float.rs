/// Normalized float from -1 to 1
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalizedF32(f32);
impl NormalizedF32 {
    pub const MAX: Self = Self(1.0);
    pub const MIN: Self = Self(-1.0);
    pub const ZERO: Self = Self(0.0);

    pub fn inner(&self) -> f32 {
        self.0
    }
}

impl From<f32> for NormalizedF32 {
    fn from(f: f32) -> Self {
        Self(if f < -1.0 {
            -1.0
        } else if f > 1.0 {
            1.0
        } else {
            f
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_below_n1_returns_n1() {
        let actual: NormalizedF32 = (-1.001).into();
        assert_eq!(-1.0, actual.0)
    }

    #[test]
    fn f_over_1_returns_1() {
        let actual: NormalizedF32 = (1.001).into();
        assert_eq!(1.0, actual.0)
    }

    #[test]
    fn f_returns_f() {
        let actual: NormalizedF32 = (0.001).into();
        assert_eq!(0.001, actual.0)
    }
}
