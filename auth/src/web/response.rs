use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
}