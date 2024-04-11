use std::{fmt, io, env};


#[derive(Debug)]
pub enum CahierError {
    EnvError(env::VarError),
    IOError(io::Error),
    InvalidCommand(String),
}

impl fmt::Display for CahierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CahierError::EnvError(ref err) => write!(f, "Environment error:\n{}", err),
            CahierError::IOError(ref err) => write!(f, "IO error:\n{}", err),
            CahierError::InvalidCommand(ref err) => write!(f, "Invalid command:\n{}", err),
        }
    }
}

impl From<env::VarError> for CahierError {
    fn from(err: env::VarError) -> CahierError {
        CahierError::EnvError(err)
    }
}

impl From<io::Error> for CahierError {
    fn from(err: io::Error) -> CahierError {
        CahierError::IOError(err)
    }
}
