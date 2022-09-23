mod ids;
mod view_state;

use crate::{game, resource_manager::ResourceManager};
pub use ids::*;
pub use view_state::*;

/// A container for FFI state.
/// This should be obscured as the game state can change over time.
pub struct State {
    view_state: ViewState,
    game: game::GameState,
    resource_manager: ResourceManager,
    ct: usize,
}
impl State {
    fn new() -> Self {
        Self {
            resource_manager: ResourceManager::new(),
            ct: 0,
            view_state: ViewState::new(),
            game: game::GameState::new(),
        }
    }

    fn render(&mut self) {
        self.game.copy_to_view(&mut self.view_state);
    }
}

#[repr(C)]
pub struct Viewable {
    pub name: *const u8,
}

/// Attempts to fetch a string with the given handle.
/// Returns null if no string was present.
#[no_mangle]
pub extern "C" fn cg_resource_fetch_string_utf8<'a>(
    cg_game_state: &'a State,
    resource: StringId,
) -> *const u8 {
    match cg_game_state.resource_manager.get_string(resource) {
        Some(s) => {
            // Need to ensure strings are \0 terminated when returning them.
            let mut s = s.clone();
            s.push('\0');
            s.as_ptr()
        }
        None => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn cb_get_string_id() -> StringId {
    StringId { id: 0 }
}

#[no_mangle]
pub extern "C" fn cb_render_view_state(cg_game_state: Box<State>) -> Box<ViewState> {
    Box::new(cg_game_state.view_state)
}

/// Creates a new game state.
#[no_mangle]
pub extern "C" fn cg_new() -> Box<State> {
    println!("New state!");
    Box::new(State::new())
}

/// Ticks the game state.
#[no_mangle]
pub extern "C" fn cg_tick<'a>(cg_game_state: &'a mut State) {
    let mut state = cg_game_state;

    state.ct += 1;
    println!("{:?}", state.ct);
    // state
}

/// Registers the given texture with the engine.
/// Provides a texture id for future usage.
#[no_mangle]
pub extern "C" fn cg_resource_register_texture<'a>(
    cg_game_state: &'a mut State,
    img_width: u32,
    img_height: u32,
) -> TextureId {
    TextureId { id: 0 }
}

/// Drops the given texture from the engine.
#[no_mangle]
pub extern "C" fn cg_resource_drop_texture<'a>(cg_game_state: &'a mut State, texture: TextureId) {}

/*
FFI examples


// Callback
#[no_mangle]
pub extern "C" fn cg_test_add_cb(cb: extern "C" fn(i32, i32)) {
    cb(1, 2);
}


*/
