use crate::error::CahierError;
use crate::constants::VERSION;


pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    if args.len() > 1 {
        return Err(CahierError::InvalidCommand("Too many arguments provided. Use 'cahier help version' for more information".into()));
    }

    println!("Cahier version {}", VERSION);

    Ok(())
}
