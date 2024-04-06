use serde::Serialize;

#[derive(Serialize)]
pub struct SignUpResponse {
    pub id: i32,
}

#[derive(Serialize)]
pub struct AuthenticateResponse {
    pub token: String,
}