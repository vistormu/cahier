use std::{fmt, io, env};


#[derive(Debug)]
pub enum CahierError {
    EnvError(env::VarError),
    IOError(io::Error),
    InvalidCommand(String),
    InvalidIp(String),
    DeviceNotFound(String),
    CommandFailed(String),
}

impl fmt::Display for CahierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CahierError::EnvError(ref err) => write!(f, "Environment error:\n{}", err),
            CahierError::IOError(ref err) => write!(f, "IO error:\n{}", err),
            CahierError::InvalidCommand(ref err) => write!(f, "Invalid command:\n{}", err),
            CahierError::InvalidIp(ref err) => write!(f, "Invalid IP address:\n{}", err),
            CahierError::DeviceNotFound(ref err) => write!(f, "Device not found:\n{}", err),
            CahierError::CommandFailed(ref err) => write!(f, "Command failed:\n{}", err),
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

impl From<std::net::AddrParseError> for CahierError {
    fn from(err: std::net::AddrParseError) -> CahierError {
        CahierError::InvalidIp(err.to_string())
    }
}

impl From<std::num::ParseIntError> for CahierError {
    fn from(err: std::num::ParseIntError) -> CahierError {
        CahierError::InvalidIp(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for CahierError {
    fn from(err: std::string::FromUtf8Error) -> CahierError {
        CahierError::InvalidIp(err.to_string())
    }
}

impl From<std::string::FromUtf16Error> for CahierError {
    fn from(err: std::string::FromUtf16Error) -> CahierError {
        CahierError::InvalidIp(err.to_string())
    }
}

