use futures::TryStreamExt;
use sqlx::Row;
use sqlx::postgres::PgRow;

use crate::projects::repository::postgres::PostgresDatabase;

use super::error::{CreateError, GetByIdError, ListError};
use super::{Project, ProjectsRepository};

impl ProjectsRepository for PostgresDatabase {
    async fn create(&self, id: &str, name: &str, user_id: i32) -> Result<String, CreateError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO projects (id, user_id, name)
            VALUES ($1, $2, $3)
            RETURNING id
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(id)
            .bind(user_id)
            .bind(name)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }
    
    async fn get_by_id(&self, id: &str) -> Result<Option<Project>, GetByIdError> {
        const GET_BY_ID_QUERY: &'static str = r#"
            SELECT
                id,
                name,
                user_id
            FROM
                projects
            WHERE
                id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(GET_BY_ID_QUERY)
            .bind(id)
            .map(Into::into)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(Into::into)
    }
    
    async fn list(&self, user_id: i32) -> Result<Vec<Project>, ListError> {
        const LIST_QUERY: &'static str = r#"
            SELECT
                id,
                name,
                user_id
            FROM
                projects
            WHERE
                user_id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut records = sqlx::query(LIST_QUERY)
            .bind(user_id)
            .map(Into::into)
            .fetch(conn.as_mut());

        let mut projects = Vec::new();

        while let Some(item) = records.try_next().await? {
            projects.push(item);
        }

        Ok(projects)
    }
}