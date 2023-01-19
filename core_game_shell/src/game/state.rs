use crate::player_input::PlayerInput;

use super::{frame::Frame, player_id::PlayerId};

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    frame: Frame,
}
impl State {
    pub fn frame(&self) -> Frame {
        self.frame
    }
    pub fn copy_from(&mut self, other: &Self) {
        *self = other.clone()
    }
    pub fn apply_input(self: &mut Self, player: PlayerId, input: PlayerInput) {
        todo!("apply input to the state")
    }
    pub fn tick(&mut self) {
        self.frame = self.frame.increment();

        todo!("run through systems")
    }
}
