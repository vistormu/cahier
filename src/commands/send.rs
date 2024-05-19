use std::process::Command;
use crate::error::CahierError;
use crate::utils::retrieve_device_by_nickname;


pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() != 4 {
        return Err(CahierError::InvalidCommand("Incorrect number of arguments provided. Use 'cahier help send' for more information".into()));
    }

    let host = args[1].clone();
    let file = args[2].clone();
    let destination = args[3].clone();

    let device = retrieve_device_by_nickname(&host)?;

    let scp_command = format!("scp -P {} -r {} {}@{}:{}", device.port, file, device.host, device.ip, destination);

    Command::new("sh")
        .arg("-c")
        .arg(scp_command)
        .status()?;

    Ok(())
}
