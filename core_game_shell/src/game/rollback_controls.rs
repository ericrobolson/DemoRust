use super::{frame::Frame, player_id::PlayerId};
use crate::player_input::PlayerInput;

pub struct RollbackControls;
impl RollbackControls {
    pub fn poll_remote(&mut self) {
        todo!("poll for remote input")
    }
    pub fn is_confirmed(&self, frame: Frame) -> bool {
        todo!("return whether input is confirmed")
    }
    pub fn last_confirmed_frame(&self) -> Frame {
        todo!()
    }
    pub fn get_player_input(&self, id: PlayerId, frame: Frame) -> PlayerInput {
        // get last input if no new input exists.
        // If new inputs,
        todo!()
    }
}
