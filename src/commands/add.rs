use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process::Command;
use crate::error::CahierError;

const CAHIER_DIR: &str = ".config/cahier";


fn create_cahier_page() -> Result<(), CahierError> {
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

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help add' for more information".into()));
    }

    create_cahier_page()?;

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
    ip = ip.chars().filter(|c| !c.is_whitespace()).collect();

    // nickname
    print!("Enter a nickname for the host: ");
    io::stdout().flush().unwrap();

    let mut nickname = String::new();
    io::stdin().read_line(&mut nickname)?;
    nickname = nickname.chars().filter(|c| !c.is_whitespace()).collect();

    // ssh-keygen
    print!("Do you want to generate a new ssh key? (y/[n]): ");
    io::stdout().flush().unwrap();

    let mut generate_key = String::new();
    io::stdin().read_line(&mut generate_key)?;
    generate_key = generate_key.chars().filter(|c| !c.is_whitespace()).collect();

    if generate_key == "y" {
        Command::new("ssh-keygen")
            .arg("-t")
            .arg("rsa")
            .arg("-b")
            .arg("4096")
            .arg("-C")
            .arg(format!("cahier@{}", host))
            .output()?;
    }

    // ssh-copy-id
    print!("Do you want to copy the ssh key to the host? (y/[n]): ");
    io::stdout().flush().unwrap();

    let mut copy_key = String::new();
    io::stdin().read_line(&mut copy_key)?;
    copy_key = copy_key.chars().filter(|c| !c.is_whitespace()).collect();

    if copy_key == "y" {
        Command::new("ssh-copy-id")
            .arg(format!("{}@{}", host, ip))
            .output()?;
    }

    // write to file
    let home_dir = env::var("HOME")?;
    let path = Path::new(home_dir.as_str()).join(CAHIER_DIR).join("page");

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(path)?;

    writeln!(file, "{}: {}@{}", nickname, host, ip)?;
    
    println!("\nSuccessfully added a new entry to the cahier page:");
    println!(" -> \x1b[36m{}\x1b[0m: \x1b[33m{}\x1b[0m@\x1b[35m{}\x1b[0m", nickname, host, ip);

    Ok(())
}
