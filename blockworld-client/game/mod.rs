use self::player_state::PlayerState;
use crate::{io::input_helper::InputState, world::chunk_provider::ClientChunkProvider};
use blockworld_utils::AM;
use std::rc::Rc;
use world::ClientWorld;

pub mod console_instr;
pub mod player_state;
pub mod settings;
pub mod world;

pub struct Blockworld {
    pub player_state: PlayerState,
    pub client_world: AM<ClientWorld>,
    pub chunk_provider: ClientChunkProvider,
}

impl Blockworld {
    /// update all entity states in game (except for camera)
    pub fn update(&mut self, state: &InputState) {
        self.player_state.update(state);
    }
}

impl Default for Blockworld {
    fn default() -> Self {
        Self {
            player_state: todo!(),
            client_world: todo!(),
            chunk_provider: todo!(),
        }
    }
}
