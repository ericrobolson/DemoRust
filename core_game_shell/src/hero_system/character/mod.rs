use core::ops::Rem;

use super::{
    characteristics::{CharacterPoints, Characteristic},
    Kilograms, PositiveNumber, Quantity, D6,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UseResult {
    Success,
    Unable {
        required: PositiveNumber,
        actual: PositiveNumber,
        characteristic_needed: Characteristic,
    },
}

pub struct Character {
    characteristics: Characteristics,
    movement_characteristics: MovementCharacteristics,
}
impl Character {
    /// Returns the lifting capacity of the character.
    pub fn lifting_capacity(&self) -> Kilograms {
        let value = self.characteristics.strength.points.inner() * 20;
        value.into()
    }

    /// Attempts to use strength for the given value.
    pub fn use_strength(&mut self, value: PositiveNumber) -> UseResult {
        let end_required = value / 10;
        if end_required > self.characteristics.endurance.active_value {
            UseResult::Unable {
                required: end_required,
                actual: self.characteristics.endurance.active_value,
                characteristic_needed: Characteristic::Endurance,
            }
        } else {
            self.characteristics.endurance.active_value -= end_required;
            UseResult::Success
        }
    }

    /// Returns the number of damage dice this character rolls.
    pub fn damage(&self) -> Quantity<D6> {
        let num_dice = self.characteristics.strength.points.inner() / 5;
        Quantity {
            number: num_dice.into(),
            item: D6,
        }
    }
}

struct CharacteristicValue {
    points: CharacterPoints,
    active_value: PositiveNumber,
}
impl From<(u64, u64)> for CharacteristicValue {
    fn from((points, active_value): (u64, u64)) -> Self {
        Self {
            points: points.into(),
            active_value: active_value.into(),
        }
    }
}

struct Characteristics {
    strength: CharacteristicValue,
    dexterity: CharacteristicValue,
    constitution: CharacteristicValue,
    intelligence: CharacteristicValue,
    ego: CharacteristicValue,
    presence: CharacteristicValue,
    offensive_combat_value: CharacteristicValue,
    defensive_combat_value: CharacteristicValue,
    offensive_mental_combat_value: CharacteristicValue,
    defensive_mental_comat_value: CharacteristicValue,
    speed: CharacteristicValue,
    physical_defense: CharacteristicValue,
    energy_defense: CharacteristicValue,
    recovery: CharacteristicValue,
    endurance: CharacteristicValue,
    body: CharacteristicValue,
    stun: CharacteristicValue,
}

struct MovementCharacteristics {
    running: CharacteristicValue,
    swimming: CharacteristicValue,
    leaping: CharacteristicValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn character() -> Character {
        Character {
            characteristics: Characteristics {
                strength: (10, 10).into(),
                dexterity: (10, 10).into(),
                constitution: (10, 10).into(),
                intelligence: (10, 10).into(),
                ego: (10, 10).into(),
                presence: (10, 10).into(),
                offensive_combat_value: (10, 10).into(),
                defensive_combat_value: (10, 10).into(),
                offensive_mental_combat_value: (10, 10).into(),
                defensive_mental_comat_value: (10, 10).into(),
                speed: (10, 10).into(),
                physical_defense: (10, 10).into(),
                energy_defense: (10, 10).into(),
                recovery: (10, 10).into(),
                endurance: (10, 10).into(),
                body: (10, 10).into(),
                stun: (10, 10).into(),
            },
            movement_characteristics: MovementCharacteristics {
                running: (12, 10).into(),
                swimming: (4, 4).into(),
                leaping: (4, 4).into(),
            },
        }
    }

    #[test]
    fn damage() {
        let mut c = character();
        c.characteristics.strength.points = 10.into();

        assert_eq!(
            Quantity {
                item: D6,
                number: 2.into(),
            },
            c.damage()
        );

        c.characteristics.strength.points = 50.into();

        assert_eq!(
            Quantity {
                item: D6,
                number: 10.into(),
            },
            c.damage()
        )
    }

    #[test]
    fn lifting_capacity_0pts() {
        let mut c = character();
        c.characteristics.strength.points = 0.into();
        let expected: Kilograms = 0.into();

        assert_eq!(expected, c.lifting_capacity());
    }

    #[test]
    fn lifting_capacity_10pts() {
        let mut c = character();
        c.characteristics.strength.points = 10.into();
        let expected: Kilograms = 200.into();

        assert_eq!(expected, c.lifting_capacity());
    }

    #[test]
    fn lifting_capacity_30pts() {
        let mut c = character();
        c.characteristics.strength.points = 30.into();
        let expected: Kilograms = 600.into();

        assert_eq!(expected, c.lifting_capacity());
    }

    #[test]
    fn use_strength_success() {
        let mut c = character();
        let strength = 30.into();
        c.characteristics.strength.points = strength;
        c.characteristics.strength.points = strength;
        let value = 35.into();
        let expected = UseResult::Success;
        assert_eq!(expected, c.use_strength(value));
    }

    #[test]
    fn use_strength_cant_be_done() {
        let mut c = character();
        let strength = 30.into();
        c.characteristics.strength.points = strength;
        c.characteristics.endurance.active_value = 2.into();
        let value = 35.into();
        let expected = UseResult::Unable {
            required: 3.into(),
            actual: 2.into(),
            characteristic_needed: Characteristic::Endurance,
        };
        assert_eq!(expected, c.use_strength(value));
    }
}
