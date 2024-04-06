use crate::users::repository::postgres::PostgresDatabase;
use super::error::GetError;
use super::{User, UserRepository};

impl UserRepository for PostgresDatabase {
    async fn get_by_id(&self, id: i32) -> Result<Option<User>, GetError> {
        const GET_BY_ID_QUERY: &'static str = r#"
            SELECT
                id,
                username
            FROM
                users
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

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetError> {
        const GET_BY_USERNAME_QUERY: &'static str = r#"
            SELECT
                id,
                username
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