use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::{jwt::{verify_jwt, ClaimsUser}, util::or_status_code::OrStatusCode};

pub struct AuthenticateExtractor(pub ClaimsUser);

impl Deref for AuthenticateExtractor {
    type Target = ClaimsUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for AuthenticateExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let (scheme, authentication) = parts.headers
            .get("Authorization")
            .or_status_code(StatusCode::UNAUTHORIZED)?
            .to_str()
            .or_status_code(StatusCode::UNAUTHORIZED)?
            .split_once(' ')
            .or_status_code(StatusCode::BAD_REQUEST)?;

        match (scheme.to_ascii_uppercase().as_ref(), authentication) {
            ("BEARER", token) => 
                verify_jwt(token.to_string())
                    .map(|user| AuthenticateExtractor(user))
                    .or_status_code(StatusCode::UNAUTHORIZED),
            _ => Err(StatusCode::BAD_REQUEST)
        }
    }
}