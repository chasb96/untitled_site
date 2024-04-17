mod error;
mod postgres;

use sqlx::postgres::PgRow;
use sqlx::Row;

use self::error::QueryError;

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
    async fn create(&self, project_id: &str, file_id: &str) -> Result<i32, QueryError>;

    async fn list(&self, project_id: &str) -> Result<Vec<ProjectFile>, QueryError>;

    async fn add_range(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError>;

    async fn delete_range(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError>;

    async fn sync(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError>;
}

pub enum ProjectFilesRepositoryOption {
    Postgres(PostgresDatabase),
}

impl ProjectFilesRepository for ProjectFilesRepositoryOption {
    async fn create(&self, project_id: &str, file_id: &str) -> Result<i32, QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(project_id, file_id).await
        }
    }

    async fn list(&self, project_id: &str) -> Result<Vec<ProjectFile>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.list(project_id).await
        }
    }
    
    async fn add_range(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.add_range(project_id, file_ids).await
        }
    }
    
    async fn delete_range(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.delete_range(project_id, file_ids).await
        }
    }
    
    async fn sync(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.sync(project_id, file_ids).await
        }
    }
}

impl Default for ProjectFilesRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}