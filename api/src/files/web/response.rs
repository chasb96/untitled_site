use serde::Serialize;

#[derive(Serialize)]
pub struct CreateFileResponse {
    pub ids: Vec<String>,
}

#[derive(Serialize)]
pub struct MetadataResponse {
    pub id: String,
    pub name: String,
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct ListMetadataResponse {
    pub files: Vec<MetadataResponse>,
}