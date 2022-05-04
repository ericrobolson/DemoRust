use crate::util::array_2d_to_1d;
use crate::{color::Color, util::array_1d_to_2d};
use crate::{sand::Sand, types::Direction};
use std::cell::RefCell;

const SIZE: usize = 32;

type Storage = Vec<Sand>;

pub struct World {
    is_dirty: RefCell<bool>,
    sand_dirty_bit: bool,
    sand: Storage,
}
impl World {
    /// Creates a new world
    pub fn new() -> Self {
        let sand = vec![Sand::Empty; SIZE * SIZE];

        Self {
            is_dirty: RefCell::new(true),
            sand,
            sand_dirty_bit: true,
        }
    }

    /// Returns the width of the world.
    pub fn width(&self) -> usize {
        SIZE
    }

    /// Returns the height of the world.
    pub fn height(&self) -> usize {
        SIZE
    }

    /// Returns whether the world is dirty since last render.
    pub fn is_dirty(&self) -> bool {
        *self.is_dirty.borrow()
    }

    /// Returns the sand at a given cell.
    pub fn cell(&self, x: usize, y: usize) -> Sand {
        self.sand[array_2d_to_1d(x, y, self.width())]
    }

    /// Puts a grain of sand down
    pub fn put_sand(&mut self, x: usize, y: usize, mut sand: Sand) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        *self.is_dirty.borrow_mut() = true;
        sand.set_dirty_mask(self.sand_dirty_bit);

        let idx = array_2d_to_1d(x, y, SIZE);

        self.sand[idx] = sand;
    }

    /// Returns a index for the cell at a given direction.
    fn get_idx(&self, x: usize, y: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::East => todo!(),
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some(array_2d_to_1d(x, y - 1, self.width()))
                }
            }
            Direction::NorthEast => todo!(),
            Direction::NorthWest => todo!(),
            Direction::South => {
                if y == self.height() - 1 {
                    None
                } else {
                    Some(array_2d_to_1d(x, y + 1, self.width()))
                }
            }
            Direction::SouthEast => {
                if y == self.height() - 1 || x == self.width() - 1 {
                    None
                } else {
                    Some(array_2d_to_1d(x + 1, y + 1, self.width()))
                }
            }
            Direction::SouthWest => {
                if y == self.height() - 1 || x == 0 {
                    None
                } else {
                    Some(array_2d_to_1d(x - 1, y + 1, self.width()))
                }
            }
            Direction::West => todo!(),
        }
    }

    /// Ticks the sim
    pub fn tick(&mut self) {
        /*
        This is still a WIP
        - Need to come up with a way to make rules for sand
        - Need to make dirt work properly
        */
        let mut dirty = false;
        self.sand_dirty_bit = !self.sand_dirty_bit;

        for i in (0..self.sand.len()).rev() {
            let (x, y) = array_1d_to_2d(i, self.width(), self.height());

            let mut cell = self.cell(x, y);

            if cell.dirty_mask() == self.sand_dirty_bit || cell.is_empty() {
                continue;
            }

            for (direction, swap_types) in cell.movement_rules() {
                match self.get_idx(x, y, *direction) {
                    Some(idx) => {
                        let mut tmp = self.sand[idx];
                        if tmp.dirty_mask() == self.sand_dirty_bit && !tmp.is_empty() {
                            continue;
                        }

                        if swap_types.contains(&tmp) {
                            tmp.set_dirty_mask(self.sand_dirty_bit);
                            cell.set_dirty_mask(self.sand_dirty_bit);

                            self.sand[i] = tmp;
                            self.sand[idx] = cell;

                            dirty = true;
                            break;
                        }
                    }
                    None => {}
                }
            }
        }

        if dirty {
            *self.is_dirty.borrow_mut() = true;
        }
    }

    /// Renders the sand.
    pub fn render<'a>(&self) -> SandIterator {
        *self.is_dirty.borrow_mut() = false;

        SandIterator {
            idx: 0,
            width: self.width(),
            height: self.height(),
            items: &self.sand,
        }
    }
}

pub struct SandIterator<'a> {
    idx: usize,
    width: usize,
    height: usize,
    items: &'a Vec<Sand>,
}

impl<'a> Iterator for SandIterator<'a> {
    type Item = SandCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.items.len() {
            let sand = self.items[self.idx];
            let (x, y) = array_1d_to_2d(self.idx, self.width, self.height);
            self.idx += 1;

            Some(SandCell {
                x: x as u32,
                y: y as u32,
                sand,
            })
        } else {
            None
        }
    }
}

/// A single cell of sand
pub struct SandCell {
    pub x: u32,
    pub y: u32,
    pub sand: Sand,
}
