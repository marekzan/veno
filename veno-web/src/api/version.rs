use std::collections::HashMap;

use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};

use thiserror::Error;

use super::errors::ApiError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ApiVersion {
    V1,
}

impl ApiVersion {
    pub fn parse_version(version: &str) -> Result<ApiVersion, ApiError> {
        match version.parse() {
            Ok(version) => Ok(version),
            Err(_) => Err(ApiVersionError::InvalidVersion {
                version: version.to_owned(),
            }
            .into()),
        }
    }
}

impl std::str::FromStr for ApiVersion {
    type Err = ApiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1" => Ok(Self::V1),
            _ => Err(ApiVersionError::VersionParseError.into()),
        }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::V1 => "v1",
        };
        write!(f, "{}", v)
    }
}

impl<S> FromRequestParts<S> for ApiVersion
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> = parts
            .extract()
            .await
            .map_err(|_| ApiVersionError::VersionExtractError)?;

        let version = params
            .get("version")
            // This does not trigger since the 404_handler fallback catches before this extractor gets called
            .ok_or(ApiVersionError::ParameterMissing)?;

        ApiVersion::parse_version(version)
    }
}

#[derive(Debug, Error)]
pub enum ApiVersionError {
    #[error("The version parameter is invalid: '{version}'")]
    InvalidVersion { version: String },
    #[error("Parameter is missing: 'version'")]
    ParameterMissing,
    #[error("Could not extract api version")]
    VersionExtractError,
    #[error("Could not parse version from request")]
    VersionParseError,
}

impl From<ApiVersionError> for ApiError {
    fn from(err: ApiVersionError) -> Self {
        let message = err.to_string();
        match err {
            ApiVersionError::InvalidVersion { version: _version } => {
                ApiError::new(StatusCode::BAD_REQUEST).message(message)
            }
            ApiVersionError::VersionExtractError => {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).message(message)
            }
            ApiVersionError::ParameterMissing => {
                ApiError::new(StatusCode::BAD_REQUEST).message(message)
            }
            ApiVersionError::VersionParseError => {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR).message(message)
            }
        }
    }
}
