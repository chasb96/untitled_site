use serde::Serialize;

#[derive(Serialize)]
pub struct CreateFileResponse {
    pub ids: Vec<String>,
}