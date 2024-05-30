use crate::io::input_helper::InputState;

use self::player_state::PlayerState;

pub mod block;
pub mod chunk;
pub mod console_instr;
pub mod player_state;
pub mod register;
pub mod save;
pub mod settings;

#[derive(Default,Debug)]
pub struct Game {
    pub player_state: PlayerState,
}

impl Game {
    /// update all entity states in game (except for camera)
    pub fn update(&mut self, state: &InputState) {
        self.player_state.update(state);
    }
}
