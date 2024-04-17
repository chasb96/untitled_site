use serde::Serialize;

#[derive(Serialize)]
pub struct ListProjectFilesResponse {
    pub file_ids: Vec<String>,
}