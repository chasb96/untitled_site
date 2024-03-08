use gloo::net::http::Request;
use serde::Deserialize;
use crate::api_error::ApiError;

#[derive(Clone, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

impl User {
    pub async fn find_by_username(username: &str) -> Result<Option<User>, ApiError> {
        let res = Request::get(&format!("/api/users/@{}", username))
            .send()
            .await?;

        match res.status() {
            404 => Ok(None),
            200 => Ok(res.json().await?),
            _ => Err(ApiError::InternalServerError),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self { 
            id: Default::default(), 
            username: Default::default() 
        }
    }
}