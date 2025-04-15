use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use veno_core::app::AppState;

use crate::resources::errors::PathParamError;

use super::{model::ArtifactResponse, service::check_all_artifacts};

pub async fn check_versions(State(app): State<Arc<AppState>>) -> impl IntoResponse {
    let response = check_all_artifacts(&app.artifacts).await;
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        Json(response),
    )
}

pub async fn all_artifacts(State(app): State<Arc<AppState>>) -> impl IntoResponse {
    let artifacts: Vec<ArtifactResponse> = app
        .artifacts
        .iter()
        .map(|artifact| ArtifactResponse::from(artifact.clone()))
        .collect();
    Json(artifacts)
}

pub async fn artifact_for_id(
    Path(artifact_id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> impl IntoResponse {
    let artifact = app
        .artifacts
        .iter()
        .find(|artifact| artifact.id == artifact_id);

    match artifact {
        Some(artifact) => {
            let response_boddy = ArtifactResponse::from(artifact.clone());
            (StatusCode::OK, Json(response_boddy)).into_response()
        }
        None => {
            let error = PathParamError {
                error_code: StatusCode::NOT_FOUND.to_string(),
                resource: String::from("artifacts"),
                param: artifact_id.clone(),
                message: format!("The Artifact with the id '{}' was not found", artifact_id),
            };
            (StatusCode::NOT_FOUND, Json(error)).into_response()
        }
    }
}
