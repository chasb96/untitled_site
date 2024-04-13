mod error;
mod postgres;

use sqlx::Row;
use sqlx::postgres::PgRow;
use self::error::{CreateError, GetByIdError, ListError};
use super::postgres::PostgresDatabase;

pub struct Metadata {
    pub id: String,
    pub key: String,
    pub user_id: i32,
    pub name: String,
    pub mime: String,
}

impl From<PgRow> for Metadata {
    fn from(row: PgRow) -> Self {
        Metadata {
            id: row.get("id"),
            key: row.get("key"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            mime: row.get("mime")
        }
    }
}

pub trait MetadataRepository {
    async fn create(&self, id: &str, key: &str, user_id: i32, name: &str, mime: &str) -> Result<String, CreateError>;

    async fn list(&self, keys: Vec<String>) -> Result<Vec<Metadata>, ListError>;

    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, GetByIdError>;
}

pub enum MetadataRepositoryOption {
    Postgres(PostgresDatabase)
}

impl MetadataRepository for MetadataRepositoryOption {
    async fn create(&self, id: &str, key: &str, user_id: i32, name: &str, mime: &str) -> Result<String, CreateError> {
        match self {
            Self::Postgres(pg) => pg.create(id, key, user_id, name, mime).await
        }
    }
    
    async fn list(&self, keys: Vec<String>) -> Result<Vec<Metadata>, ListError> {
        match self {
            Self::Postgres(pg) => pg.list(keys).await
        }
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, GetByIdError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await
        }
    }
}

impl Default for MetadataRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}