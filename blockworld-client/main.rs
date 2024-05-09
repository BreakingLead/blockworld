mod game;
mod io;
mod render;
fn main() {
    pollster::block_on(render::window_init::run());
}
