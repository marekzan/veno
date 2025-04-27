use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use veno_core::app::AppState;

use super::handlers::{all_notifiers, notifier_for_id, notify};

pub fn notifiers_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(all_notifiers))
        .route("/{notifier_id}", get(notifier_for_id))
        .route("/notify", post(notify))
}
