mod movement;
pub use movement::*;

use super::PositiveNumber;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Characteristic {
    /// Lifting capacity + base hth damage
    Strength,
    /// who acts first in combat and dex based rolls
    Dexterity,
    /// determines if a character is stunned in combat and provides con rolls
    Constitution,
    Intelligence,
    Ego,
    Presence,
    OffensiveCombatValue,
    DefensiveCombatValue,
    OffensiveMentalCombatValue,
    DefensiveMentalCombatValue,
    Speed,
    PhysicalDefense,
    EnergyDefense,
    Recovery,
    Endurance,
    Body,
    Stun,
}
impl Characteristic {
    pub fn base_value(&self) -> PositiveNumber {
        match self {
            Characteristic::Strength => 10,
            Characteristic::Dexterity => 10,
            Characteristic::Constitution => 10,
            Characteristic::Intelligence => 10,
            Characteristic::Ego => 10,
            Characteristic::Presence => 10,
            Characteristic::OffensiveCombatValue => 3,
            Characteristic::DefensiveCombatValue => 3,
            Characteristic::OffensiveMentalCombatValue => 3,
            Characteristic::DefensiveMentalCombatValue => 3,
            Characteristic::Speed => 2,
            Characteristic::PhysicalDefense => 2,
            Characteristic::EnergyDefense => 2,
            Characteristic::Recovery => 4,
            Characteristic::Endurance => 20,
            Characteristic::Body => 10,
            Characteristic::Stun => 20,
        }
        .into()
    }

    pub fn cost(&self) -> CharacteristicCost {
        match self {
            Characteristic::Strength => (1, 1),
            Characteristic::Dexterity => (2, 1),
            Characteristic::Constitution => (1, 1),
            Characteristic::Intelligence => (1, 1),
            Characteristic::Ego => (1, 1),
            Characteristic::Presence => (1, 1),
            Characteristic::OffensiveCombatValue => (5, 1),
            Characteristic::DefensiveCombatValue => (5, 1),
            Characteristic::OffensiveMentalCombatValue => (3, 1),
            Characteristic::DefensiveMentalCombatValue => (3, 1),
            Characteristic::Speed => (10, 1),
            Characteristic::PhysicalDefense => (1, 1),
            Characteristic::EnergyDefense => (1, 1),
            Characteristic::Recovery => (1, 1),
            Characteristic::Endurance => (1, 5),
            Characteristic::Body => (1, 1),
            Characteristic::Stun => (1, 2),
        }
        .into()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CharacterPoints(PositiveNumber);
impl CharacterPoints {
    pub fn inner(&self) -> u64 {
        self.0.inner()
    }
}
impl From<u64> for CharacterPoints {
    fn from(p: u64) -> Self {
        Self(p.into())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CharacteristicCost {
    character_points: CharacterPoints,
    points_gained: CharacterPoints,
}
impl CharacteristicCost {
    pub fn character_points(&self) -> CharacterPoints {
        self.character_points
    }
    pub fn points_gained(&self) -> CharacterPoints {
        self.points_gained
    }
}
impl From<(u64, u64)> for CharacteristicCost {
    fn from((character_points, points_gained): (u64, u64)) -> Self {
        Self {
            character_points: character_points.into(),
            points_gained: points_gained.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
