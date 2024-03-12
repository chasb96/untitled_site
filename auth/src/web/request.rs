use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthenticateRequest {
    pub token: String,
}