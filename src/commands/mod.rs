use crate::error::CahierError;

mod add;
mod bring;
mod clear;
mod config;
mod connect;
mod delete;
mod help;
mod list;
mod ping;
mod send;
mod setup;
mod version;

fn exit_with_error(message: &str) {
    println!("\x1b[31mCahier panicked with the following message:\x1b[0m");
    println!("{}", message);
    std::process::exit(1);
}

pub fn run(args: Vec<String>) {
    if args.len() < 2 {
        exit_with_error("No command provided. Use `cahier help` for more information.");
    }

    let mut args = args;
    args.remove(0);
    let command = args[0].clone();

    let result = match command.as_str() {
        "add" => add::execute(args),
        "bring" => bring::execute(args),
        "clear" => clear::execute(args),
        "config" => config::execute(args),
        "connect" => connect::execute(args),
        "delete" => delete::execute(args),
        "help" => help::execute(args),
        "list" => list::execute(args),
        "ping" => ping::execute(args),
        "send" => send::execute(args),
        "setup" => setup::execute(args),
        "version" => version::execute(args),
        _ => Err(CahierError::InvalidCommand("Invalid command. Use 'cahier help' for more information.".into())),
    };

    match result {
        Ok(_) => (),
        Err(e) => exit_with_error(&e.to_string()),
    }
}
