use std::fs::File;

use rmath::map_to_range;
use rrng::{Noise, Perlin, Rng};

fn main() {
    let seed = std::env::args().nth(1).expect("no seed given");
    let seed = rmath::hash::fnv1a_32(seed.as_bytes());

    let w = 1024;
    let h = 1024;

    let mut img = rimg::Png::new(w, h);
    let mut rng = rrng::Rng::new(seed);

    let mut grid = rcontainers::Grid::new(w as usize, h as usize, [255, 255, 255, 255]);

    /*
       let mut elevation = rrng::Noise::new(w, h);
       elevation.fill(0.0);

       elevation.fractal_noise(
           8,
           Some(|factor, rng| {
               let width = 12.0 * factor;
               let height = 8.0 * factor;
               let scale = 500.0 / factor;
               Box::new(Perlin::new(width as usize, height as usize, scale, rng))
           }),
           None,
           &mut rng,
       );
       elevation.normalize();
    */
    // Generate rough mountains
    let mut elevation = rrng::Noise::new(w, h);

    elevation.fractal_noise(
        8,
        Some(|factor, rng| {
            let width = 12.0 * factor;
            let height = 4.0 * factor;
            let scale = 1200.0 / factor;
            Box::new(Perlin::new(width as usize, height as usize, scale, rng))
        }),
        None,
        &mut rng,
    );

    elevation.normalize();

    for ((x, y), item) in grid.iter_mut() {
        let value = map_to_range(elevation.value(x, y), -1.0, 1.0, 0.0, 255.0);

        let r = value as u8;
        let g = value as u8;
        let b = value as u8;

        *item = [r, g, b, 255];
    }

    for ((x, y), item) in grid.iter() {
        img.set_pixel(x, y, *item);
    }

    let mut file = File::create("test_render.png").unwrap();
    img.write_to(&mut file).unwrap();
}

pub struct WorldSettings {
    pub width: usize,
    pub height: usize,
}
pub fn world_generation(settings: WorldSettings, rng: &mut Rng) -> rcontainers::Grid<f32> {
    let width = settings.width;
    let height = settings.height;

    let mut grid = rcontainers::Grid::new(width, height, 0.0);

    let mut continent = Noise::new(width, height);

    continent.fill(0.0);

    continent.fractal_noise(
        1,
        Some(|factor, rng| {
            let width = 12.0 * factor;
            let height = 8.0 * factor;
            let scale = 500.0 / factor;
            Box::new(Perlin::new(width as usize, height as usize, scale, rng))
        }),
        None,
        rng,
    );
    continent.normalize();

    grid
}
