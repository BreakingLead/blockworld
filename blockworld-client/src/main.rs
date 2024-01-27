fn main() {
    pollster::block_on(blockworld_client::run());
}
