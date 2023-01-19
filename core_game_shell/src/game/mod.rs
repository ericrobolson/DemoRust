mod frame;
mod game_timer;
mod player_id;
mod rollback_controls;
mod state;

use crate::math::sequences::sequence_a_after_b_u16;
use game_timer::*;
use player_id::*;
use rollback_controls::*;
use state::*;

const MAX_TICKS_PER_UPDATE: usize = 10;
type Players = [Option<PlayerId>; 12];

pub struct Game {
    tick_timer: GameTimer,
    players: Players,
    controls: RollbackControls,
    confirmed_state: State,
    current_state: State,
}
impl Game {
    /// Performs an update on the game.
    pub fn update(&mut self, delta_t_seconds: f32) {
        self.controls.poll_remote();

        // Gaffer on games fix your timestep.
        // https://gafferongames.com/post/fix_your_timestep/
        self.tick_timer.update(delta_t_seconds);
        for _ in 0..MAX_TICKS_PER_UPDATE {
            match self.tick_timer.tick() {
                TimerResult::Ticked => self.execute_ticks_with_rollback(),
                TimerResult::NotTicked => break,
            }
        }
    }

    /// Performs all ticks
    fn execute_ticks_with_rollback(&mut self) {
        let current_frame = self.current_state.frame();

        // Take checkpoint and iterate on things
        let should_rollback = sequence_a_after_b_u16(
            self.controls.last_confirmed_frame().inner(),
            self.confirmed_state.frame().inner(),
        );
        if should_rollback {
            let working_state = &mut self.current_state;
            working_state.copy_from(&self.confirmed_state);

            // Iterate over all frames that need to roll back.
            // Skip current frame though as we'll handle that after.
            while working_state.frame() != current_frame {
                Self::tick(working_state, &self.players, &self.controls);

                // Checkpoint state if needed
                if self.controls.is_confirmed(working_state.frame()) {
                    self.confirmed_state.copy_from(working_state);
                }
            }
        }

        // Perform regular tick
        Self::tick(&mut self.current_state, &self.players, &self.controls);
    }

    /// Ticks the given state after sourcing all player input.
    fn tick(state: &mut State, players: &Players, controls: &RollbackControls) {
        for player in players.iter().filter_map(|p| *p) {
            let input = controls.get_player_input(player, state.frame());

            state.apply_input(player, input);
        }

        state.tick();
    }
}
