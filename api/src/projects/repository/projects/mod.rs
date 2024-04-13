pub mod error;
mod postgres;

use sqlx::Row;
use sqlx::postgres::PgRow;
use self::error::{CreateError, GetByIdError, ListError};
use super::postgres::PostgresDatabase;

pub struct Project {
    pub id: String,
    pub name: String,
    pub user_id: i32,
}

impl From<PgRow> for Project {
    fn from(row: PgRow) -> Self {
        Project {
            id: row.get("id"),
            name: row.get("name"),
            user_id: row.get("user_id"),
        }
    }
}

pub trait ProjectsRepository {
    async fn create(&self, id: &str, name: &str, user_id: i32) -> Result<String, CreateError>;

    async fn get_by_id(&self, id: &str) -> Result<Option<Project>, GetByIdError>;

    async fn list(&self, user_id: i32) -> Result<Vec<Project>, ListError>;
}

pub enum ProjectsRepositoryOption {
    Postgres(PostgresDatabase),
}

impl ProjectsRepository for ProjectsRepositoryOption {
    async fn create(&self, id: &str, name: &str, user_id: i32) -> Result<String, CreateError> {
        match self {
            Self::Postgres(pg) => pg.create(id, name, user_id).await
        }
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Project>, GetByIdError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(id).await
        }
    }
    
    async fn list(&self, user_id: i32) -> Result<Vec<Project>, ListError> {
        match self {
            Self::Postgres(pg) => pg.list(user_id).await
        }
    }
}

impl Default for ProjectsRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}