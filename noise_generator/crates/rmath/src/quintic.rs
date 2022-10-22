/// Quintic smoothing function. https://en.wikipedia.org/wiki/Quintic_function
pub fn quintic(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
