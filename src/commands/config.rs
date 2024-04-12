use crate::error::CahierError;
use crate::utils::open_page;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help config' for more information".into()));
    }

    open_page()?;

    Ok(())
}

