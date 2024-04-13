use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::projects::repository::project_files::ProjectFilesRepositoryOption;

pub struct ProjectFilesRepositoryExtractor(ProjectFilesRepositoryOption);

impl Deref for ProjectFilesRepositoryExtractor {
    type Target = ProjectFilesRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ProjectFilesRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for ProjectFilesRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(ProjectFilesRepositoryExtractor::default())
    }
}