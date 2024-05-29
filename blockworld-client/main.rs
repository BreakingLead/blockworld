use clap::Parser;

mod game;
mod io;
mod render;

#[derive(Parser, Clone, Default)]
#[command(author, version, about)]
pub struct BootArgs {
    #[arg(long, default_value = "600")]
    height: u32,
    #[arg(long, default_value = "800")]
    width: u32,
    #[arg(long, default_value = "false")]
    full_screen: bool,
}

fn main() {
    pollster::block_on(render::window_init::run());
}
