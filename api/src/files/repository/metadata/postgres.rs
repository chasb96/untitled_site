use sqlx::Row;
use sqlx::postgres::PgRow;
use crate::files::repository::postgres::PostgresDatabase;
use super::MetadataRepository;

impl MetadataRepository for PostgresDatabase {
    async fn create(&self, id: &str, key: &str, user_id: i32, name: &str) -> Result<String, super::error::CreateError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO metadata (id, key, user_id, name)
            VALUES ($1, $2, $3, $4)
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
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }
}