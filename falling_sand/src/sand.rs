use crate::{color::Color, types::Direction};

const EMPTY: u8 = 0;
const DIRT: u8 = EMPTY + 1;
const WATER: u8 = DIRT + 1;

const DIRTY_MASK: u8 = 0b1000_0000;
const MOVE_LEFT_MASK: u8 = 0b0100_0000;

const MASK_BITS: u8 = DIRTY_MASK | MOVE_LEFT_MASK;

#[derive(Clone, Debug, Copy)]
pub struct Sand(u8);
impl Sand {
    pub const Empty: Self = Self(EMPTY);
    pub const Dirt: Self = Self(DIRT);
    pub const Water: Self = Self(WATER);
}

impl PartialEq for Sand {
    fn eq(&self, other: &Self) -> bool {
        // Zero out the dirty mask
        let mask = !MASK_BITS;
        self.0 & mask == other.0 & mask
    }
}

impl Sand {
    /// Returns the dirty mask value for the piece of sand.
    pub fn dirty_mask(&self) -> bool {
        (self.0 & DIRTY_MASK) > 0
    }

    fn inner(&self) -> u8 {
        self.0 & (!MASK_BITS)
    }

    pub fn move_left(&self) -> bool {
        (self.0 & MOVE_LEFT_MASK) > 0
    }

    pub fn set_move_left(&mut self, left: bool) {
        let value = if left {
            self.0 | MOVE_LEFT_MASK
        } else {
            !MOVE_LEFT_MASK & self.0
        };

        self.0 = value;
    }

    /// Sets the dirty mask value for the piece of sand.
    pub fn set_dirty_mask(&mut self, dirty: bool) {
        let value = if dirty {
            self.0 | DIRTY_MASK
        } else {
            !DIRTY_MASK & self.0
        };

        self.0 = value;
    }

    /// Returns whether the sand is empty or not.
    pub fn is_empty(&self) -> bool {
        self.0 == EMPTY
    }

    /// Returns the color of the sand.
    pub fn color(&self) -> Color {
        match self.inner() {
            DIRT => (128, 100, 85).into(),
            WATER => (0, 255, 0).into(),
            EMPTY | _ => (0, 0, 0, 0).into(),
        }
    }

    /// Returns the movement rules for the given cell.
    pub fn movement_rules(&self) -> &'static [(Direction, &'static [Sand])] {
        match self.inner() {
            EMPTY => &[],
            WATER => {
                if self.move_left() {
                    &[
                        (Direction::South, &[Sand::Empty]),
                        (Direction::SouthWest, &[Sand::Empty]),
                        (Direction::SouthEast, &[Sand::Empty]),
                    ]
                } else {
                    &[
                        (Direction::South, &[Sand::Empty]),
                        (Direction::SouthEast, &[Sand::Empty]),
                        (Direction::SouthWest, &[Sand::Empty]),
                    ]
                }
            }
            _ => {
                if self.move_left() {
                    &[
                        (Direction::South, &[Sand::Empty, Sand::Water]),
                        (Direction::SouthWest, &[Sand::Empty, Sand::Water]),
                        (Direction::SouthEast, &[Sand::Empty, Sand::Water]),
                    ]
                } else {
                    &[
                        (Direction::South, &[Sand::Empty, Sand::Water]),
                        (Direction::SouthEast, &[Sand::Empty, Sand::Water]),
                        (Direction::SouthWest, &[Sand::Empty, Sand::Water]),
                    ]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_dirty_sets_to_true() {
        let mut v = Sand(0);
        v.set_dirty_mask(true);

        assert_eq!(0b1000_0000, v.0);
        assert_eq!(true, v.dirty_mask());

        v.set_dirty_mask(true);

        assert_eq!(0b1000_0000, v.0);
        assert_eq!(true, v.dirty_mask());
    }

    #[test]
    fn set_dirty_sets_to_false() {
        let mut v = Sand(u8::MAX);
        v.set_dirty_mask(false);

        assert_eq!(!0b1000_0000, v.0);
        assert_eq!(false, v.dirty_mask());

        v.set_dirty_mask(false);

        assert_eq!(!0b1000_0000, v.0);
        assert_eq!(false, v.dirty_mask());
    }

    #[test]
    fn dirty_bit_empty() {
        let mut cell = Sand::Empty;

        cell.set_dirty_mask(true);
        assert_eq!(0b1000_0000, cell.0);

        cell.set_dirty_mask(false);
        assert_eq!(0b0000_0000, cell.0);
    }

    #[test]
    fn dirty_bit_dirt() {
        let mut cell = Sand::Dirt;

        cell.set_dirty_mask(true);
        assert_eq!(DIRT | DIRTY_MASK, cell.0);

        cell.set_dirty_mask(false);
        assert_eq!(DIRT, cell.0);
    }
}
