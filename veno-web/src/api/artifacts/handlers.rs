use std::sync::Arc;

use axum::{
    extract::{OriginalUri, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::trace;
use utoipa::OpenApi;
use veno_core::app::AppState;

use crate::api::{errors::ApiError, version::ApiVersion};

use super::{model::ArtifactResponse, service::check_all_artifacts};

#[derive(OpenApi)]
#[openapi(paths(check_versions, all_artifacts, artifact_for_id))]
pub struct V1ArtifactsApi;

#[utoipa::path(
    get,
    path="/check",
    responses(
        (status= OK, description = "Returns a set of checked artifacts with its new versions if there are any.", body = ArtifactResponse),
        (status= OK, description = "Returns a message if there are no new versions", body = serde_json::Value),
        (status= INTERNAL_SERVER_ERROR, description = "If during the check a server error occurs", body = ApiError)
    )
)]
pub async fn check_versions(
    version: ApiVersion,
    original_uri: OriginalUri,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Using API version: {}", version);
    let response = check_all_artifacts(&app).await;
    match response {
        Ok(Some(new_versions)) => return Ok(Json(new_versions).into_response()),
        Ok(None) => {
            return Ok(Json(
                json!({"message": "There are currently no new versions of your artifacts"}),
            )
            .into_response())
        }
        Err(_err) => Err(ArtifactError::InternalServerError(original_uri.path()).into()),
    }
}

#[utoipa::path(
    get,
    path="/",
    responses(
        (status= OK, description = "Get all artifact configurations", body = Vec<ArtifactResponse>)
    )
)]
pub async fn all_artifacts(
    version: ApiVersion,
    State(app): State<Arc<AppState>>,
) -> impl IntoResponse {
    trace!("Using API version: {}", version);
    let artifacts: Vec<ArtifactResponse> = app
        .artifacts
        .iter()
        .map(|artifact| ArtifactResponse::from(artifact.clone()))
        .collect();
    Json(artifacts)
}

#[utoipa::path(
    get,
    path="/{artifact_id}",
    responses(
        (status= OK, description = "Get a specific artifact with id = artifact_id", body = ArtifactResponse),
        (status= NOT_FOUND, description = "Returns not_found if the artifact_id had no match", body = ApiError),
    )
)]
pub async fn artifact_for_id(
    version: ApiVersion,
    original_uri: OriginalUri,
    Path((_version, artifact_id)): Path<(String, String)>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Using API version: {}", version);
    let artifact = app
        .artifacts
        .iter()
        .find(|artifact| artifact.id == artifact_id);

    match artifact {
        Some(artifact) => {
            let response_body = ArtifactResponse::from(artifact.clone());
            Ok((StatusCode::OK, Json(response_body)).into_response())
        }
        None => Err(ArtifactError::NotFoundParam {
            param: artifact_id.clone(),
            path: original_uri.path(),
        }
        .into()),
    }
}

// FIX this is ugly! i think we need to put actions like these under ../actions/
// this also makes the intention clearer and the endpoint more understandable for the caller
pub async fn artifact_for_id_check_wrapper(
    version: ApiVersion,
    original_uri: OriginalUri,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let path_params = Path((version.to_string(), "check".to_string()));
    artifact_for_id(version, original_uri, path_params, State(app)).await
}

#[derive(Debug, Error)]
pub enum ArtifactError<'a> {
    #[error("The artifact with the id={param} was not found.")]
    NotFoundParam { param: String, path: &'a str },
    #[error("There was an internal server error. Please try again later.")]
    InternalServerError(&'a str),
}

impl<'a> From<ArtifactError<'a>> for ApiError {
    fn from(err: ArtifactError) -> Self {
        let message = err.to_string();
        match err {
            ArtifactError::NotFoundParam { param: _, path } => ApiError::new(StatusCode::NOT_FOUND)
                .message(message)
                .path(path),
            ArtifactError::InternalServerError(path) => {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message(message)
                    .path(path)
            }
        }
    }
}
