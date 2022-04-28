mod csg;
mod image;
mod renderer;
mod world;

use ext_ggez::event;
use ext_ggez::filesystem;
use ext_ggez::graphics::{self, Color};
use ext_ggez::timer;
use ext_ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
    image: graphics::Image,
    text: graphics::Text,
    csg_img: image::Rgba8Image,
    x: u32,
    y: u32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        filesystem::print_all(ctx);

        let mut csg_img = image::Rgba8Image::new(320, 320);
        let world = world::World::new();
        let renderer = renderer::Renderer::new();

        for x in 0..csg_img.width() {
            for y in 0..csg_img.height() {
                let w = csg_img.width() as f32;
                let h = csg_img.height() as f32;
                let c = renderer.color(
                    &world,
                    (w, h).into(),
                    ((x + 1) as f32, (y + 1) as f32).into(),
                );

                csg_img.put_pixel(x, y, c);
            }
        }

        let image = graphics::Image::from_rgba8(
            ctx,
            csg_img.width() as u16,
            csg_img.height() as u16,
            csg_img.rgba_bytes(),
        )
        .unwrap();

        let text = graphics::Text::new("Hello world!");

        let s = MainState {
            x: 0,
            y: 0,
            image,
            text,
            csg_img,
        };

        Ok(s)
    }
}

impl event::EventHandler<ext_ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.csg_img.put_pixel(self.x, self.y, (255, 0, 0).into());

        self.x += 1;
        if self.x >= self.csg_img.width() {
            self.x = 0;
            self.y += 1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Check if images should be updated

        if self.csg_img.is_dirty() {
            self.image = graphics::Image::from_rgba8(
                ctx,
                self.csg_img.width() as u16,
                self.csg_img.height() as u16,
                self.csg_img.rgba_bytes(),
            )
            .unwrap();
        }

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let dest_point = ext_glam::Vec2::new(0.0, 0.0);
        graphics::draw(ctx, &self.image, (dest_point, 0.0, Color::WHITE))?;
        graphics::draw(ctx, &self.text, (dest_point, 0.0, Color::BLACK))?;

        graphics::present(ctx)?;

        timer::yield_now();
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
