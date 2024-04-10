use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum CreateError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for CreateError { }

impl Display for CreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateError::Sqlx(e) => write!(f, "Error running query: {}", e),
            CreateError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for CreateError {
    fn from(value: PoolError<SqlxError>) -> Self {
        CreateError::Pool(value)
    }
}

impl From<SqlxError> for CreateError {
    fn from(value: SqlxError) -> Self {
        CreateError::Sqlx(value)
    }
}