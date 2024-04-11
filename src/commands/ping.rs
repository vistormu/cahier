use std::process::Command;

use crate::error::CahierError;
use crate::utils::retrieve_device_by_nickname;


pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() != 2 {
        return Err(CahierError::InvalidCommand("Invalid number of arguments provided. Use 'cahier help ping' for more information.".into()));
    }

    let host = &args[1];
    let device = retrieve_device_by_nickname(host)?;
    
    let ping_command = format!("ping {}", device.ip);

    Command::new("sh")
        .arg("-c")
        .arg(ping_command)
        .status()?;

    Ok(())
}
