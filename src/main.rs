use std::env;

mod commands;
mod error;
mod constants;
mod utils;
mod network_device;


fn main() {
    let args: Vec<String> = env::args().collect();

    commands::run(args);
}
