use axum::extract::{Path, Query};
use axum::{http::StatusCode, Json};
use serde::Deserialize;
use rand::distributions::{ Alphanumeric, DistString };

use crate::axum::extractors::authenticate::AuthenticateExtractor;
use crate::projects::axum::extractors::project_files_repository::ProjectFilesRepositoryExtractor;
use crate::projects::axum::extractors::project_repository::ProjectRepositoryExtractor;
use crate::util::or_status_code::{OrInternalServerError, OrNotFound, OrStatusCode};
use crate::projects::repository::projects::ProjectsRepository;
use crate::projects::repository::project_files::ProjectFilesRepository;

use super::request::CreateProjectRequest;
use super::response::{CreateProjectResponse, ListProjectFilesResponse, ListProjectsResponse, ProjectResponse};

pub async fn create_project(
    AuthenticateExtractor(user): AuthenticateExtractor,
    project_repository: ProjectRepositoryExtractor,
    Json(request): Json<CreateProjectRequest>
) -> Result<Json<CreateProjectResponse>, StatusCode> {
    let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    
    let id = project_repository
        .create(&id, &request.name, user.id)
        .await
        .or_internal_server_error()?;

    Ok(Json(
        CreateProjectResponse {
            id,
        }
    ))
}

pub async fn get_project_by_id(
    project_repository: ProjectRepositoryExtractor,
    Path(id): Path<String>
) -> Result<Json<ProjectResponse>, StatusCode> {
    let project = project_repository
        .get_by_id(&id)
        .await
        .or_internal_server_error()?
        .or_status_code(StatusCode::NOT_FOUND)?;

    Ok(Json(
        ProjectResponse {
            id: project.id,
            name: project.name,
        }
    ))
}

#[derive(Deserialize)]
pub struct ListProjectsQuery {
    user_id: i32,
}

pub async fn list_projects(
    project_repository: ProjectRepositoryExtractor,
    query: Query<ListProjectsQuery>
) -> Result<Json<ListProjectsResponse>, StatusCode> {
    let projects = project_repository
        .list(query.user_id)
        .await
        .or_internal_server_error()?;

    let mut response_items = Vec::new();

    for project in projects {
        response_items.push(ProjectResponse {
            id: project.id,
            name: project.name,
        })
    }

    Ok(Json(
        ListProjectsResponse {
            projects: response_items
        }
    ))
}

pub async fn list_project_files(
    project_repository: ProjectRepositoryExtractor,
    project_files_repository: ProjectFilesRepositoryExtractor,
    Path(project_id): Path<String>,
) -> Result<Json<ListProjectFilesResponse>, StatusCode> {
    project_repository
        .get_by_id(&project_id)
        .await
        .or_not_found()?;

    let project_files = project_files_repository
        .list(&project_id)
        .await
        .or_internal_server_error()?;

    let mut file_ids = Vec::new();

    for project_file in project_files {
        file_ids.push(project_file.file_id);
    }

    Ok(Json(
        ListProjectFilesResponse {
            file_ids
        }
    ))
}