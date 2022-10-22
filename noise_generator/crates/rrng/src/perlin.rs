use rcontainers::Grid;
use rmath::{lerp, quintic, Vec2d};

use crate::{unit_vector, Generator, Rng};

/// Representation for Perlin noise.
pub struct Perlin {
    scale: f32,
    items: rcontainers::Grid<Vec2d<f32>>,
}

impl Perlin {
    /// Creates a new Perlin noise.
    pub fn new(width: usize, height: usize, scale: f32, rng: &mut Rng) -> Self {
        let mut items = Grid::new(width, height, Vec2d::default());

        for (_, item) in items.iter_mut() {
            *item = unit_vector(rng)
        }

        Self { scale, items }
    }
}

impl Generator for Perlin {
    /// Gets the given value at the given coordinates.
    fn get(&self, x: usize, y: usize) -> f32 {
        let x = x as f32 / self.scale;
        let y = y as f32 / self.scale;

        let x_floor = x.floor();
        let x_floor_usize = x_floor as usize;
        let y_floor = y.floor();
        let y_floor_usize = y_floor as usize;

        let v1 = *self.items.item(x_floor_usize, y_floor_usize);
        let v2 = *self
            .items
            .item(x_floor_usize.wrapping_add(1), y_floor_usize);
        let v3 = *self
            .items
            .item(x_floor_usize, y_floor_usize.wrapping_add(1));
        let v4 = *self
            .items
            .item(x_floor_usize.wrapping_add(1), y_floor_usize.wrapping_add(1));

        let local_x = x - x_floor;
        let local_y = y - y_floor;
        let p1 = Vec2d {
            x: local_x,
            y: local_y,
        };
        let p2 = Vec2d {
            x: local_x - 1.0,
            y: local_y,
        };
        let p3 = Vec2d {
            x: local_x,
            y: local_y - 1.0,
        };
        let p4 = Vec2d {
            x: local_x - 1.0,
            y: local_y - 1.0,
        };

        let d1 = v1.dot(p1);
        let d2 = v2.dot(p2);
        let d3 = v3.dot(p3);
        let d4 = v4.dot(p4);

        let ix1 = lerp(d1, d2, quintic(local_x));
        let ix2 = lerp(d3, d4, quintic(local_x));

        lerp(ix1, ix2, quintic(local_y))
    }
}
