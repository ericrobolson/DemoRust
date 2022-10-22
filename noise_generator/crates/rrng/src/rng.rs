/// A representation of a random number generator.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Rng(u32);
impl Rng {
    /// Creates a new instance of an RNG.
    pub fn new(seed: u32) -> Self {
        Self(seed)
    }

    /// Returns the next value in the RNG sequence.
    pub fn next(&mut self) -> u32 {
        // https://stackoverflow.com/a/3062783
        // https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use

        const MODULUS: u32 = u32::MAX;
        const MULTIPLIER: u32 = 1664525;
        const INCREMENT: u32 = 1013904223;

        let n = (MULTIPLIER.wrapping_mul(self.0).wrapping_add(INCREMENT)) % MODULUS;

        self.0 = n;

        return n;
    }

    /// Returns a normalized 0..1 random value.
    /// Not guaranteed to be deterministic.
    pub fn next_f32(&mut self) -> f32 {
        let n = self.next() as f32;

        n / u32::MAX as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_next_returns_random_number() {
        let mut r = Rng::new(300);

        assert_eq!(1514926248, r.next());
        assert_eq!(2494464500, r.next());
    }
}
