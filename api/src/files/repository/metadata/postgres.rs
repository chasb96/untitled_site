use sqlx::Row;
use sqlx::postgres::PgRow;
use crate::files::repository::postgres::PostgresDatabase;
use super::{error::{GetByIdError, ListError}, Metadata, MetadataRepository};

impl MetadataRepository for PostgresDatabase {
    async fn create(&self, id: &str, key: &str, user_id: i32, name: &str, mime: &str) -> Result<String, super::error::CreateError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO metadata (id, key, user_id, name, mime)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(id)
            .bind(key)
            .bind(user_id)
            .bind(name)
            .bind(mime)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }
    
    async fn list(&self, keys: Vec<String>) -> Result<Vec<Metadata>, ListError> {
        const LIST_QUERY: &'static str = r#"
            SELECT
                id,
                key,
                user_id,
                name,
                mime
            FROM
                metadata
            WHERE
                key IN ($1)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(LIST_QUERY)
            .bind(keys)
            .map(Into::into)
            .fetch_all(conn.as_mut())
            .await
            .map_err(Into::into)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Metadata>, GetByIdError> {
        const GET_BY_ID_QUERY: &'static str = r#"
            SELECT
                id,
                key,
                user_id,
                name,
                mime
            FROM
                metadata
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
}