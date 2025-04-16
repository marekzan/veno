use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1/artifacts", api = crate::resources::v1::artifacts::handlers::ArtifactsApi)
    )
)]
pub struct ApiDoc;
