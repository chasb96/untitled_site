use futures::TryStreamExt;
use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::projects::repository::postgres::PostgresDatabase;

use super::error::{CreateError, ListError};
use super::{ProjectFile, ProjectFilesRepository};

impl ProjectFilesRepository for PostgresDatabase {
    async fn create(&self, project_id: &str, file_id: &str) -> Result<i32, CreateError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO project_files (project_id, file_id)
            VALUES ($1, $2)
            RETURNING id
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(project_id)
            .bind(file_id)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }

    async fn list(&self, project_id: &str) -> Result<Vec<ProjectFile>, ListError> {
        const LIST_QUERY: &'static str = r#"
            SELECT
                id,
                project_id,
                file_id
            FROM
                project_files
            WHERE
                project_id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut records = sqlx::query(LIST_QUERY)
            .bind(project_id)
            .map(Into::into)
            .fetch(conn.as_mut());

        let mut project_files = Vec::new();

        while let Some(item) = records.try_next().await? {
            project_files.push(item)
        }

        Ok(project_files)
    }
}