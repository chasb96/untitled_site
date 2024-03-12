use serde::Serialize;

#[derive(Serialize)]
pub struct AuthenticateResponse {
    pub id: i32,
}