use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct PathParamError {
    pub error_code: String,
    pub resource: String,
    pub param: String,
    pub message: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct InternalServerError {
    pub error_code: String,
    pub message: String,
}
