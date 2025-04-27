use std::sync::Arc;

use axum::{routing::get, Router};
use veno_core::app::AppState;

use super::handlers::{
    all_artifacts, artifact_for_id, artifact_for_id_check_wrapper, check_versions,
};

pub fn artifacts_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(all_artifacts))
        .route("/{artifact_id}", get(artifact_for_id))
        .route(
            "/check",
            get(artifact_for_id_check_wrapper).post(check_versions),
        )
}
