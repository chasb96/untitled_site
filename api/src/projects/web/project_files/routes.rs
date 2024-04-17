use axum::extract::Path;
use axum::{http::StatusCode, Json};

use crate::axum::extractors::authenticate::AuthenticateExtractor;
use crate::projects::axum::extractors::project_files_repository::ProjectFilesRepositoryExtractor;
use crate::projects::axum::extractors::project_repository::ProjectRepositoryExtractor;
use crate::util::or_status_code::{OrInternalServerError, OrNotFound};
use crate::projects::repository::projects::ProjectsRepository;
use crate::projects::repository::project_files::ProjectFilesRepository;

use super::request::FileIdSetRequest;
use super::response::ListProjectFilesResponse;

pub async fn list_project_files(
    project_repository: ProjectRepositoryExtractor,
    project_files_repository: ProjectFilesRepositoryExtractor,
    Path(project_id): Path<String>,
) -> Result<Json<ListProjectFilesResponse>, StatusCode> {
    project_repository
        .get_by_id(&project_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let project_files = project_files_repository
        .list(&project_id)
        .await
        .or_internal_server_error()?;

    let response = ListProjectFilesResponse {
        file_ids: project_files
            .into_iter()
            .map(|project_file| project_file.file_id)
            .collect()
    };

    Ok(Json(response))
}

pub async fn associate_files(
    AuthenticateExtractor(user): AuthenticateExtractor,
    project_repository: ProjectRepositoryExtractor,
    project_files_repository: ProjectFilesRepositoryExtractor,
    Path(project_id): Path<String>,
    Json(request): Json<FileIdSetRequest>
) -> Result<StatusCode, StatusCode> {
    let project = project_repository
        .get_by_id(&project_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN)
    }

    project_files_repository
        .add_range(&project_id, request.file_ids)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::OK)
}

pub async fn disassociate_files(
    AuthenticateExtractor(user): AuthenticateExtractor,
    project_repository: ProjectRepositoryExtractor,
    project_files_repository: ProjectFilesRepositoryExtractor,
    Path(project_id): Path<String>,
    Json(request): Json<FileIdSetRequest>
) -> Result<StatusCode, StatusCode> {
    let project = project_repository
        .get_by_id(&project_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN)
    }

    project_files_repository
        .delete_range(&project_id, request.file_ids)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::OK)
}

pub async fn sync_files(
    AuthenticateExtractor(user): AuthenticateExtractor,
    project_repository: ProjectRepositoryExtractor,
    project_files_repository: ProjectFilesRepositoryExtractor,
    Path(project_id): Path<String>,
    Json(request): Json<FileIdSetRequest>
) -> Result<StatusCode, StatusCode> {
    let project = project_repository
        .get_by_id(&project_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN)
    }

    project_files_repository
        .sync(&project_id, request.file_ids)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::OK)
}