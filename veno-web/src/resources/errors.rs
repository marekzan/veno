use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct ResourceError {
    pub code: u16,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl ResourceError {
    pub fn new(status_code: StatusCode) -> Self {
        let kind = match status_code.canonical_reason() {
            Some(reason) => reason.to_owned(),
            None => status_code.to_string().replace("_", " "),
        };

        Self {
            code: status_code.as_u16(),
            kind,
            ..Default::default()
        }
    }

    pub fn message<S: ToString>(mut self, message: S) -> Self {
        self.message = Some(message.to_string());
        self
    }
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_owned());
        self
    }
}

impl From<StatusCode> for ResourceError {
    fn from(status_code: StatusCode) -> Self {
        Self::new(status_code)
    }
}

impl IntoResponse for ResourceError {
    fn into_response(self) -> axum::response::Response {
        error!("Error response: {:?}", self);
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}
