use crate::io::input_helper::InputState;

use self::player_state::PlayerState;

pub mod block;
pub mod chunk;
pub mod player_state;
pub mod register;

#[derive(Default)]
pub struct Game {
    pub player_state: PlayerState,
}

impl Game {
    pub fn update(&mut self, state: &InputState) {
        self.player_state.update(state);
    }
}
