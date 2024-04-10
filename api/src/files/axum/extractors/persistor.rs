use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::files::persist::PersistorOption;

pub struct PersistorExtractor<'a>(PersistorOption<'a>);

impl<'a> Deref for PersistorExtractor<'a> {
    type Target = PersistorOption<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Default for PersistorExtractor<'a> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<'d, T> FromRequestParts<T> for PersistorExtractor<'d> {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(PersistorExtractor::default())
    }
}