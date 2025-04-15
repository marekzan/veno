use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(crate::resources::v1::artifacts::handlers::check_versions))]
pub struct ApiDoc;
