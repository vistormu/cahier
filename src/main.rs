use std::env;

mod commands;
mod error;
mod constants;


fn main() {
    let args: Vec<String> = env::args().collect();

    commands::run(args);
}
