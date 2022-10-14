/// A number type.
/// Provides deterministic and non-deterministic functionality.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct N(f32);
impl N {
    /// DETERMINISTIC
    /// Returns a fast inverse square root.
    pub fn inv_square_root(&self) -> Self {
        Self(fast_inv_square_root(self.0))
    }

    /// NONDETERMINISTIC
    /// Returns the inverse square root.
    pub fn inv_square_root_nondeterministic(&self) -> Self {
        Self(1.0 / self.0.sqrt())
    }

    /// DETERMINISTIC
    /// Returns a square root.
    pub fn sqrt(&self) -> Self {
        Self(sqrt(self.0))
    }

    /// NONDETERMINISTIC
    /// Returns a square root.
    pub fn sqrt_nondeterministic(&self) -> Self {
        Self(self.0.sqrt())
    }
}

/// Returns a fast inverse square root.
/// Based off Quake.
fn fast_inv_square_root(number: f32) -> f32 {
    const THREE_HALFS: f32 = 1.5;

    let xhalf = number * 0.5;

    let i: i32 = unsafe { core::mem::transmute(number) };
    let i = 0x5f3759df - (i >> 1);
    let x: f32 = unsafe { core::mem::transmute(i) };
    let x = x * (THREE_HALFS - xhalf * x * x);
    x
}

/// Returns a square root approximation.
/// Based on fast_inv_square_root()
fn sqrt(number: f32) -> f32 {
    1.0 / fast_inv_square_root(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_4_returns_2() {
        // let expected = 2.0;
        let expected = 2.0033915;
        let actual = sqrt(4.0);
        assert_eq!(expected, actual)
    }

    #[test]
    fn sqrt_9_returns_3() {
        // let expected = 3.0;
        let expected = 3.003425;
        let actual = sqrt(9.0);
        assert_eq!(expected, actual)
    }

    #[test]
    fn fast_inv_square_root_sample_48() {
        let n: f32 = 48.0;
        // let expected: f32 = 1.0 / n.sqrt();
        let expected: f32 = 0.14421171;
        let actual = fast_inv_square_root(n);
        assert_eq!(expected, actual)
    }

    #[test]
    fn fast_inv_square_root_sample_4() {
        let n: f32 = 4.0;

        // let expected: f32 = 1.0 / n.sqrt();
        let expected: f32 = 0.49915358;
        let actual = fast_inv_square_root(n);
        assert_eq!(expected, actual)
    }
}
