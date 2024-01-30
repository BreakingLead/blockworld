//! Render stuff.

pub mod camera;
pub mod instance;
pub mod state;
pub mod texture;
pub mod vertex;
pub mod resource;

trait Renderable{
    fn get_vertex_data ();
}