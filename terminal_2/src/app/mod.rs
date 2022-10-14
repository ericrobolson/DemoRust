use crate::{cursor::Cursor, render_view};
use crossterm::event::{Event, KeyCode};
use std::time::{Duration, Instant};

/// The application that is ran.
pub struct App {
    width: u16,
    height: u16,
    dirty: bool,
    last_tick: Instant,
    tick_rate: Duration,
    accumulated: Duration,
    frame: usize,
}
impl App {
    /// Creates a new instance of the app.
    pub fn new(width: u16, height: u16, tick_hz: u32) -> Self {
        Self {
            frame: 0,
            last_tick: Instant::now(),
            accumulated: Duration::default(),
            tick_rate: Duration::from_secs_f32(1.0 / tick_hz as f32),
            width,
            height,
            dirty: true,
        }
    }

    /// Ticks the app.
    pub fn update(&mut self) -> Option<render_view::RenderView> {
        // Gaffer on games type timestep
        {
            let now = Instant::now();
            self.accumulated += now - self.last_tick;
            self.last_tick = now;
            let mut remaining_ticks = 10;
            while self.accumulated >= self.tick_rate && remaining_ticks > 0 {
                self.accumulated -= self.tick_rate;
                self.tick();
                remaining_ticks -= 1;
            }
        }

        // Present render if there's a dirty state.
        if self.dirty {
            self.dirty = false;

            Some(self.render())
        } else {
            None
        }
    }

    /// Performs a single tick of the app.
    fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        // TODO: if state changes need to set dirty to true.
    }

    /// Returns a render view.
    pub fn render(&self) -> render_view::RenderView {
        let w = self.width as usize;
        let h = self.height as usize;

        let mut view = render_view::RenderView::new(w, h);

        for cell in view.iter_mut() {
            cell.item.char = ' ';
            if cell.x == 0 {
                cell.item.char = (cell.y as u8 + ('0' as u8)) as char;
            }
        }

        view
    }

    /// Handles the given event
    pub fn handle_event(&mut self, event: Event) -> crossterm::Result<()> {
        match event {
            Event::Resize(w, h) => {
                self.dirty = true;

                self.width = w;
                self.height = h;
            }
            Event::Key(e) => {
                let dirty = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right];
                if dirty.contains(&e.code) {
                    if e.code == KeyCode::Up {
                        Cursor::move_up(1)?;
                    } else if e.code == KeyCode::Down {
                        Cursor::move_down(1)?;
                    }

                    if e.code == KeyCode::Left {
                        Cursor::move_left(1)?;
                    } else if e.code == KeyCode::Right {
                        Cursor::move_right(1)?;
                    }
                }
            }
            Event::Mouse(_) | Event::Paste(_) | Event::FocusLost | Event::FocusGained => {
                println!("UNHANDLED: {:?}", event);
            }
        }

        Ok(())
    }
}
