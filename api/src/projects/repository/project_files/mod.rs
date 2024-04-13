mod error;
mod postgres;

use sqlx::postgres::PgRow;
use sqlx::Row;

use super::project_files::error::{CreateError, ListError};
use super::postgres::PostgresDatabase;

pub struct ProjectFile {
    pub id: i32,
    pub project_id: String,
    pub file_id: String,
}

impl From<PgRow> for ProjectFile {
    fn from(row: PgRow) -> Self {
        ProjectFile {
            id: row.get("id"),
            project_id: row.get("project_id"),
            file_id: row.get("file_id"),
        }
    }
}

pub trait ProjectFilesRepository {
    async fn create(&self, project_id: &str, file_id: &str) -> Result<i32, CreateError>;

    async fn list(&self, project_id: &str) -> Result<Vec<ProjectFile>, ListError>;
}

pub enum ProjectFilesRepositoryOption {
    Postgres(PostgresDatabase),
}

impl ProjectFilesRepository for ProjectFilesRepositoryOption {
    async fn create(&self, project_id: &str, file_id: &str) -> Result<i32, CreateError> {
        match self {
            Self::Postgres(pg) => pg.create(project_id, file_id).await
        }
    }

    async fn list(&self, project_id: &str) -> Result<Vec<ProjectFile>, ListError> {
        match self {
            Self::Postgres(pg) => pg.list(project_id).await
        }
    }
}

impl Default for ProjectFilesRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}