#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct ViewState {
    pub entity_positions: [Position; 500],
    pub active_entities: u32,
}
impl ViewState {
    pub fn new() -> Self {
        Self {
            active_entities: 0,
            entity_positions: [Position::default(); 500],
        }
    }
}
