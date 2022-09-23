use crate::c_api::ViewState;

pub struct GameState {
    tick: u64,
}
impl GameState {
    pub fn new() -> Self {
        Self { tick: 0 }
    }
    pub fn tick(&mut self) {
        println!("Hi: {:?}", self.tick);
        self.tick += 1;
    }

    /// Copies the game state to the view.
    pub fn copy_to_view(&self, view_state: &mut ViewState) {}
}
