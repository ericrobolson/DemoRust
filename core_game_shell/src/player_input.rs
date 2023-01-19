use crate::math::{map_range, NormalizedF32};

type N = u32;

/*
Use a u64 to represent all bytes.
This way it can be sent over the wire easily.
*/

const FLAG_EMPTY: N = 0b0000_0000_0000_0000;
const AXIS_FLAG: N = 0b1111;

const PRIMARY_X_AXIS_INDEX: N = 0;
const PRIMARY_X_AXIS: N = AXIS_FLAG << PRIMARY_X_AXIS_INDEX;

const PRIMARY_Y_AXIS_INDEX: N = 1 * 4;
const PRIMARY_Y_AXIS: N = AXIS_FLAG << PRIMARY_Y_AXIS_INDEX;

const SECONDARY_X_AXIS_INDEX: N = 2 * 4;
const SECONDARY_X_AXIS: N = AXIS_FLAG << SECONDARY_X_AXIS_INDEX;

const SECONDARY_Y_AXIS_INDEX: N = 3 * 4;
const SECONDARY_Y_AXIS: N = AXIS_FLAG << SECONDARY_Y_AXIS_INDEX;

/// The maximum value that can be held in 4 bits
const AXIS_MIN_VALUE: f32 = 0.0;
/// The minimum value that can be held in 4 bits
const AXIS_MAX_VALUE: f32 = 15.0;

const CLAMP_VALUE: f32 = 0.07;

macro_rules! make_input {
    ($get_id:ident,$set_id:ident => $index:expr, $flag:expr) => {
        pub fn $set_id(&mut self, value: NormalizedF32) {
            self.set_axis($index, $flag, value)
        }

        pub fn $get_id(&self) -> NormalizedF32 {
            self.get_axis($index, $flag)
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerInput(N);
impl PlayerInput {
    make_input!(get_primary_x_axis, set_primary_x_axis => PRIMARY_X_AXIS_INDEX, PRIMARY_X_AXIS);
    make_input!(get_primary_y_axis, set_primary_y_axis => PRIMARY_Y_AXIS_INDEX, PRIMARY_Y_AXIS);

    make_input!(get_secondary_x_axis, set_secondary_x_axis => SECONDARY_X_AXIS_INDEX, SECONDARY_X_AXIS);
    make_input!(get_secondary_y_axis, set_secondary_y_axis => SECONDARY_Y_AXIS_INDEX, SECONDARY_Y_AXIS);

    /// Creates a new instance of a player input.
    pub fn new() -> Self {
        let mut i = Self(0);

        i.set_primary_x_axis(NormalizedF32::ZERO);

        i
    }

    /// Gets some axis bits.
    fn get_axis(&self, index: N, flag: N) -> NormalizedF32 {
        let value = self.0 & flag >> index;
        let value_f32 = value as f32;
        let mapped_value: f32 = map_range(
            value_f32,
            AXIS_MIN_VALUE,
            AXIS_MAX_VALUE,
            NormalizedF32::MIN.inner(),
            NormalizedF32::MAX.inner(),
        );

        let should_clamp_due_to_rounding_errors = mapped_value <= CLAMP_VALUE && mapped_value > 0.0
            || mapped_value < 0.0 && mapped_value >= -CLAMP_VALUE;

        (if should_clamp_due_to_rounding_errors {
            0.0
        } else {
            mapped_value
        })
        .into()
    }

    /// Sets some axis bits.
    fn set_axis(&mut self, index: N, flag: N, value: NormalizedF32) {
        let four_bit_f32 = map_range(
            value.inner(),
            NormalizedF32::MIN.inner(),
            NormalizedF32::MAX.inner(),
            AXIS_MIN_VALUE,
            AXIS_MAX_VALUE,
        );

        let four_bit_n = four_bit_f32 as N;

        self.0 = self.0 & (!flag) | (four_bit_n << index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_x_axis_returns_min() {
        let mut i = PlayerInput::new();
        i.set_primary_x_axis(NormalizedF32::MIN);
        assert_eq!(0, i.0);
    }

    #[test]
    fn get_x_axis_returns_min() {
        let mut i = PlayerInput::new();
        i.set_primary_x_axis(NormalizedF32::MIN);
        assert_eq!(NormalizedF32::MIN, i.get_primary_x_axis());
    }

    #[test]
    fn set_x_axis_returns_max() {
        let mut i = PlayerInput::new();
        i.set_primary_x_axis(NormalizedF32::MAX);
        assert_eq!(15, i.0)
    }
    #[test]
    fn get_x_axis_returns_max() {
        let mut i = PlayerInput::new();
        i.set_primary_x_axis(NormalizedF32::MAX);
        assert_eq!(NormalizedF32::MAX, i.get_primary_x_axis());
    }

    #[test]
    fn set_x_axis_returns_half() {
        let mut i = PlayerInput::new();
        i.set_primary_x_axis(0.0.into());
        assert_eq!(7, i.0)
    }
    #[test]
    fn get_x_axis_returns_half() {
        let mut i = PlayerInput::new();
        i.set_primary_x_axis(0.0.into());
        assert_eq!(NormalizedF32::ZERO, i.get_primary_x_axis());
    }

    #[test]
    fn new_defaults_x_axis_to_half() {
        let i = PlayerInput::new();
        assert_eq!(NormalizedF32::ZERO, i.get_primary_x_axis())
    }
}
