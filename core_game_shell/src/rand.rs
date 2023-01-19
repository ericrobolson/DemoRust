use crate::math::map_range;

type N = u64;
pub struct Rng(N);

impl Rng {
    pub fn new(seed: N) -> Self {
        let mut x = seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        Self(x)
    }

    pub fn u8(&mut self) -> u8 {
        let n = self.0 % (u8::MAX as N);
        *self = Self::new(self.0);

        n as u8
    }

    pub fn f32(&mut self) -> f32 {
        let n = self.0 as f32;
        let n = n / (N::MAX as f32);
        *self = Self::new(self.0);

        n
    }
}

#[test]
fn stuff() {
    let mut a = Rng::new(123);

    let mut x = vec![];
    for _ in 0..100 {
        x.push(a.f32())
    }

    assert_eq!(vec![0.0], x)
}
