type N = u16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame(N);
impl Frame {
    /// Returns the inner representation of the frame.
    pub fn inner(&self) -> N {
        self.0
    }
    /// Decrements the current frame.
    pub fn decrement(&self) -> Self {
        Self(self.0.wrapping_sub(1))
    }
    /// Increments the current frame.
    pub fn increment(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}
impl From<u16> for Frame {
    fn from(frame: N) -> Self {
        Self(frame)
    }
}
