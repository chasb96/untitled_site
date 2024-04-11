use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListMetadataRequest {
    pub keys: Vec<String>,
}