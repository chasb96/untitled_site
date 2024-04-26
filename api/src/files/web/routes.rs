use axum::{extract::{Multipart, Path, Request}, http::{header::{CONTENT_DISPOSITION, CONTENT_TYPE}, HeaderMap, HeaderValue, StatusCode}, response::IntoResponse, Json, RequestExt};
use bytes::Bytes;
use file_format::FileFormat;
use rand::distributions::{Alphanumeric, DistString};
use crate::{axum::extractors::authenticate::AuthenticateExtractor, files::{axum::extractors::{metadata_repository::MetadataRepositoryExtractor, persistor::PersistorExtractor}, web::response::MetadataResponse}, util::or_status_code::{OrBadRequest, OrInternalServerError, OrNotFound}};
use super::{request::ListMetadataRequest, response::{CreateFileResponse, ListMetadataResponse}};
use crate::files::persist::Persistor;
use crate::files::repository::metadata::MetadataRepository;

pub async fn post_files<'a>(
    authenticate_extractor: AuthenticateExtractor,
    persistor: PersistorExtractor<'a>,
    metadata_repository: MetadataRepositoryExtractor,
    request: Request
) -> impl IntoResponse {
    let content_type = request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|content_type| content_type.to_str().ok())
        .or_bad_request()?;

    if content_type == "application/json" {
        let json = request
            .extract()
            .await
            .or_internal_server_error()?;
    
        list_metadata(metadata_repository, json)
            .await
            .map(|json| json.into_response())
    } else if content_type.starts_with("multipart/form-data") {
        let multipart = request
            .extract()
            .await
            .or_internal_server_error()?;

        create_file(authenticate_extractor, persistor, metadata_repository, multipart)
            .await
            .map(|json| json.into_response())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn create_file<'a>(
    AuthenticateExtractor(user): AuthenticateExtractor,
    persistor: PersistorExtractor<'a>,
    metadata_repository: MetadataRepositoryExtractor,
    mut request: Multipart
) -> Result<Json<CreateFileResponse>, StatusCode> {
    const UPLOAD_CAP: usize = 16;
    const FILE_FORMAT_WHITELIST: &[FileFormat] = &[
        FileFormat::StereolithographyAscii,
        FileFormat::PortableNetworkGraphics,
        FileFormat::PlainText,
        FileFormat::PortableDocumentFormat,
        FileFormat::JointPhotographicExpertsGroup,
    ];

    let mut ids = Vec::new();

    while let Some(field) = request.next_field().await.or_internal_server_error()? {
        let name = field.name().or_bad_request()?.to_string();
        let bytes = field.bytes().await.or_bad_request()?;

        let file_format = FileFormat::from_bytes(&bytes);

        if !FILE_FORMAT_WHITELIST.contains(&file_format) {
            return Err(StatusCode::BAD_REQUEST);
        }

        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let key = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        persistor
            .persist(&key, bytes)
            .await
            .or_internal_server_error()?;

        metadata_repository
            .create(&id, &key, user.id, &name, file_format.media_type())
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

pub async fn list_metadata(
    metadata_repository: MetadataRepositoryExtractor,
    Json(request): Json<ListMetadataRequest>
) -> Result<Json<ListMetadataResponse>, StatusCode> {
    const LIST_CAP: usize = 256;

    if request.keys.len() > LIST_CAP {
        return Err(StatusCode::BAD_REQUEST);
    }

    let metadata = metadata_repository
        .list(request.keys)
        .await
        .or_internal_server_error()?;

    Ok(Json(
        ListMetadataResponse {
            files: metadata
                .iter()
                .map(|metadata| MetadataResponse {
                    name: metadata.name.to_owned(),
                    user_id: metadata.user_id,
                })
                .collect()
        }
    ))
}

pub async fn get_by_id<'a>(
    persistor: PersistorExtractor<'a>,
    metadata_repository: MetadataRepositoryExtractor,
    Path(id): Path<String>
) -> Result<(HeaderMap, Bytes), StatusCode> {
    let metadata = metadata_repository
        .get_by_id(&id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let content = persistor
        .read(&metadata.key)
        .await
        .or_internal_server_error()?;
    
    let content_type = HeaderValue::from_str(&metadata.mime).or_internal_server_error()?;
    let content_disposition = HeaderValue::from_str(&format!("attachment; filename=\"{}\"", metadata.name)).or_internal_server_error()?;

    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, content_type);
    headers.insert(CONTENT_DISPOSITION, content_disposition);

    Ok((headers, content))
}