use crate::{
    containers::{Cell, CellMut, Grid},
    style,
};

pub struct RenderCell<'a, T> {
    pub x: usize,
    pub y: usize,
    pub item: &'a T,
}

pub struct RenderView(Grid<style::StyledCell>);
impl RenderView {
    pub fn new(w: usize, h: usize) -> Self {
        Self(Grid::new(w, h))
    }
    pub fn get(&self, x: usize, y: usize) -> Option<Cell<style::StyledCell>> {
        match self.0.get(x, y) {
            Some(item) => Some(Cell { x, y, item }),
            None => None,
        }
    }
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Cell<style::StyledCell>> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = CellMut<style::StyledCell>> {
        self.0.iter_mut()
    }
}
