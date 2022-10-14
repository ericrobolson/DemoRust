pub fn map_range<N>(value: N, source_min: N, source_max: N, dest_min: N, dest_max: N) -> N
where
    N: core::ops::Sub<Output = N>
        + core::ops::Mul<Output = N>
        + core::ops::Div<Output = N>
        + core::ops::Add<Output = N>
        + Copy,
{
    (value - source_min) * (dest_max - dest_min) / (source_max - source_min) + dest_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_range_10_to_100() {
        let value = 10;
        let source_min = 0;
        let source_max = 10;
        let dest_min = 0;
        let dest_max = 100;

        let expected = 100;

        let result = map_range(value, source_min, source_max, dest_min, dest_max);
        assert_eq!(expected, result);
    }
}
