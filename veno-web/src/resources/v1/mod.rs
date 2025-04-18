use std::sync::Arc;

use artifacts::routes::artifacts_routes;
use axum::Router;
use notifiers::routes::notifiers_routes;
use veno_core::app::AppState;

pub mod artifacts;
pub mod notifiers;

pub fn v1_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/artifacts", artifacts_routes())
        .nest("/notifiers", notifiers_routes())
}
