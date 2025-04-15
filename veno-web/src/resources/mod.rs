use std::sync::Arc;

use v1::v1_routes;
use veno_core::app::AppState;

use axum::{routing::get, Router};

mod errors;
mod v1;

// fn assert_state_bounds<T: Clone + Send + Sync + 'static>(_: &T) {}

pub async fn serve_api(app: Arc<AppState>) {
    let router = Router::new()
        .route("/", get(|| async { "Hello" }))
        .nest("/api/v1", v1_routes())
        .with_state(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
