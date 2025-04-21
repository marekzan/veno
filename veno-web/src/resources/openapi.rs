use crate::resources::v1::artifacts::handlers::V1ArtifactsApi;
use crate::resources::v1::notifiers::handlers::V1NotifiersApi;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1/artifacts", api = V1ArtifactsApi),
        (path = "/api/v1/notifiers", api = V1NotifiersApi)
    )
)]
pub struct ApiDoc;
