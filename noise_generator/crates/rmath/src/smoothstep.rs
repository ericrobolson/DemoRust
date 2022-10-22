/// Smoothstep function. https://en.wikipedia.org/wiki/Smoothstep
pub fn smoothstep(value: f32, edge0: f32, edge1: f32) -> f32 {
    if value < edge0 {
        0.0
    } else if value >= edge1 {
        1.0
    } else {
        value * value * (3.0 - 2.0 * value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoothstep_simple() {
        let a = 3.5;

        assert_eq!(1.0, smoothstep(a, 0.0, 1.0))
    }
}
