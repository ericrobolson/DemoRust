use rcontainers::Grid;

use crate::{
    perlin::{self, Perlin},
    Rng,
};

#[derive(Clone)]
pub struct Noise(Grid<f32>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    ReplaceIfLessThan { n: f32, value: f32 },
}

pub trait Generator {
    fn get(&self, x: usize, y: usize) -> f32;
}

impl Noise {
    pub fn erode(&mut self, erosion_distance: usize) {
        let new_w = self.0.width() + erosion_distance * 2;
        let new_h = self.0.height() + erosion_distance * 2;

        // create a new grid, copy to cells
        let mut n: Grid<f32> = Grid::new(new_w, new_h, -1.0);

        // Copy to inner grid
        for x in 0..self.0.width() {
            for y in 0..self.0.height() {
                *n.item_mut(x + erosion_distance, y + erosion_distance) = self.value(x, y);
            }
        }

        self.0 = n;
    }

    /// Initializes the noise from Perlin.
    pub fn perlin(&mut self, width: usize, height: usize, scale: f32, rng: &mut Rng) {
        let perlin = perlin::Perlin::new(width, height, scale, rng);
        for ((x, y), item) in self.0.iter_mut() {
            *item = perlin.get(x, y);
        }
    }

    /// Applies the given function to the noise.
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = ((usize, usize), &mut f32)> + 'a {
        self.0.iter_mut()
    }

    /// Applies fractal noise using the provided generator.
    pub fn fractal_noise(
        &mut self,
        depth: usize,
        generator: Option<fn(f32, &mut Rng) -> Box<dyn Generator>>,
        factor: Option<fn(usize) -> f32>,
        rng: &mut Rng,
    ) {
        let generator = match generator {
            Some(g) => g,
            None => Self::default_generator,
        };

        let factor = match factor {
            Some(factor) => factor,
            None => Self::default_factor,
        };

        for n in 0..depth {
            let f = factor(n);
            self.accumulate(generator(f, rng), 1.0 / f);
        }
    }

    /// Accumulates the given values.
    pub(crate) fn accumulate(&mut self, generator: Box<dyn Generator>, factor: f32) {
        for ((x, y), item) in self.0.iter_mut() {
            *item += generator.get(x, y) * factor;
        }
    }

    /// Sets the grid to one value.
    pub fn fill(&mut self, value: f32) {
        let value = value.clamp(-1.0, 1.0);
        for (_, item) in self.0.iter_mut() {
            *item = value;
        }
    }

    /// Creates a new noise grid.
    pub fn new(width: usize, height: usize) -> Self {
        Self(Grid::new(width, height, 0.0))
    }

    /// Resizes the noise grid.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.0 = Grid::new(width, height, 0.0);
    }

    /// Renormalizes the noise so that everything is distributed evenly.
    pub fn normalize(&mut self) {
        let mut max = 0.0;
        for (_, item) in self.0.iter() {
            if *item > max {
                max = *item;
            }
        }

        for (_, item) in self.0.iter_mut() {
            *item = rmath::map_to_range(*item, 0.0, max, 0.0, 1.0);
        }
    }

    /// Performs a gaussian blur on the noise.
    pub fn blur(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut buffer = Grid::new(self.0.width(), self.0.height(), 0.);

            for x in 0..self.0.width() {
                for y in 0..self.0.height() {
                    let value = self.get_mean_of_pixels(x, y);
                    *buffer.item_mut(x, y) = value;
                }
            }

            self.0 = buffer;
        }
    }

    // Returns the arithmatic mean of the 3 pixels for the grid.
    fn get_mean_of_pixels(&self, x: usize, y: usize) -> f32 {
        let x_min = x.wrapping_sub(1);
        let x_max = x.wrapping_add(1);
        let y_min = y.wrapping_sub(1);
        let y_max = y.wrapping_add(1);

        let g = &self.0;

        let mut val = 0.0;

        for x in [x_min, x, x_max] {
            for y in [y_min, y, y_max] {
                val += *g.item(x, y);
            }
        }

        let val = val / 9.0;

        val
    }

    /// Reapplies noise.
    pub fn noise(&mut self, rng: &mut Rng) {
        for (_, item) in self.0.iter_mut() {
            *item = rng.next_f32();
        }
    }

    /// Returns a value on the grid.
    pub fn value(&self, x: usize, y: usize) -> f32 {
        *self.0.item(x, y)
    }

    fn default_factor(n: usize) -> f32 {
        2.0_f32.powf((n + 1) as f32)
    }

    fn default_generator(factor: f32, rng: &mut Rng) -> Box<dyn Generator> {
        let width = 12.0 * factor;
        let height = 8.0 * factor;
        let scale = 100.0 / factor;
        Box::new(Perlin::new(width as usize, height as usize, scale, rng))
    }
}

impl std::ops::MulAssign for Noise {
    fn mul_assign(&mut self, rhs: Self) {
        for ((x, y), item) in self.iter_mut() {
            *item *= rhs.value(x, y);
        }

        self.normalize();
    }
}

impl std::ops::Mul for Noise {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut noise = self.clone();

        for ((x, y), item) in noise.iter_mut() {
            *item = self.value(x, y) * rhs.value(x, y);
        }

        noise.normalize();

        noise
    }
}
