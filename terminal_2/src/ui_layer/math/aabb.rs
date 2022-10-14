use super::Vec2d;
use core::ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb2d<N> {
    pub min: Vec2d<N>,
    pub max: Vec2d<N>,
}

impl<N> Add<N> for Aabb2d<N> {
    type Output = Self;

    fn add(self, rhs: N) -> Self::Output {
        todo!()
    }
}

impl<N> Add<Vec2d<N>> for Aabb2d<N> {
    type Output = Self;

    fn add(self, rhs: Vec2d<N>) -> Self::Output {
        todo!()
    }
}
impl<N> AddAssign<N> for Aabb2d<N> {
    fn add_assign(&mut self, rhs: N) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
