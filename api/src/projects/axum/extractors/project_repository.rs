use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};
use crate::projects::repository::projects::ProjectsRepositoryOption;

pub struct ProjectRepositoryExtractor(ProjectsRepositoryOption);

impl Deref for ProjectRepositoryExtractor {
    type Target = ProjectsRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ProjectRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for ProjectRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(ProjectRepositoryExtractor::default())
    }
}