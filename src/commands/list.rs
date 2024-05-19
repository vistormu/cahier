use crate::error::CahierError;
use crate::utils::read_devices_from_page;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help list' for more information".into()));
    }

    let devices = read_devices_from_page()?;

    for device in devices {
        println!(" -> \x1b[36m{}\x1b[0m: \x1b[33m{}\x1b[0m@\x1b[35m{}\x1b[0m:{}", device.nickname, device.host, device.ip, device.port);
    }

    Ok(())
}

