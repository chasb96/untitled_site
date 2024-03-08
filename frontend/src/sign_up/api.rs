use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use crate::api_error::ApiError;

#[derive(Serialize)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct CreateUserResponse {
    id: i32,
}

impl User {
    pub async fn create_user(username: &str, password: &str) -> Result<i32, ApiError> {
        let res = Request::post("/api/users/create")
            .json(
                &User {
                    username: username.to_string(),
                    password: password.to_string(),
                }
            )?
            .send()
            .await?;

        match res.status() {
            200 => Ok(res.json::<CreateUserResponse>().await?.id),
            _ => Err(ApiError::InternalServerError),
        }
    }
}