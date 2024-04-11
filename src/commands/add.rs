use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::net::IpAddr;

use crate::error::CahierError;
use crate::utils::{create_page, add_device_to_page};
use crate::network_device:: NetworkDevice;


pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help add' for more information".into()));
    }

    create_page()?;

    let device = get_network_device()?;
    create_ssh_key()?;
    send_ssh_key(&device)?;
    add_device_to_page(device)?;
    
    Ok(())
}

fn get_network_device() -> Result<NetworkDevice, CahierError> {
    // nickname
    print!("Enter a nickname for the host: ");
    io::stdout().flush().unwrap();

    let mut nickname = String::new();
    io::stdin().read_line(&mut nickname)?;
    nickname = nickname.chars().filter(|c| !c.is_whitespace()).collect();

    // host name
    print!("Enter the host name: ");
    io::stdout().flush().unwrap();

    let mut host = String::new();
    io::stdin().read_line(&mut host)?;
    host = host.chars().filter(|c| !c.is_whitespace()).collect();

    // ip address
    print!("Enter the ip address: ");
    io::stdout().flush().unwrap();

    let mut ip = String::new();
    io::stdin().read_line(&mut ip)?;
    ip = ip.chars()
        .filter(|c| !c.is_whitespace())
        .collect();


    let ip: IpAddr = ip.parse()?;

    Ok(NetworkDevice {
        host,
        ip,
        nickname,
    })
}

fn create_ssh_key() -> Result<(), CahierError> {
    print!("Do you want to generate a new ssh key? (y/[n]): ");
    io::stdout().flush().unwrap();

    let mut generate_key = String::new();
    io::stdin().read_line(&mut generate_key)?;
    generate_key = generate_key.chars().filter(|c| !c.is_whitespace()).collect();

    if generate_key == "y" {
        println!("\n\x1b[33mGenerating a new ssh key\x1b[0m\n");
        let keygen_command = "ssh-keygen -t rsa -b 4096";
        let status = Command::new("sh")
            .arg("-c")
            .arg(keygen_command)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Err(CahierError::CommandFailed("ssh-keygen".into()));
        }
    }

    Ok(())
}

fn send_ssh_key(device: &NetworkDevice) -> Result<(), CahierError> {
    print!("Do you want to copy the ssh key to the host? (y/[n]): ");
    io::stdout().flush().unwrap();

    let mut copy_key = String::new();
    io::stdin().read_line(&mut copy_key)?;
    copy_key = copy_key.chars().filter(|c| !c.is_whitespace()).collect();

    if copy_key == "y" {
        println!("\n\x1b[33mCopying the ssh key to the host\x1b[0m\n");
        let ssh_key_command = format!("ssh-copy-id {}@{}", device.host, device.ip);
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
