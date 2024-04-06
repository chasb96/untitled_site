use axum::extract::Path;
use axum::{http::StatusCode, Json};
use crate::users::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::util::or_status_code::{OrInternalServerError, OrNotFound};
use crate::users::repository::users::UserRepository;

use super::response::GetUserResponse;

pub async fn get_by_id(
    user_repository: UserRepositoryExtractor,
    Path(id): Path<i32>
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = user_repository
        .get_by_id(id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    Ok(Json(
        GetUserResponse {
            id: user.id,
            username: user.username,
        }
    ))
}

pub async fn get_by_username(
    user_repository: UserRepositoryExtractor,
    Path(username): Path<String>
) -> Result<Json<GetUserResponse>, StatusCode> {
    let user = user_repository
        .get_by_username(&username)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    Ok(Json(
        GetUserResponse {
            id: user.id,
            username: user.username,
        }
    ))
}