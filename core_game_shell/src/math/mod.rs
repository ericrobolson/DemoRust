mod dnum;
mod normalized_float;
pub mod sequences;

pub use dnum::*;

pub use normalized_float::*;

/// Returns the minimum of two numbers.
#[inline(always)]
pub fn min<N>(a: N, b: N) -> N
where
    N: core::cmp::PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

/// Returns the maximum of two numbers.
#[inline(always)]
pub fn max<N>(a: N, b: N) -> N
where
    N: core::cmp::PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

/// Maps one numeric range to another.
#[inline(always)]
pub fn map_range<N>(value: N, from_min: N, from_max: N, to_min: N, to_max: N) -> N
where
    N: Copy
        + core::ops::Sub<Output = N>
        + core::ops::Mul<Output = N>
        + core::ops::Div<Output = N>
        + core::ops::Add<Output = N>,
{
    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_range_maps_properly_case_a() {
        let value = -1.0;
        let from_min = -1.0;
        let from_max = 1.0;
        let to_min = 0.0;
        let to_max = 15.0;
        assert_eq!(0.0, map_range(value, from_min, from_max, to_min, to_max))
    }
    #[test]
    fn map_range_maps_properly_case_b() {
        let value = 0.0;
        let from_min = -1.0;
        let from_max = 1.0;
        let to_min = 0.0;
        let to_max = 15.0;
        assert_eq!(7.5, map_range(value, from_min, from_max, to_min, to_max))
    }
    #[test]
    fn map_range_maps_properly_case_c() {
        let value = 1.0;
        let from_min = -1.0;
        let from_max = 1.0;
        let to_min = 0.0;
        let to_max = 15.0;
        assert_eq!(15.0, map_range(value, from_min, from_max, to_min, to_max))
    }

    #[test]
    fn map_range_succeeds_for_same_range() {
        let value = 0;
        let from_min = 0;
        let from_max = 16;
        let to_min = 0;
        let to_max = 16;
        assert_eq!(0, map_range(value, from_min, from_max, to_min, to_max))
    }
    #[test]
    fn map_range_maps_to_larger_range() {
        let value = 8;
        let from_min = 0;
        let from_max = 16;
        let to_min = 0;
        let to_max = 32;
        assert_eq!(16, map_range(value, from_min, from_max, to_min, to_max))
    }
    #[test]
    fn map_range_maps_to_smaller_range() {
        let value = 8;
        let from_min = 0;
        let from_max = 16;
        let to_min = 0;
        let to_max = 8;
        assert_eq!(4, map_range(value, from_min, from_max, to_min, to_max))
    }

    #[test]
    fn map_range_maps_to_negative_range() {
        let value = 8;
        let from_min = 0;
        let from_max = 16;
        let to_min = -127;
        let to_max = 128;
        assert_eq!(0, map_range(value, from_min, from_max, to_min, to_max))
    }

    #[test]
    fn min_returns_min() {
        let a = 0.1;
        let b = 0.0;

        assert_eq!(b, min(a, b))
    }

    #[test]
    fn max_returns_max() {
        let a = 0.1;
        let b = 0.3;

        assert_eq!(b, max(a, b))
    }
}
