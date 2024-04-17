use futures::TryStreamExt;
use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::projects::repository::postgres::PostgresDatabase;

use super::{error::QueryError, ProjectFile, ProjectFilesRepository};

impl ProjectFilesRepository for PostgresDatabase {
    async fn create(&self, project_id: &str, file_id: &str) -> Result<i32, QueryError> {
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

    async fn list(&self, project_id: &str) -> Result<Vec<ProjectFile>, QueryError> {
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
    
    async fn add_range(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError> {
        const ADD_RANGE_QUERY: &'static str = r#"
            INSERT INTO project_files
            SELECT $1, file_id
            FROM UNNEST($2) AS file_ids(file_id)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(ADD_RANGE_QUERY)
            .bind(project_id)
            .bind(file_ids)
            .execute(conn.as_mut())
            .await?;

        Ok(())
    }
    
    async fn delete_range(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError> {
        const DELETE_RANGE_QUERY: &'static str = r#"
            DELETE 
            FROM project_files
            WHERE
                project_id = $1
                AND file_id IN ($2)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(DELETE_RANGE_QUERY)
            .bind(project_id)
            .bind(file_ids)
            .execute(conn.as_mut())
            .await?;

        Ok(())
    }
    
    async fn sync(&self, project_id: &str, file_ids: Vec<String>) -> Result<(), QueryError> {
        let deleted_file_ids = self.list(project_id)
            .await?
            .into_iter()
            .map(|project_file| project_file.file_id)
            .collect();

        self.delete_range(project_id, deleted_file_ids).await?;

        self.add_range(project_id, file_ids).await
    }
}