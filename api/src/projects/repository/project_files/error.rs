use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum QueryError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for QueryError { }

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::Sqlx(e) => write!(f, "Error running query: {}", e),
            QueryError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for QueryError {
    fn from(value: PoolError<SqlxError>) -> Self {
        QueryError::Pool(value)
    }
}

impl From<SqlxError> for QueryError {
    fn from(value: SqlxError) -> Self {
        QueryError::Sqlx(value)
    }
}