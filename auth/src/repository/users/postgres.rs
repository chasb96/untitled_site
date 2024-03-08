use crate::repository::postgres::PostgresDatabase;
use super::{error::CreateUserError, UserRepository};
use sqlx::Row;
use sqlx::postgres::PgRow;

impl UserRepository for PostgresDatabase {
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<i32, CreateUserError> {
        const GET_QUERY: &'static str = r#"
            SELECT id
            FROM users
            WHERE username = $1
        "#;

        const INSERT_QUERY: &'static str = r#"
            INSERT INTO users (username, password_hash)
            VALUES ($1, $2)
            RETURNING id
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let user_id: Option<i32> = sqlx::query(GET_QUERY)
            .bind(username)
            .map(|row: PgRow| row.get("id"))
            .fetch_optional(conn.as_mut())
            .await
            .map_err(CreateUserError::from)?;

        if user_id.is_some() {
            return Err(CreateUserError::UsernameTaken);
        }        

        sqlx::query(INSERT_QUERY)
            .bind(username)
            .bind(password_hash)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }
}