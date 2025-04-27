mod artifacts;
mod errors;
mod notifiers;
mod openapi;
mod version;

use anyhow::Result;
use artifacts::routes::artifacts_routes;
use errors::ApiError;
use notifiers::routes::notifiers_routes;
use openapi::ApiDoc;
use serde_json::json;
use std::sync::Arc;
use thiserror::Error;
use tower_http::cors::{Any, CorsLayer};
use tracing::trace;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use veno_core::app::AppState;

use axum::{
    body::Body,
    extract::Request,
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Method, StatusCode,
    },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

pub fn serve_api(app: Arc<AppState>) -> Router {
    Router::new()
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/health", get(health_handler))
        .nest("/api/{version}/artifacts", artifacts_routes())
        .nest("/api/{version}/notifiers", notifiers_routes())
        .fallback(error_404_handler)
        .layer(cors_layer())
        .layer(middleware::from_fn(logging_middleware))
        .with_state(app)
}

#[tracing::instrument(level = tracing::Level::TRACE, name = "axum_request", skip_all, fields(method=request.method().to_string(), uri=request.uri().to_string()))]
pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response {
    trace!("received a request",);
    next.run(request).await
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
}
pub async fn error_404_handler(request: Request) -> ApiError {
    tracing::error!("route not found: {:?}", request);
    RootError::NotFound(format!("{:?}", request.uri())).into()
}

pub async fn health_handler() -> Result<impl IntoResponse, ApiError> {
    Ok(Json(json!({"status": "healthy"})))
}

#[derive(Debug, Error)]
enum RootError {
    #[error("The requested route '{0}' does not exist")]
    NotFound(String),
}

impl From<RootError> for ApiError {
    fn from(value: RootError) -> Self {
        let message = value.to_string();
        match value {
            RootError::NotFound(path) => ApiError::new(StatusCode::NOT_FOUND)
                .message(message)
                .path(&path),
        }
    }
}
