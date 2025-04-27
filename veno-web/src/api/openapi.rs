use utoipa::OpenApi;

use crate::api::{artifacts::handlers::V1ArtifactsApi, notifiers::handlers::V1NotifiersApi};

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1/artifacts", api = V1ArtifactsApi),
        (path = "/api/v1/notifiers", api = V1NotifiersApi)
    )
)]
pub struct ApiDoc;
