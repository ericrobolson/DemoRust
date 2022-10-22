/// Maps a value from one numerical range to another.
pub fn map_to_range<N>(value: N, src_min: N, src_max: N, dest_min: N, dest_max: N) -> N
where
    N: core::ops::Sub<Output = N>
        + Copy
        + core::ops::Div<Output = N>
        + core::ops::Mul<Output = N>
        + core::ops::Add<Output = N>,
{
    (dest_max - dest_min) / (src_max - src_min) * (value - src_min) + dest_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_to_range_case_a() {
        let value = 10;
        let src_min = 0;
        let src_max = 10;
        let dest_min = 0;
        let dest_max = 100;

        let result = map_to_range(value, src_min, src_max, dest_min, dest_max);
        assert_eq!(100, result)
    }

    #[test]
    fn map_to_range_case_b() {
        let value = 0;
        let src_min = 0;
        let src_max = 5;
        let dest_min = 0;
        let dest_max = 20;

        let result = map_to_range(value, src_min, src_max, dest_min, dest_max);
        assert_eq!(0, result)
    }

    #[test]
    fn map_to_range_case_c() {
        let value = 0;
        let src_min = -10;
        let src_max = 10;
        let dest_min = 0;
        let dest_max = 20;

        let result = map_to_range(value, src_min, src_max, dest_min, dest_max);
        assert_eq!(10, result)
    }
}
