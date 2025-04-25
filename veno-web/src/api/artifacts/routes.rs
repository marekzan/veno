use std::sync::Arc;

use axum::{routing::get, Router};
use veno_core::app::AppState;

use super::handlers::{all_artifacts, artifact_for_id, check_versions};

pub fn artifacts_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(all_artifacts))
        .route("/{artifact_id}", get(artifact_for_id))
        .route("/check", get(check_versions))
}
