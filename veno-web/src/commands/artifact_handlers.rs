use tracing::error;

use crate::resources::v1::artifacts::model::{ArtifactResponse, CheckedArtifact};

use super::artifact_commands::{CheckCommand, CommandError, GetAllCommand, GetByIdCommand};

pub fn handle_get_all_artifacts(command: GetAllCommand) {
    let artifacts: Vec<ArtifactResponse> = command
        .artifacts
        .iter()
        .map(|artifact| ArtifactResponse::from(artifact.clone()))
        .collect();
    if let Err(_err) = command.responder.send(artifacts) {
        error!("{}", CommandError::InternalServerError)
    }
}

pub async fn handle_check<'a>(
    command: CheckCommand<'_>,
) -> {
    let mut new_versions: Vec<CheckedArtifact> = Vec::new();

    let checked_artifacts = command.app.check_all_artifacts().await;

    for (artifact, result) in checked_artifacts {
        match result {
            Ok(Some(latest_version)) => {
                new_versions.push(CheckedArtifact {
                    name: artifact.name.clone(),
                    current_version: artifact.current_version.clone(),
                    latest_version,
                });
            }
            Ok(None) => {}
            Err(err) => {
                error!(
                    "An error occured while checking for a new version for {}\n{}",
                    artifact.name, err
                );
            }
        }
    }

    if new_versions.is_empty() {
        return Ok(None);
    }

    Ok(Some(new_versions))

    if let Err(_err) = command.responder.send(artifacts) {
        error!("{}", CommandError::InternalServerError)
    }
}

pub fn handle_get_by_id(command: GetByIdCommand) -> Result<(), CommandError> {
    let artifact = command
        .artifacts
        .iter()
        .find(|artifact| artifact.id == artifact_id);

    match artifact {
        Some(artifact) => {
            let response_boddy = ArtifactResponse::from(artifact.clone());
            Ok(Json(response_boddy))
        }
        None => Err(ArtifactError::NotFoundWithParam {
            param: artifact_id.clone(),
            path: format!("/api/v1/artifacts/{artifact_id}"),
        }
        .into()),
    }
}
