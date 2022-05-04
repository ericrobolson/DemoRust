use std::ops::{Add, Div, Mul, Range, Rem};

/// Converts a 2d point to a 1d index
pub fn array_2d_to_1d<N>(x: N, y: N, width: N) -> N
where
    N: Mul<Output = N> + Add<Output = N> + Div<Output = N> + Rem<Output = N> + Copy,
{
    width * x + y
}

/// Converts a 1d array to a 2d point
pub fn array_1d_to_2d<N>(idx: N, width: N, height: N) -> (N, N)
where
    N: Mul<Output = N> + Add<Output = N> + Div<Output = N> + Rem<Output = N> + Copy,
{
    (idx / width, idx % height)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<N, const WIDTH: usize, const HEIGHT: usize>
where
    N: Clone + core::fmt::Debug + Default + PartialEq,
{
    objects: Vec<N>,
}

impl<N, const WIDTH: usize, const HEIGHT: usize> Grid<N, WIDTH, HEIGHT>
where
    N: Clone + core::fmt::Debug + Default + PartialEq,
{
    /// Creates a new grid.
    pub fn new() -> Self {
        let mut objects = Vec::with_capacity(WIDTH * HEIGHT);
        for _ in 0..WIDTH * HEIGHT {
            objects.push(N::default());
        }

        Self { objects }
    }

    fn safeguard(x: usize, y: usize) -> (usize, usize) {
        let x = x % WIDTH;
        let y = y % HEIGHT;
        (x, y)
    }

    pub fn item(&self, x: usize, y: usize) -> &N {
        let (x, y) = Self::safeguard(x, y);

        let idx = array_2d_to_1d(x, y, WIDTH);
        &self.objects[idx]
    }

    pub fn item_mut(&mut self, x: usize, y: usize) -> &mut N {
        let (x, y) = Self::safeguard(x, y);

        let idx = array_2d_to_1d(x, y, HEIGHT);
        &mut self.objects[idx]
    }

    pub fn width(&self) -> usize {
        WIDTH
    }

    pub fn xrange(&self) -> Range<usize> {
        0..WIDTH
    }

    pub fn yrange(&self) -> Range<usize> {
        0..HEIGHT
    }

    pub fn height(&self) -> usize {
        HEIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_is_bijective() {
        let x_start = 0;
        let y_start = 0;
        let width = 2;
        let height = 4;

        let output_1d = array_2d_to_1d(x_start, y_start, width);

        let output_2d = array_1d_to_2d(output_1d, width, height);

        assert_eq!((x_start, y_start), output_2d);
    }
}
