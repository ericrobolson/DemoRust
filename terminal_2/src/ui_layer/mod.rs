use self::{element::*, math::Vec2d};
use std::time::Duration;

mod element;
mod math;

pub enum Event {
    Tick { delta_t: Duration },
    Cursor(CursorEvent),
}

pub enum CursorEvent {
    Move(Vec2d<u32>),
}

pub enum Renderable {}

pub struct Ui {
    elements: Vec<Element>,
    renderables: Vec<Renderable>,
}

impl Ui {
    /// Handles the given event
    pub fn handle_event(&mut self, event: Event) {
        for element in self.elements.iter_mut() {
            element.handle_event(&event);
        }
    }

    /// Presents the UI view
    pub fn render(&self) -> impl Iterator<Item = &Renderable> {
        self.renderables.iter()
    }
}
