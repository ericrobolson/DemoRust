/// Maps a value from one numerical range to another.
#[allow(unused_variables)]
pub fn index_2d_to_1d<N>(x: N, y: N, width: N, height: N) -> N
where
    N: core::ops::Sub<Output = N>
        + Copy
        + core::ops::Div<Output = N>
        + core::ops::Mul<Output = N>
        + core::ops::Rem<Output = N>
        + core::ops::Add<Output = N>,
{
    y * width + x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_2d_to_1d_a() {
        let x = 9;
        let y = 9;
        let width = 10;
        let height = 10;

        assert_eq!(99, index_2d_to_1d(x, y, width, height))
    }
}
