mod camera;
mod draw;
mod texture;
mod vertex;
mod window_init;
fn main() {
    pollster::block_on(window_init::run());
}
