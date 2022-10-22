use core::ops::*;

/// Linear interpolation.
pub fn lerp<N>(a: N, b: N, t: N) -> N
where
    N: Add<Output = N> + Sub<Output = N> + Mul<Output = N> + Copy,
{
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_0_returns_a() {
        let a = 1.0;
        let b = 3.0;
        let t = 0.0;
        let expected = a;

        assert_eq!(expected, lerp(a, b, t))
    }

    #[test]
    fn t_1_returns_a() {
        let a = 1.0;
        let b = 3.0;
        let t = 1.0;
        let expected = b;

        assert_eq!(expected, lerp(a, b, t))
    }

    #[test]
    fn t_point5_returns_mid_point() {
        let a = 0.0;
        let b = 4.0;
        let t = 0.5;
        let expected = 2.0;

        assert_eq!(expected, lerp(a, b, t))
    }
}
