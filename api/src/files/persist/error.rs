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

#[derive(Debug)]
pub enum ReadError {
    IO(io::Error),
}

impl Error for ReadError { }

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::IO(e) => write!(f, "Error reading file: {}", e),
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(value: io::Error) -> Self {
        ReadError::IO(value)
    }
}