mod renderer;

fn main() {
    pollster::block_on(renderer::run());
}
