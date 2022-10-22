/// Maps a value from one numerical range to another.
#[allow(unused_variables)]
pub fn index_1d_to_2d<N>(index: N, width: N, height: N) -> (N, N)
where
    N: core::ops::Sub<Output = N>
        + Copy
        + core::ops::Div<Output = N>
        + core::ops::Mul<Output = N>
        + core::ops::Rem<Output = N>,
{
    let x = index % width;
    let y = index / width;

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_1d_to_2d_a() {
        let index = 99;
        let width = 10;
        let height = 10;

        assert_eq!((9, 9), index_1d_to_2d(index, width, height))
    }
}
