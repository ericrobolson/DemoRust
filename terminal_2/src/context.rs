use crate::{app::App, cursor::Cursor, render_view::RenderView, style};
use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
    queue,
    style::Stylize,
    terminal, ExecutableCommand,
};
use std::{
    io::{stdout, Write},
    time::Duration,
};

/// A context that manages the lifecycle of the terminal.
pub struct Context {
    width: u16,
    height: u16,
    refresh_rate_hz: u32,
}
impl Context {
    /// Creates a new context.
    pub fn new(refresh_rate_hz: u32) -> crossterm::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;

        let (width, height) = crossterm::terminal::size()?;

        Ok(Self {
            refresh_rate_hz,
            width,
            height,
        })
    }

    /// Returns the size of the terminal
    pub fn size(&self) -> crossterm::Result<(u16, u16)> {
        Ok((self.width, self.height))
    }

    /// Resizes the context.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Runs the given context and app.
    pub fn run(mut self, mut app: App) -> crossterm::Result<()> {
        let mut should_exit = false;
        let mut stdout = stdout();
        let mut last_render: Option<RenderView> = None;

        while !should_exit {
            if crossterm::event::poll(Duration::from_secs_f32(1.0 / self.refresh_rate_hz as f32))? {
                let event = crossterm::event::read()?;

                // System handling of events outside of application space.
                match &event {
                    Event::Key(e) => {
                        // Always allow exiting. This is done at the root level to prevent anyone from overriding this.
                        if e.modifiers.contains(KeyModifiers::CONTROL)
                            && e.code == KeyCode::Char('c')
                        {
                            should_exit = true;
                        }
                    }
                    Event::Resize(width, height) => {
                        last_render = None;
                        self.resize(*width, *height)
                    }
                    _ => {}
                }
                app.handle_event(event)?;
            };

            if let Some(view) = app.update() {
                stdout.execute(crossterm::cursor::SavePosition)?;
                Cursor::set_visible(false)?;

                // Render each cell in the view
                for cell in view.iter() {
                    // Skip cell if it's the same as the last render
                    let mut skip = false;
                    if let Some(last_render) = &last_render {
                        if let Some(last_cell) = last_render.get(cell.x, cell.y) {
                            if last_cell == cell {
                                skip = true;
                            }
                        }
                    }

                    // Render cell
                    if !skip {
                        let mut content = cell.item.char.stylize();
                        if let Some(background) = cell.item.background {
                            content = content.on(map(background));
                        }
                        if let Some(color) = cell.item.color {
                            content = content.with(map(color));
                        }

                        queue!(
                            stdout,
                            crossterm::cursor::MoveTo(cell.x as u16, cell.y as u16),
                            crossterm::style::PrintStyledContent(content),
                        )?;
                    }
                }

                stdout.execute(crossterm::cursor::RestorePosition)?;
                Cursor::set_visible(true)?;

                last_render = Some(view);
            }

            stdout.flush()?;
        }

        Ok(())
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");

        queue!(stdout(), crossterm::cursor::MoveTo(self.width, self.height),)
            .expect("Could not reset cursor");

        println!("");
    }
}

fn map(color: style::Color) -> crossterm::style::Color {
    let (r, g, b) = color.into();
    crossterm::style::Color::Rgb { r, g, b }
}
