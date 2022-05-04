mod color;
mod image;
mod sand;
mod timer;
mod types;
mod util;
mod world;

use ext_ggez::filesystem;
use ext_ggez::graphics::{self, Color, FilterMode};
use ext_ggez::{event, graphics::DrawParam};
use ext_ggez::{Context, GameResult};
use std::env;
use std::path;
use util::array_1d_to_2d;

struct MainState {
    image: graphics::Image,
    text: graphics::Text,
    sand_img: image::Rgba8Image,
    world: world::World,
    sand_timer: timer::Timer,
    sand_spawn_timer: timer::Timer,
    sand_flipper: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        filesystem::print_all(ctx);

        let mut sand_img = image::Rgba8Image::new(32, 32);
        let mut world = world::World::new();

        let mut image = graphics::Image::from_rgba8(
            ctx,
            sand_img.width() as u16,
            sand_img.height() as u16,
            sand_img.rgba_bytes(),
        )
        .unwrap();

        let text = graphics::Text::new("Hello world!");

        let s = MainState {
            sand_timer: timer::Timer::new(30),
            image,
            text,
            sand_img,
            world,
            sand_spawn_timer: timer::Timer::new(4),
            sand_flipper: false,
        };

        Ok(s)
    }
}

impl event::EventHandler<ext_ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.sand_spawn_timer.ticked() {
            let sand = if self.sand_flipper {
                sand::Sand::Dirt
            } else {
                sand::Sand::Water
            };

            self.sand_flipper = !self.sand_flipper;

            self.world.put_sand(self.world.width() / 2, 0, sand);
        }
        if self.sand_timer.ticked() {
            self.world.tick();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Check if images should be updated

        if self.world.is_dirty() {
            for cell in self.world.render() {
                self.sand_img.put_pixel(cell.x, cell.y, cell.sand.color());
            }
        }

        if self.sand_img.is_dirty() {
            self.image = graphics::Image::from_rgba8(
                ctx,
                self.sand_img.width() as u16,
                self.sand_img.height() as u16,
                self.sand_img.rgba_bytes(),
            )
            .unwrap();
        }

        self.image.set_filter(FilterMode::Nearest);

        graphics::clear(ctx, [0.4, 0.4, 0.4, 1.0].into());

        let dest_point = ext_glam::Vec2::new(32.0, 32.0);
        let scale = ext_glam::Vec2::new(10.0, 10.0);

        let params = DrawParam::default()
            .dest(dest_point)
            .scale(scale)
            .color(Color::WHITE);
        graphics::draw(ctx, &self.image, params)?;
        graphics::draw(ctx, &self.text, (dest_point, 0.0, Color::BLACK))?;

        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("../resources")
    };

    let cb = ext_ggez::ContextBuilder::new("imageview", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
