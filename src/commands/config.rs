use std::env;
use std::process::Command;

use crate::error::CahierError;
use crate::constants::CAHIER_DIR;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help config' for more information".into()));
    }

    let default_editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let home = env::var("HOME")?;
    let path = format!("{}/{}/page", home, CAHIER_DIR);

    Command::new(default_editor)
        .arg(path)
        .status()?;

    Ok(())
}

