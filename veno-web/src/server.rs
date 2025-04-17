use std::sync::Arc;

use tracing::info;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use veno_core::app::AppState;

use axum::{routing::get, Router};

use crate::resources::{openapi::ApiDoc, v1::v1_routes};
pub async fn run(app: Arc<AppState>) {
    let router = Router::new()
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/", get(|| async { "Hello" }))
        .nest("/api/v1", v1_routes())
        .with_state(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Axum server is now running");
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
