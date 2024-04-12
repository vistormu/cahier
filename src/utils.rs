use std::env;
use std::io::{self, BufRead};
use std::fs;
use std::path::Path;
use std::io::Write;

use crate::error::CahierError;
use crate::network_device::NetworkDevice;


pub fn create_page(path: String) -> Result<(), CahierError> {
    let mut path = path;
    if path.starts_with("~") {
        let home_dir = env::var("HOME")?;
        path = path.replace("~", home_dir.as_str());
    }

    if path.contains("~") {
        return Err(CahierError::CommandFailed(("path contains invalid character '~'").to_string()));
    }

    let path = Path::new(path.as_str());

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let file_path = path.join("page");
    if !file_path.exists() {
        fs::File::create(file_path)?;
    }

    Ok(())
}

pub fn clear_page() -> Result<(), CahierError> {
    let cahier_dir = match env::var("CAHIER_PATH") { 
        Ok(dir) => dir,
        Err(_) => return Err(CahierError::SetupNotComplete),
    };

    let path = Path::new(cahier_dir.as_str()).join("page");

    if path.exists() {
        fs::remove_file(path)?;
    }

    Ok(())
}

pub fn read_devices_from_page() -> Result<Vec<NetworkDevice>, CahierError> {
    let cahier_dir = match env::var("CAHIER_PATH") { 
        Ok(dir) => dir,
        Err(_) => return Err(CahierError::SetupNotComplete),
    };
    let path = Path::new(cahier_dir.as_str()).join("page");

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
    let cahier_dir = match env::var("CAHIER_PATH") { 
        Ok(dir) => dir,
        Err(_) => return Err(CahierError::SetupNotComplete),
    };
    let path = Path::new(cahier_dir.as_str()).join("page");

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(path)?;

    writeln!(file, "{}: {}@{}", device.nickname, device.host, device.ip)?;

    println!("\nSuccessfully added a new entry to the cahier page:");
    println!(" -> \x1b[36m{}\x1b[0m: \x1b[33m{}\x1b[0m@\x1b[35m{}\x1b[0m", device.nickname, device.host, device.ip);


    Ok(())
}

pub fn retrieve_device_by_nickname(nickname: &str) -> Result<NetworkDevice, CahierError> {
    let cahier_dir = match env::var("CAHIER_PATH") { 
        Ok(dir) => dir,
        Err(_) => return Err(CahierError::SetupNotComplete),
    };
    let path = Path::new(cahier_dir.as_str()).join("page");

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
    let cahier_dir = match env::var("CAHIER_PATH") { 
        Ok(dir) => dir,
        Err(_) => return Err(CahierError::SetupNotComplete),
    };
    let path = Path::new(cahier_dir.as_str()).join("page");

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

pub fn open_page() -> Result<(), CahierError> {
    let cahier_dir = match env::var("CAHIER_PATH") { 
        Ok(dir) => dir,
        Err(_) => return Err(CahierError::SetupNotComplete),
    };
    let path = Path::new(cahier_dir.as_str()).join("page");

    let default_editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

    let status = std::process::Command::new(default_editor.clone())
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(CahierError::CommandFailed((default_editor).to_string()));
    }

    Ok(())
}
