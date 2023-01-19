pub use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TickRate(Seconds);
impl TickRate {
    /// Returns the seconds value in a F32.
    pub fn to_f32_seconds(&self) -> f32 {
        self.0.inner()
    }
}
impl From<f32> for TickRate {
    fn from(s: f32) -> Self {
        Self(s.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_f32() {
        let t: TickRate = 0.1.into();
    }
}
