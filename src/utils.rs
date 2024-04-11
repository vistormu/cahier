use std::env;
use std::io::{self, BufRead};
use std::fs;
use std::path::Path;
use std::io::Write;

use crate::error::CahierError;
use crate::constants::CAHIER_DIR;
use crate::network_device::NetworkDevice;


pub fn create_page() -> Result<(), CahierError> {
    let home_dir = env::var("HOME")?;

    let path = Path::new(home_dir.as_str()).join(CAHIER_DIR);

    if !path.exists() {
        fs::create_dir_all(path.clone())?;
    }

    let file_path = path.join("page");
    if !file_path.exists() {
        fs::File::create(file_path)?;
    }

    Ok(())
}

pub fn clear_page() -> Result<(), CahierError> {
    let home_dir = env::var("HOME")?;

    let path = Path::new(home_dir.as_str()).join(CAHIER_DIR);

    if !path.exists() {
        return Ok(());
    }

    let file_path = path.join("page");
    if file_path.exists() {
        fs::remove_file(file_path)?;
    }

    Ok(())
}

pub fn read_devices_from_page() -> Result<Vec<NetworkDevice>, CahierError> {
    let home_dir = env::var("HOME")?;
    let path = Path::new(home_dir.as_str()).join(CAHIER_DIR).join("page");

    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut devices = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(":").collect();

        let host = parts[1].split("@").collect::<Vec<&str>>()[0];
        let ip = parts[1].split("@").collect::<Vec<&str>>()[1];

        devices.push(NetworkDevice {
            host: host.to_string(),
            ip: ip.parse()?,
            nickname: parts[0].to_string(),
        });
    }

    Ok(devices)
}

pub fn add_device_to_page(device: NetworkDevice) -> Result<(), CahierError> {
    let home_dir = env::var("HOME")?;
    let path = Path::new(home_dir.as_str()).join(CAHIER_DIR).join("page");

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(path)?;

    writeln!(file, "{}: {}@{}", device.nickname, device.host, device.ip)?;

    println!("\nSuccessfully added a new entry to the cahier page:");
    println!(" -> \x1b[36m{}\x1b[0m: \x1b[33m{}\x1b[0m@\x1b[35m{}\x1b[0m", device.nickname, device.host, device.ip);


    Ok(())
}

pub fn retrieve_device_by_nickname(nickname: &str) -> Result<NetworkDevice, CahierError> {
    let home_dir = env::var("HOME")?;
    let path = Path::new(home_dir.as_str()).join(CAHIER_DIR).join("page");

    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(":").collect();

        if parts[0] == nickname {
            let host = parts[1].split("@").collect::<Vec<&str>>()[0];
            let ip = parts[1].split("@").collect::<Vec<&str>>()[1];

            return Ok(NetworkDevice {
                host: host.to_string(),
                ip: ip.parse()?,
                nickname: parts[0].to_string(),
            });
        }
    }

    Err(CahierError::DeviceNotFound(nickname.to_string()))
}

pub fn delete_device_by_nickname(nickname: &str) -> Result<(), CahierError> {
    let home_dir = env::var("HOME")?;
    let path = Path::new(home_dir.as_str()).join(CAHIER_DIR).join("page");

    let file = fs::File::open(path.clone())?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(":").collect();

        if parts[0] != nickname {
            lines.push(line);
        }
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
