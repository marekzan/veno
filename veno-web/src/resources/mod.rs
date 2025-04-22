use std::sync::Arc;

use anyhow::Result;
use errors::ResourceError;
use openapi::ApiDoc;
use serde_json::json;
use thiserror::Error;
use tower_http::cors::{Any, CorsLayer};
use tracing::trace;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use v1::v1_routes;

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

use crate::App;

mod errors;
mod openapi;
pub mod v1;

pub fn serve_api(app: Arc<App>) -> Router {
    Router::new()
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/health", get(health_handler))
        .nest("/api/v1", v1_routes())
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
pub async fn error_404_handler(request: Request) -> ResourceError {
    tracing::error!("route not found: {:?}", request);
    RootError::NotFound(format!("{:?}", request.uri())).into()
}

pub async fn health_handler() -> Result<impl IntoResponse, ResourceError> {
    Ok(Json(json!({"status": "healthy"})))
}

#[derive(Debug, Error)]
enum RootError {
    #[error("The requested route '{0}' does not exist")]
    NotFound(String),
}

impl From<RootError> for ResourceError {
    fn from(value: RootError) -> Self {
        let message = value.to_string();
        match value {
            RootError::NotFound(path) => ResourceError::new(StatusCode::NOT_FOUND)
                .message(message)
                .path(&path),
        }
    }
}
