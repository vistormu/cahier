use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::net::IpAddr;

use crate::error::CahierError;
use crate::utils::add_device_to_page;
use crate::network_device:: NetworkDevice;


pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help add' for more information".into()));
    }

    let device = get_network_device()?;
    send_ssh_key(&device)?;
    add_device_to_page(device)?;
    
    Ok(())
}

fn get_network_device() -> Result<NetworkDevice, CahierError> {
    // nickname
    print!("Enter a \x1b[35mnickname\x1b[0m for the host: ");
    io::stdout().flush().unwrap();

    let mut nickname = String::new();
    io::stdin().read_line(&mut nickname)?;
    nickname = nickname.chars().filter(|c| !c.is_whitespace()).collect();

    // host name
    print!("Enter the \x1b[35mhost name\x1b[0m: ");
    io::stdout().flush().unwrap();

    let mut host = String::new();
    io::stdin().read_line(&mut host)?;
    host = host.chars().filter(|c| !c.is_whitespace()).collect();

    // ip address
    print!("Enter the \x1b[35mip address\x1b[0m: ");
    io::stdout().flush().unwrap();

    let mut ip = String::new();
    io::stdin().read_line(&mut ip)?;
    ip = ip.chars()
        .filter(|c| !c.is_whitespace())
        .collect();


    let ip: IpAddr = ip.parse()?;

    // port
    print!("Enter the \x1b[35mport\x1b[0m (optional): ");
    io::stdout().flush().unwrap();

    let mut port = String::new();
    io::stdin().read_line(&mut port)?;
    port = port.chars().filter(|c| !c.is_whitespace()).collect();

    let port = if port.is_empty() {
        "22".to_string()
    } else {
        port
    };

    let port: u16 = port.parse()?;



    Ok(NetworkDevice {
        host,
        ip,
        nickname,
        port,
    })
}

fn send_ssh_key(device: &NetworkDevice) -> Result<(), CahierError> {
    print!("Do you want to \x1b[35msend the ssh key\x1b[0m to the host? (y/[n]): ");
    io::stdout().flush().unwrap();

    let mut copy_key = String::new();
    io::stdin().read_line(&mut copy_key)?;
    copy_key = copy_key.chars().filter(|c| !c.is_whitespace()).collect();

    if copy_key == "y" {
        println!("\n\x1b[33mCopying the ssh key to the host\x1b[0m\n");
        let ssh_key_command = format!("ssh-copy-id -p {} {}@{}", device.port, device.host, device.ip);
        let status = Command::new("sh")
            .arg("-c")
            .arg(ssh_key_command)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Err(CahierError::CommandFailed("ssh-copy-id".into()));
        }
    }

    Ok(())
}
