use thiserror::Error;
use tokio::sync::oneshot::Sender;
use veno_core::{app::AppState, artifact::Artifact};

use crate::resources::v1::artifacts::model::ArtifactResponse;

#[derive(Debug)]
pub struct GetAllCommand {
    pub artifacts: Vec<Artifact>,
    pub responder: Sender<Vec<ArtifactResponse>>,
}

#[derive(Debug)]
pub struct CheckCommand<'a> {
    pub app: &'a AppState,
    pub responder: Sender<Result<Vec<ArtifactResponse>, CommandError>>,
}

#[derive(Debug)]
pub struct GetByIdCommand<'a> {
    pub artifact_id: &'a str,
    pub artifacts: Vec<Artifact>,
    pub responder: Sender<Result<Vec<ArtifactResponse>, CommandError>>,
}

#[derive(Debug)]
pub enum ArtifactCommand<'a> {
    GetAll(GetAllCommand),
    Check(CheckCommand<'a>),
    GetById(GetByIdCommand<'a>),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("The requested artifact was not found")]
    ArtifactNotFound,
    #[error("There was an internal server error")]
    InternalServerError,
}
