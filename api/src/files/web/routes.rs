use axum::{extract::Multipart, http::StatusCode, Json};
use rand::distributions::{Alphanumeric, DistString};
use crate::{axum::extractors::authenticate::AuthenticateExtractor, files::axum::extractors::{metadata_repository::MetadataRepositoryExtractor, persistor::PersistorExtractor}, util::or_status_code::{OrBadRequest, OrInternalServerError}};
use super::response::CreateFileResponse;
use crate::files::persist::Persistor;
use crate::files::repository::metadata::MetadataRepository;

const UPLOAD_CAP: usize = 16;

pub async fn create_file<'a>(
    AuthenticateExtractor(user): AuthenticateExtractor,
    persistor: PersistorExtractor<'a>,
    metadata_repository: MetadataRepositoryExtractor,
    mut request: Multipart
) -> Result<Json<CreateFileResponse>, StatusCode> {
    let mut ids = Vec::new();

    while let Some(field) = request.next_field().await.or_internal_server_error()? {
        let name = field.name().or_bad_request()?.to_string();
        let bytes = field.bytes().await.or_bad_request()?;

        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let key = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        persistor
            .persist(&key, bytes)
            .await
            .or_internal_server_error()?;

        metadata_repository
            .create(&id, &key, user.id, &name)
            .await
            .or_internal_server_error()?;

        ids.push(id.to_string());

        if ids.len() >= UPLOAD_CAP {
            break;
        }
    }

    Ok(Json(
        CreateFileResponse {
            ids
        }
    ))
}