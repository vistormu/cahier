use crate::error::CahierError;

pub fn execute(args: Vec<String>) -> Result<(), CahierError> {
    for args in args {
        println!("{}", args);
    }

    Ok(())
}

