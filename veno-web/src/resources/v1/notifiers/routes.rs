use std::sync::Arc;

use axum::{routing::get, Router};
use veno_core::app::AppState;

use super::handlers::all_notifiers;

pub fn notifiers_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(all_notifiers))
}
