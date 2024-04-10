use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum PersistError {
    IO(io::Error),
}

impl Error for PersistError { }

impl Display for PersistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistError::IO(e) => write!(f, "Error writing file: {}", e),
        }
    }
}

impl From<io::Error> for PersistError {
    fn from(value: io::Error) -> Self {
        PersistError::IO(value)
    }
}