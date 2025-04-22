use tokio::sync::oneshot::Sender;
use veno_core::artifact::Artifact;

use crate::resources::v1::artifacts::model::ArtifactResponse;

#[derive(Debug)]
pub struct GetAllCommand {
    pub path: String,
    pub artifacts: Vec<Artifact>,
    pub responder: Sender<Vec<ArtifactResponse>>,
}

#[derive(Debug)]
pub struct CheckCommand {
    path: String,
    artifacts: Vec<Artifact>,
    responder: Sender<Result<Vec<ArtifactResponse>, CommandError>>,
}

#[derive(Debug)]
pub struct GetByIdCommand {
    path: String,
    artifacts: Vec<Artifact>,
    responder: Sender<Result<Vec<ArtifactResponse>, CommandError>>,
}

#[derive(Debug)]
pub enum ArtifactCommand {
    GetAll(GetAllCommand),
    Check(CheckCommand),
    GetById(GetByIdCommand),
}

#[derive(Debug)]
pub enum CommandError {
    ArtifactNotFound,
    InternalServerError,
}
