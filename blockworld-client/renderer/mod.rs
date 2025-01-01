pub mod atlas_image;
pub mod bytes_provider;
pub mod camera;
mod debug_gui;
pub mod entity;
pub mod gui;
pub mod meshing;
pub mod resource_manager;
mod shaders;
pub mod vertex;
pub mod world_renderer;

pub mod pipeline;
pub mod render_state;
pub mod texture;
pub mod uniform;
pub mod vertex;
pub mod window_init;

pub use window_init::run;
