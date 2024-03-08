use std::{env, error::Error, fmt::Display, sync::OnceLock};
use deadpool::managed::{Pool, BuildError};
use super::deadpool::ConnectionManager;

static CONNECTION_POOL: OnceLock<Pool<ConnectionManager>> = OnceLock::new();

#[derive(Debug)]
pub enum InitializeConnectionPoolError {
    Sqlx(sqlx::Error),
    Deadpool(BuildError),
}

impl Error for InitializeConnectionPoolError { }

impl Display for InitializeConnectionPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitializeConnectionPoolError::Sqlx(e) => write!(f, "InitializeConnectionPoolError::Sqlx({})", e),
            InitializeConnectionPoolError::Deadpool(e) => write!(f, "InitializeConnectionPoolError::Deadpool({})", e),
        }
    }
}

impl From<sqlx::Error> for InitializeConnectionPoolError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<BuildError> for InitializeConnectionPoolError {
    fn from(value: BuildError) -> Self {
        Self::Deadpool(value)
    }
}

pub struct PostgresDatabase {
    pub connection_pool: &'static Pool<ConnectionManager>,
}

impl Default for PostgresDatabase {
    fn default() -> Self {
        Self { 
            connection_pool: CONNECTION_POOL
                .get_or_init(|| {
                    let connection_string = env::var("DATABASE_URL")
                        .expect("Unable to obtain env var DATABASE_URL");

                    let manager = ConnectionManager {
                        connection_string,
                    };

                    Pool::builder(manager)
                        .build()
                        .expect("Unable to create connection pool")
                })
        }
    }
}