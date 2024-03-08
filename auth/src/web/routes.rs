use axum::{http::StatusCode, Json};
use log::error;
use crate::{axum::extractors::user_repository::UserRepositoryExtractor, hash::scrypt::generate_password_hash, repository::users::error::CreateUserError, util::or_status_code::OrInternalServerError};
use super::{request::CreateUserRequest, response::CreateUserResponse};
use crate::repository::users::UserRepository;

pub async fn create_user(
    user_repository: UserRepositoryExtractor,
    Json(request): Json<CreateUserRequest>
) -> Result<Json<CreateUserResponse>, StatusCode> {
    let password_hash = generate_password_hash(&request.password)
        .or_internal_server_error()?;

    let id = user_repository
        .create_user(&request.username, &password_hash)
        .await
        .map_err(|err| match err {
            CreateUserError::UsernameTaken => StatusCode::BAD_REQUEST,
            e => {
                error!("{:?}", e);
                
                StatusCode::INTERNAL_SERVER_ERROR
            },
        })?;

    Ok(Json(
        CreateUserResponse {
            id,
        }
    ))
}