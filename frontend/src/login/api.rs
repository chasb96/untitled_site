use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use crate::api_error::ApiError;

#[derive(Serialize)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthenticateResponse {
    token: String,
}

impl User {
    pub async fn authenticate(username: &str, password: &str) -> Result<String, ApiError> {
        let res = Request::post("/api/authenticate")
            .json(
                &User {
                    username: username.to_string(),
                    password: password.to_string(),
                }
            )?
            .send()
            .await?;

        match res.status() {
            200 => Ok(res.json::<AuthenticateResponse>().await?.token),
            _ => Err(ApiError::InternalServerError),
        }
    }
}