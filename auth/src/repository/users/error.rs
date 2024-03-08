use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum CreateUserError {
    UsernameTaken,
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for CreateUserError { }

impl Display for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateUserError::UsernameTaken => write!(f, "Username taken"),
            CreateUserError::Sqlx(e) => write!(f, "Error running query: {}", e),
            CreateUserError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for CreateUserError {
    fn from(value: PoolError<SqlxError>) -> Self {
        CreateUserError::Pool(value)
    }
}

impl From<SqlxError> for CreateUserError {
    fn from(value: SqlxError) -> Self {
        CreateUserError::Sqlx(value)
    }
}