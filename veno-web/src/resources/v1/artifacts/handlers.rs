use std::{fmt::Display, sync::Arc, time::Duration};

use crate::{
    commands::{
        artifact_commands::{ArtifactCommand, GetAllCommand},
        Command,
    },
    resources::errors::ResourceError,
    App,
};
use axum::{
    extract::{OriginalUri, Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use thiserror::Error;
use tokio::{
    sync::oneshot,
    time::{timeout, Timeout},
};
use tracing::{error, info};
use utoipa::OpenApi;

use super::{model::ArtifactResponse, service::check_all_artifacts};

#[derive(OpenApi)]
#[openapi(paths(check_versions, all_artifacts, artifact_for_id))]
pub struct V1ArtifactsApi;

#[utoipa::path(
    get,
    path="/check",
    responses(
        (status= OK, description = "Returns a set of checked artifacts with its new versions if there are any.", body = ArtifactResponse),
        (status= OK, description = "Retursn a message if there are no new versions", body = serde_json::Value),
        (status= INTERNAL_SERVER_ERROR, description = "If during the check a server error occurs", body = ResourceError)
    )
)]
#[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
pub async fn check_versions(
    State(app): State<Arc<App>>,
    uri: OriginalUri,
) -> Result<impl IntoResponse, ResourceError> {
    let response = check_all_artifacts(&app.config).await;
    match response {
        Ok(Some(new_versions)) => return Ok(Json(new_versions).into_response()),
        Ok(None) => {
            return Ok(Json(
                json!({"message": "There are currently no new versions of your artifacts"}),
            )
            .into_response())
        }
        Err(_err) => Err(ArtifactError::InternalServerError(uri.0).into()),
    }
}

#[utoipa::path(
    get,
    path="/",
    responses(
        (status= OK, description = "Get all artifact configurations", body = Vec<ArtifactResponse>)
    )
)]
#[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
pub async fn all_artifacts(
    State(app): State<Arc<App>>,
    uri: OriginalUri,
) -> Result<impl IntoResponse, ResourceError> {
    info!("{:?}", uri);
    let (responder_tx, responder_rx) = oneshot::channel();
    let command = GetAllCommand {
        path: "/api/v1/artifacts".into(),
        artifacts: app.config.artifacts.clone(),
        responder: responder_tx,
    };

    // app.command_tx
    //     .send(Command::Artifact(ArtifactCommand::GetAll(command)))
    //     .await
    //     .map_err(|_err| ArtifactError::InternalServerError(uri.clone()))?;

    // match responder_rx.await {
    //     Ok(artifacts) => return Ok(Json(artifacts)),
    //     Err(_err) => return Err(ArtifactError::InternalServerError(uri).into()),
    // };

    match app
        .command_tx
        .send(Command::Artifact(ArtifactCommand::GetAll(command)))
        .await
    {
        Ok(_) => match timeout(Duration::from_secs(10), responder_rx).await {
            Ok(Ok(artifacts)) => Ok(Json(artifacts)),
            Ok(Err(err)) => {
                error!("{}", err);
                return Err(ArtifactError::InternalServerError(uri.0).into());
            }
            Err(err) => {
                error!("{}", err);
                return Err(ArtifactError::TimeoutError(uri.0).into());
            }
        },
        Err(err) => {
            error!("{}", err);
            return Err(ArtifactError::InternalServerError(uri.0).into());
        }
    }

    // Json(artifacts)
}

#[utoipa::path(
    get,
    path="/{artifact_id}",
    responses(
        (status= OK, description = "Get a specific artifact with id = artifact_id", body = ArtifactResponse),
        (status= NOT_FOUND, description = "Returns not_found if the artifact_id had no match", body = ResourceError),
    )
)]
#[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
pub async fn artifact_for_id(
    Path(artifact_id): Path<String>,
    State(app): State<Arc<App>>,
) -> Result<impl IntoResponse, ResourceError> {
    let artifact = app
        .config
        .artifacts
        .iter()
        .find(|artifact| artifact.id == artifact_id);

    match artifact {
        Some(artifact) => {
            let response_boddy = ArtifactResponse::from(artifact.clone());
            Ok((StatusCode::OK, Json(response_boddy)).into_response())
        }
        None => Err(ArtifactError::NotFoundWithParam {
            param: artifact_id.clone(),
            path: format!("/api/v1/artifacts/{artifact_id}"),
        }
        .into()),
    }
}

#[derive(Debug, Error)]
pub enum ArtifactError {
    #[error("The artifact with the id={param} was not found.")]
    NotFoundWithParam { param: String, path: String },
    #[error("There was an internal server error. Please try again later.")]
    InternalServerError(Uri),
    #[error("The request took to long to respond. Aborting... Please try again later")]
    TimeoutError(Uri),
}

impl From<ArtifactError> for ResourceError {
    fn from(err: ArtifactError) -> Self {
        let message = err.to_string();
        match err {
            ArtifactError::NotFoundWithParam { param: _, path } => {
                ResourceError::new(StatusCode::NOT_FOUND)
                    .message(message)
                    .path(format!("{}", path).as_str())
            }
            ArtifactError::InternalServerError(path) => {
                ResourceError::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message(message)
                    .path(format!("{}", path).as_str())
            }
            ArtifactError::TimeoutError(path) => ResourceError::new(StatusCode::REQUEST_TIMEOUT)
                .message(message)
                .path(format!("{}", path).as_str()),
        }
    }
}
