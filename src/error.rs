use std::{env::VarError, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EnvError(VarError),
    RequestError(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EnvError(message) => {
                write!(f, "message:{}", message)
            }
            Error::RequestError(message) => write!(f, "message:{}", message),
        }
    }
}

impl std::error::Error for Error {}
