use std::io::{self, Write};

use crate::error::CahierError;
use crate::utils::delete_device_by_nickname;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 2 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help delete' for more information".into()));
    }

    let nickname = args[1].clone();
    
    print!("Are you sure you want to delete the device \x1b[35m{}\x1b[0m? (y/[n]): ", nickname);
    io::stdout().flush().unwrap();

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    confirm = confirm.chars().filter(|c| !c.is_whitespace()).collect();

    if confirm == "y" {
        delete_device_by_nickname(&nickname)?;
    }

    Ok(())
}

