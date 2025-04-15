use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use veno_core::app::AppState;

use super::model::NotifierResponse;

pub async fn all_notifiers(State(app): State<Arc<AppState>>) -> impl IntoResponse {
    let notifiers: Vec<NotifierResponse> = app
        .notifiers
        .iter()
        .map(|notifier| NotifierResponse::from(notifier.clone()))
        .collect();
    Json(notifiers)
}
