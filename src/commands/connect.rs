use crate::error::CahierError;
use crate::utils::retrieve_device_by_nickname;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 2 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help connect' for more information".into()));
    }

    let host = args[1].clone();
    let device = retrieve_device_by_nickname(&host)?;

    println!("Connecting to \x1b[35m{}\x1b[0m.", device.nickname);

    let ssh_command = format!("ssh -X {}@{}", device.host, device.ip);

    std::process::Command::new("sh")
        .arg("-c")
        .arg(ssh_command)
        .status()?;

    Ok(())
}

