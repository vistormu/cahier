use std::io::{self, Write};

use crate::error::CahierError;
use crate::utils::clear_page;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help clear' for more information".into()));
    }

    print!("Are you sure you want to clear the page? (y/[n]): ");
    io::stdout().flush().unwrap();

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    confirm = confirm.chars().filter(|c| !c.is_whitespace()).collect();

    if confirm == "y" {
        clear_page()?;
    }

    Ok(())
}
