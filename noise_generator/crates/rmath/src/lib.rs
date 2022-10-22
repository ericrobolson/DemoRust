#![cfg_attr(not(test), no_std)]
//
pub mod hash;
mod index_1d_to_2d;
mod index_2d_to_1d;
mod lerp;
mod map_to_range;
mod quintic;
mod smoothstep;
mod vec2d;

pub use index_1d_to_2d::*;
pub use index_2d_to_1d::*;
pub use lerp::*;
pub use map_to_range::*;
pub use quintic::*;
pub use smoothstep::*;
pub use vec2d::*;
