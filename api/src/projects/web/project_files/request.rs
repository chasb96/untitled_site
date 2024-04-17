use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileIdSetRequest {
    pub file_ids: Vec<String>
}