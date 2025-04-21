use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use thiserror::Error;
use utoipa::OpenApi;
use veno_core::app::AppState;

use crate::resources::errors::ResourceError;

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
pub async fn all_notifiers(State(app): State<Arc<AppState>>) -> impl IntoResponse {
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
    Path(notifier_id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ResourceError> {
    let notifier = app
        .notifiers
        .iter()
        .find(|notifier| notifier.name == notifier_id);

    match notifier {
        Some(notifier) => {
            let response_boddy = NotifierResponse::from(notifier.clone());
            Ok((StatusCode::OK, Json(response_boddy)).into_response())
        }
        None => Err(NotifierError::NotFoundWithParam {
            param: notifier_id.clone(),
            path: format!("/api/v1/notifiers/{notifier_id}"),
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
pub async fn notify(State(app): State<Arc<AppState>>) -> impl IntoResponse {
    app.notify().await;
    StatusCode::OK
}

#[derive(Debug, Error)]
pub enum NotifierError {
    #[error("The Notifier with the id={param} was not found.")]
    NotFoundWithParam { param: String, path: String },
}

impl From<NotifierError> for ResourceError {
    fn from(err: NotifierError) -> Self {
        let message = err.to_string();
        match err {
            NotifierError::NotFoundWithParam { param: _, path } => {
                ResourceError::new(StatusCode::NOT_FOUND)
                    .message(message)
                    .path(format!("{}", path).as_str())
            }
        }
    }
}
