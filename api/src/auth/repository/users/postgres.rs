use crate::auth::repository::postgres::PostgresDatabase;
use super::error::GetUserError;
use super::error::SignUpError;
use super::User;
use super::UserRepository;
use sqlx::Row;
use sqlx::postgres::PgRow;

impl UserRepository for PostgresDatabase {
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<i32, SignUpError> {
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
            .map_err(SignUpError::from)?;

        if user_id.is_some() {
            return Err(SignUpError::UsernameTaken);
        }        

        sqlx::query(INSERT_QUERY)
            .bind(username)
            .bind(password_hash)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }
    
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError> {
        const GET_BY_USERNAME_QUERY: &'static str = r#"
            SELECT
                id,
                username,
                password_hash
            FROM
                users
            WHERE
                username = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(GET_BY_USERNAME_QUERY)
            .bind(username)
            .map(Into::into)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(Into::into)
    }
}