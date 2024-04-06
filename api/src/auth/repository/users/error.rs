use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum SignUpError {
    UsernameTaken,
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for SignUpError { }

impl Display for SignUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignUpError::UsernameTaken => write!(f, "Username taken"),
            SignUpError::Sqlx(e) => write!(f, "Error running query: {}", e),
            SignUpError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for SignUpError {
    fn from(value: PoolError<SqlxError>) -> Self {
        SignUpError::Pool(value)
    }
}

impl From<SqlxError> for SignUpError {
    fn from(value: SqlxError) -> Self {
        SignUpError::Sqlx(value)
    }
}

#[derive(Debug)]
pub enum GetUserError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for GetUserError { }

impl Display for GetUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetUserError::Sqlx(e) => write!(f, "Error running query: {}", e),
            GetUserError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for GetUserError {
    fn from(value: PoolError<SqlxError>) -> Self {
        GetUserError::Pool(value)
    }
}

impl From<SqlxError> for GetUserError {
    fn from(value: SqlxError) -> Self {
        GetUserError::Sqlx(value)
    }
}