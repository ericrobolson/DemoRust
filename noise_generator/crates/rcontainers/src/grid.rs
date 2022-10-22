use rmath::index_1d_to_2d;

/// A grid container.
#[derive(Debug, Clone)]
pub struct Grid<T>
where
    T: Clone,
{
    items: Vec<T>,
    width: usize,
    height: usize,
}
impl<T> Grid<T>
where
    T: Clone,
{
    /// Creates a new grid.
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let mut items = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            items.push(default.clone());
        }

        Self {
            items,
            width,
            height,
        }
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns safely mapped coordinates for the grid.
    fn coordinates(&self, x: usize, y: usize) -> (usize, usize) {
        (x % self.width, y % self.height)
    }

    /// Returns the item at the given coordinates.
    pub fn item(&self, x: usize, y: usize) -> &T {
        let (x, y) = self.coordinates(x, y);
        let idx = rmath::index_2d_to_1d(x, y, self.width, self.height);
        &self.items[idx]
    }

    /// Returns the item at the given coordinates.
    pub fn item_mut(&mut self, x: usize, y: usize) -> &mut T {
        let (x, y) = self.coordinates(x, y);
        let idx = rmath::index_2d_to_1d(x, y, self.width, self.height);
        &mut self.items[idx]
    }

    /// Returns an iterator of items and their coordinates.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = ((usize, usize), &T)> + 'a {
        self.items
            .iter()
            .enumerate()
            .map(|(idx, item)| (index_1d_to_2d(idx, self.width, self.height), item))
    }

    /// Returns an iterator of mutable items and their coordinates.
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> + 'a {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(idx, item)| (index_1d_to_2d(idx, self.width, self.height), item))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn width_returns_width() {
        let w = 128;
        let h = 182;
        let grid = Grid::<bool>::new(w, h, false);

        assert_eq!(w, grid.width());
    }

    #[test]
    fn height_returns_height() {
        let w = 128;
        let h = 182;
        let grid = Grid::<bool>::new(w, h, false);

        assert_eq!(h, grid.height());
    }
}
