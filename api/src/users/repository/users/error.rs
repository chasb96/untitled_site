use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum GetError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for GetError { }

impl Display for GetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetError::Sqlx(e) => write!(f, "Error running query: {}", e),
            GetError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for GetError {
    fn from(value: PoolError<SqlxError>) -> Self {
        GetError::Pool(value)
    }
}

impl From<SqlxError> for GetError {
    fn from(value: SqlxError) -> Self {
        GetError::Sqlx(value)
    }
}