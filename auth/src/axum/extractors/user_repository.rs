use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};
use crate::repository::users::UserRepositoryOption;

pub struct UserRepositoryExtractor(UserRepositoryOption);

impl Deref for UserRepositoryExtractor {
    type Target = UserRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UserRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for UserRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(UserRepositoryExtractor::default())
    }
}