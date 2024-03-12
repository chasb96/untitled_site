use axum::{http::StatusCode, Json};
use crate::jwt::{verify_jwt, ClaimsUser, ValidateJwtError};
use super::{request::AuthenticateRequest, response::AuthenticateResponse};

pub async fn authenticate(
    Json(request): Json<AuthenticateRequest>
) -> Result<Json<AuthenticateResponse>, StatusCode> {
    verify_jwt(request.token)
        .map(|user: ClaimsUser| Json(AuthenticateResponse {
            id: user.id,
        }))
        .map_err(|err| match err {
            ValidateJwtError::HmacKey(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ValidateJwtError::Verify(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ValidateJwtError::Claims(_) => StatusCode::UNAUTHORIZED,
        })
}