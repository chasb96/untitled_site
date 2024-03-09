use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String,
}