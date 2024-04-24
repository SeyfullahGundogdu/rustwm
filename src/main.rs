use crate::backends::winit;
mod backends;
mod config;
mod handlers;
mod state;
mod utils;

fn main() {
    winit::init_winit();
}
