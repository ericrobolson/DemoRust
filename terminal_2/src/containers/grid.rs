/// A reference to a cell in a grid.
#[derive(PartialEq)]
pub struct Cell<'a, T> {
    pub x: usize,
    pub y: usize,
    pub item: &'a T,
}

/// A reference to a mutable cell in a grid.
pub struct CellMut<'a, T> {
    pub x: usize,
    pub y: usize,
    pub item: &'a mut T,
}

/// A grid of elements
#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T> {
    items: Vec<T>,
    width: usize,
    height: usize,
}
impl<T> Grid<T>
where
    T: Default + Clone,
{
    /// Creates a new grid of the given size
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            items: vec![T::default(); width * height],
            width,
            height,
        }
    }

    /// Converts a 2d index to a 1d index
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(self.width * y + x)
        } else {
            None
        }
    }

    /// Returns a list of the items with their coordinates.
    pub fn iter(&self) -> impl Iterator<Item = Cell<T>> {
        self.items.iter().enumerate().map(|(idx, item)| {
            let x = idx % self.width;
            let y = idx / self.width;

            Cell { x, y, item }
        })
    }

    /// Returns a list of the items with their coordinates.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = CellMut<T>> {
        self.items.iter_mut().enumerate().map(|(idx, item)| {
            let x = idx % self.width;
            let y = idx / self.width;
            CellMut { x, y, item }
        })
    }

    /// Returns the given item.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        match self.index(x, y) {
            Some(idx) => Some(&self.items[idx]),
            None => None,
        }
    }

    /// Sets the given item
    pub fn set(&mut self, x: usize, y: usize, item: T) {
        match self.index(x, y) {
            Some(idx) => self.items[idx] = item,
            None => {}
        }
    }

    /// Returns a mutable handle to the given item.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        match self.index(x, y) {
            Some(idx) => Some(&mut self.items[idx]),
            None => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_sets_first_item() {
        let w = 4;
        let h = 4;
        let mut g = Grid::<bool>::new(w, h);
        g.set(0, 0, true);

        assert_eq!(true, g.items[0]);
    }

    #[test]
    fn grid_sets_properly() {
        let w = 4;
        let h = 4;
        let mut g = Grid::<(usize, usize)>::new(w, h);
        for x in 0..w {
            for y in 0..h {
                g.set(x, y, (x, y));
            }
        }

        for x in 0..w {
            for y in 0..h {
                assert_eq!(Some(&(x, y)), g.get(x, y));
            }
        }
    }
}
