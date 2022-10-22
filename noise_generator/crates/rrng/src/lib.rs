//#![cfg_attr(not(test), no_std)]
mod noise;
mod perlin;
mod rng;

pub use noise::*;
pub use perlin::*;
pub use rng::*;

/// Creates a random unit vector.
pub(crate) fn unit_vector(rng: &mut Rng) -> rmath::Vec2d<f32> {
    let phi = 2.0 * core::f32::consts::PI * rng.next_f32();

    let x = phi.cos();
    let y = phi.sin();

    (x, y).into()
}
