mod action;
mod commands;
mod group;
mod util;

use std::env;

fn main() {
    commands::parse(env::args().collect());
}