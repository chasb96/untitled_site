pub mod error;
mod postgres;

use sqlx::Row;
use sqlx::postgres::PgRow;
use self::error::{GetUserError, SignUpError};
use super::postgres::PostgresDatabase;

pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl From<PgRow> for User {
    fn from(row: PgRow) -> Self {
        User {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
        }
    }
}

pub trait UserRepository {
    async fn create_user(&self, username: &str, password: &str) -> Result<i32, SignUpError>;

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError>;
}

pub enum UserRepositoryOption {
    Postgres(PostgresDatabase),
}

impl UserRepository for UserRepositoryOption {
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<i32, SignUpError> {
        match self {
            Self::Postgres(pg) => pg.create_user(username, password_hash).await
        }
    }
    
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError> {
        match self {
            Self::Postgres(pg) => pg.get_by_username(username).await
        }
    }
}

impl Default for UserRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}