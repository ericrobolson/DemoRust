mod app;
mod containers;
mod context;
mod cursor;
mod math;
mod render_view;
mod style;
mod ui_layer;

fn main() -> crossterm::Result<()> {
    let tick_hz = 60;
    let context = context::Context::new(tick_hz)?;
    let (width, height) = context.size()?;

    context.run(app::App::new(width, height, tick_hz))
}
