#![deny(unused_must_use)]

use clap::Parser;
use renderer::run;

mod game;
mod renderer;
mod resource;

fn main() {
    pollster::block_on(run());
}
