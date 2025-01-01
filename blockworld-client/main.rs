#![deny(unused_must_use)]
#![feature(int_roundings, lazy_cell)]
use std::sync::OnceLock;

use clap::Parser;
use renderer::run;

mod game;
mod renderer;

fn main() {
    pollster::block_on(run());
}
