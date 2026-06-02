pub mod atlas_image;
pub mod camera;
pub mod meshing;
pub mod resource;
pub mod resource_manager;
mod shaders;
pub mod vertex;

pub mod init_helpers;
pub mod input_manager;

// --- being rewritten below ---
pub mod render_state;
pub mod window_init;
pub mod world_renderer;

pub use window_init::run;
