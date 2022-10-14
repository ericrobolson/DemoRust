use super::*;
use math::*;
use std::{fmt::Debug, time::Duration};

#[derive(Clone, Debug)]
pub struct OnSelect;

#[derive(Clone)]
pub struct OnTick(fn(&mut Element, delta_t: Duration));
impl Debug for OnTick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OnTick")
    }
}

#[derive(Clone)]
pub struct OnHover(fn(&mut Element));
impl Debug for OnHover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OnHover")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventResult {
    Ok,
    Suppress,
}

#[derive(Clone, Debug)]
pub struct Element {
    pub on_hover: Option<OnHover>,
    pub on_select: Option<OnSelect>,
    pub on_tick: Option<OnTick>,
    pub children: Vec<Element>,
    pub position: Option<Vec2d<u32>>,
    pub size: Option<Vec2d<u32>>,
    pub aabb: Option<Aabb2d<u32>>,
    pub view_layer: Option<u32>,
}
impl Element {
    /// Handles the given event.
    pub fn handle_event(&mut self, event: &Event) -> EventResult {
        let mut result = EventResult::Ok;

        for child in self.children.iter_mut() {
            match child.handle_event(event) {
                EventResult::Ok => {}
                EventResult::Suppress => {
                    result = EventResult::Suppress;
                }
            }
        }

        if result != EventResult::Suppress {
            let mut should_hover = false;

            match event {
                Event::Tick { delta_t } => {
                    if let Some(on_tick) = self.on_tick.take() {
                        on_tick.0(self, *delta_t);
                        self.on_tick = Some(on_tick);
                    }
                }
                Event::Cursor(event) => {
                    //
                    match event {
                        super::CursorEvent::Move(position) => {
                            if let Some(aabb) = self.aabb {
                                if let Some(position) = self.position {
                                    let aabb = aabb + position;
                                }
                            }
                            //
                        }
                    }
                }
            }

            if should_hover {}
        }

        result
    }
}

#[cfg(test)]
mod tests {}
