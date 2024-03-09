use axum::{http::StatusCode, Json};
use log::error;
use crate::hash::scrypt::verify_password;
use crate::jwt::{generate_jwt, ClaimsUser};
use crate::repository::users::error::SignUpError;
use crate::util::or_status_code::OrStatusCode;
use crate::{axum::extractors::user_repository::UserRepositoryExtractor, hash::scrypt::generate_password_hash, util::or_status_code::OrInternalServerError};
use crate::repository::users::UserRepository;
use super::request::{AuthenticateRequest, SignUpRequest};
use super::response::{AuthenticateResponse, SignUpResponse};

pub async fn sign_up(
    user_repository: UserRepositoryExtractor,
    Json(request): Json<SignUpRequest>
) -> Result<Json<SignUpResponse>, StatusCode> {
    let password_hash = generate_password_hash(&request.password)
        .or_internal_server_error()?;

    let id = user_repository
        .create_user(&request.username, &password_hash)
        .await
        .map_err(|err| match err {
            SignUpError::UsernameTaken => StatusCode::BAD_REQUEST,
            e => {
                error!("{:?}", e);
                
                StatusCode::INTERNAL_SERVER_ERROR
            },
        })?;

    Ok(Json(
        SignUpResponse {
            id,
        }
    ))
}

pub async fn authenticate(
    user_repository: UserRepositoryExtractor,
    Json(request): Json<AuthenticateRequest>
) -> Result<Json<AuthenticateResponse>, StatusCode> {
    let user = user_repository
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?
        .or_status_code(StatusCode::UNAUTHORIZED)?;

    if !verify_password(&request.password, &user.password_hash).or_internal_server_error()? {
        return Err(StatusCode::UNAUTHORIZED)
    }

    Ok(Json(
        AuthenticateResponse {
            token: generate_jwt(ClaimsUser::from(user))
                .or_internal_server_error()?,
        }
    ))
}