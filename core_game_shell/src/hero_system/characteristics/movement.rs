use crate::hero_system::Meters;

use super::CharacterPoints;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MovementType {
    Running,
    Swimming,
    Leaping,
}

impl MovementType {
    pub fn base_value(&self) -> Meters {
        match self {
            MovementType::Running => 12,
            MovementType::Swimming => 4,
            MovementType::Leaping => 4,
        }
        .into()
    }

    pub fn cost(&self) -> MovementCost {
        match self {
            MovementType::Running => (1, 1),
            MovementType::Swimming => (1, 2),
            MovementType::Leaping => (1, 2),
        }
        .into()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MovementCost {
    character_points: CharacterPoints,
    meters_gained: Meters,
}
impl MovementCost {
    pub fn character_points(&self) -> CharacterPoints {
        self.character_points
    }
    pub fn meters_gained(&self) -> Meters {
        self.meters_gained
    }
}
impl From<(u64, i64)> for MovementCost {
    fn from((character_points, meters_gained): (u64, i64)) -> Self {
        Self {
            character_points: character_points.into(),
            meters_gained: meters_gained.into(),
        }
    }
}
