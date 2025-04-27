use std::sync::Arc;

use axum::{
    extract::{OriginalUri, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use thiserror::Error;
use tracing::trace;
use utoipa::OpenApi;
use veno_core::app::AppState;

use crate::api::{errors::ApiError, version::ApiVersion};

use super::model::NotifierResponse;

#[derive(OpenApi)]
#[openapi(paths(all_notifiers, notifier_for_id, notify))]
pub struct V1NotifiersApi;

#[utoipa::path(
    get,
    path="",
    responses(
        (status= OK, description = "Get all notifier configuration", body = Vec<NotifierResponse>)
    )
)]
pub async fn all_notifiers(
    version: ApiVersion,
    State(app): State<Arc<AppState>>,
) -> impl IntoResponse {
    trace!("Using API version: {}", version);
    let notifiers: Vec<NotifierResponse> = app
        .notifiers
        .iter()
        .map(|notifier| NotifierResponse::from(notifier.clone()))
        .collect();
    Json(notifiers)
}

#[utoipa::path(
    get,
    path="/{notifier_id}",
    responses(
        (status= OK, description = "Get a specific notifier with id = notifier_id", body = NotifierResponse)
    )
)]
pub async fn notifier_for_id(
    version: ApiVersion,
    original_uri: OriginalUri,
    Path((_version, notifier_id)): Path<(String, String)>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    trace!("Using API version: {}", version);
    let notifier = app
        .notifiers
        .iter()
        .find(|notifier| notifier.name == notifier_id);

    match notifier {
        Some(notifier) => {
            let response_boddy = NotifierResponse::from(notifier.clone());
            Ok((StatusCode::OK, Json(response_boddy)).into_response())
        }
        None => Err(NotifierError::NotFoundParam {
            param: notifier_id.clone(),
            path: original_uri.path(),
        }
        .into()),
    }
}

#[utoipa::path(
    get,
    path="/notify",
    responses(
        (status= OK, description = "Runs all notifiers")
    )
)]
pub async fn notify(version: ApiVersion, State(app): State<Arc<AppState>>) -> impl IntoResponse {
    trace!("Using API version: {}", version);
    app.notify().await;
    StatusCode::OK
}

#[derive(Debug, Error)]
pub enum NotifierError<'a> {
    #[error("The Notifier with the id={param} was not found.")]
    NotFoundParam { param: String, path: &'a str },
}

impl<'a> From<NotifierError<'a>> for ApiError {
    fn from(err: NotifierError) -> Self {
        let message = err.to_string();
        match err {
            NotifierError::NotFoundParam { param: _, path } => ApiError::new(StatusCode::NOT_FOUND)
                .message(message)
                .path(path),
        }
    }
}
