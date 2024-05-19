use std::io::{self, Write};
use std::process::{Command, Stdio};

use crate::error::CahierError;
use crate::utils::create_page;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help setup' for more information".into()));
    }

    println!("Hello! I am \x1b[35mCahier\x1b[0m, your personal SSH assistant!");

    create_page()?;
    println!("I have added the \x1b[35mcahier page\x1b[0m to ~/.cahier.\n");

    create_ssh_key()?;

    println!("\n\x1b[33mSetup complete!\x1b[0m");
    println!("Now you can start adding devices to your page using the \x1b[35madd\x1b[0m command. Use \x1b[35mcahier help\x1b[0m for more information.");
    println!("Enjoy using \x1b[35mCahier\x1b[0m!");

    Ok(())
}

fn create_ssh_key() -> Result<(), CahierError> {
    print!("Do you want to generate a new \x1b[35mssh key\x1b[0m in your local machine? (y/[n]): ");
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
