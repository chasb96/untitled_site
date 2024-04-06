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

#[derive(Debug)]
pub enum GetByIdError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for GetByIdError { }

impl Display for GetByIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetByIdError::Sqlx(e) => write!(f, "Error running query: {}", e),
            GetByIdError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for GetByIdError {
    fn from(value: PoolError<SqlxError>) -> Self {
        GetByIdError::Pool(value)
    }
}

impl From<SqlxError> for GetByIdError {
    fn from(value: SqlxError) -> Self {
        GetByIdError::Sqlx(value)
    }
}

#[derive(Debug)]
pub enum ListError {
    Sqlx(SqlxError),
    Pool(PoolError<SqlxError>),
}

impl Error for ListError { }

impl Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListError::Sqlx(e) => write!(f, "Error running query: {}", e),
            ListError::Pool(e) => write!(f, "Error obtaining connection from pool: {}", e),
        }
    }
}

impl From<PoolError<SqlxError>> for ListError {
    fn from(value: PoolError<SqlxError>) -> Self {
        ListError::Pool(value)
    }
}

impl From<SqlxError> for ListError {
    fn from(value: SqlxError) -> Self {
        ListError::Sqlx(value)
    }
}