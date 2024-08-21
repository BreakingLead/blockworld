#![deny(unused_must_use)]
use std::sync::OnceLock;

use clap::Parser;
use renderer::window_init::run;

mod block;
mod debug;
mod game;
mod io;
mod network;
mod renderer;
mod resources;
mod tileentity;
mod world;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1280)]
    pub width: u32,

    #[arg(short, long, default_value_t = 720)]
    pub height: u32,

    #[arg(short, long)]
    pub full_screen: bool,
}

static cli_args: OnceLock<Args> = OnceLock::new();

pub fn get_cli_args() -> &'static Args {
    cli_args.get_or_init(|| {
        let args = Args::parse();
        args
    })
}

fn main() {
    pollster::block_on(run());
}
