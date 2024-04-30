use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListMetadataRequest {
    pub ids: Vec<String>,
}