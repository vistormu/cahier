use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;

use crate::error::CahierError;
use crate::utils::create_page;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help setup' for more information".into()));
    }

    println!("Hello! I am \x1b[35mCahier\x1b[0m, your personal SSH assistant!\n");
    println!("First, let's specify the \x1b[35mcahier path\x1b[0m to access my page.");

    print!("-> \x1b[35mcahier path\x1b[0m [~/.cahier/]: ");
    io::stdout().flush().unwrap();

    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    path = path.chars().filter(|c| !c.is_whitespace()).collect();

    if path.is_empty() {
        path = "~/.cahier/".to_string();
    }

    add_cahier_path_to_shell_rc(&path)?;

    let shell = env::var("SHELL").unwrap_or_else(|_| "".to_string());
    let shell = shell.split('/').last().unwrap_or("").to_string();
    println!("-> \x1b[35mCAHIER_PATH\x1b[0m has been set to \x1b[35m{}\x1b[0m and added to your \x1b[35m{} rc\x1b[0m file.", path, shell);

    create_page(path)?;
    println!("-> The \x1b[35mcahier page\x1b[0m path has been created.\n");

    create_ssh_key()?;

    println!("\n\x1b[33mSetup complete!\x1b[0m\n");
    println!("Now you can start adding devices to your page using the \x1b[35madd\x1b[0m command. Use \x1b[35mcahier help\x1b[0m for more information.\n");
    println!("Enjoy using \x1b[35mCahier\x1b[0m! And don't forget to \x1b[33mrestart your terminal\x1b[0m to apply the changes.\n");

    Ok(())
}

fn add_cahier_path_to_shell_rc(path: &str) -> Result<(), CahierError> {
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    let shell_rc = if shell.contains("bash") {
        ".bashrc"
    } else if shell.contains("zsh") {
        ".zshrc"
    } else {
        return Err(CahierError::UnsupportedShell(shell));
    };

    let home_dir = env::var("HOME")?;
    let shell_rc_path = PathBuf::from(home_dir.clone()).join(shell_rc);
    let shell_rc_path = shell_rc_path.to_str().ok_or(CahierError::UnsupportedShell(shell.clone()))?;
    let path = path.replace("~", home_dir.as_str());

    let command = format!("echo '\n# cahier\nexport CAHIER_PATH=\"{}\"' >> {}", path, shell_rc_path);
    let status = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(CahierError::CommandFailed("echo".into()));
    }

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
