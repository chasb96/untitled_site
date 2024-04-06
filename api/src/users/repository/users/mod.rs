pub mod error;
mod postgres;

use sqlx::Row;
use sqlx::postgres::PgRow;
use self::error::GetError;

use super::postgres::PostgresDatabase;

pub struct User {
    pub id: i32,
    pub username: String,
}

impl From<PgRow> for User {
    fn from(row: PgRow) -> Self {
        User {
            id: row.get("id"),
            username: row.get("username"),
        }
    }
}

pub trait UserRepository {
    async fn get_by_id(&self, id: i32) -> Result<Option<User>, GetError>;

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetError>;
}

pub enum UserRepositoryOption {
    Postgres(PostgresDatabase),
}

impl UserRepository for UserRepositoryOption {
    async fn get_by_id(&self, id: i32) -> Result<Option<User>, GetError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await
        }
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetError> {
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