use self::player_state::PlayerState;
use crate::io::input_helper::InputState;
use std::rc::Rc;
use world::ClientWorld;

pub mod console_instr;
pub mod player_state;
pub mod register;
pub mod settings;
pub mod world;

pub struct Game {
    pub player_state: PlayerState,
    pub client_world: Rc<ClientWorld>,
    pub chunk_provider: ClientChunkProvider,
}

impl Game {
    /// update all entity states in game (except for camera)
    pub fn update(&mut self, state: &InputState) {
        self.player_state.update(state);
    }
}
