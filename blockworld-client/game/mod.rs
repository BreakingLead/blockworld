use std::rc::Rc;

use chunk_provider::ClientChunkProvider;
use world::ClientWorld;

use crate::io::input_helper::InputState;

use self::player_state::PlayerState;

pub mod block;
pub mod chunk;
pub mod chunk_provider;
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
impl Default for Game {
    fn default() -> Self {
        let client_world = Rc::new(ClientWorld);
        Self {
            player_state: Default::default(),
            client_world: client_world.clone(),
            // ! TEMP
            chunk_provider: ClientChunkProvider::new(client_world.clone(), 16),
        }
    }
}

impl Game {
    /// update all entity states in game (except for camera)
    pub fn update(&mut self, state: &InputState) {
        self.player_state.update(state);
    }
}
