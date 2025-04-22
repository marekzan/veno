use crate::resources::v1::artifacts::model::ArtifactResponse;

use super::artifact_commands::{CommandError, GetAllCommand};

pub fn handle_get_all_artifacts(command: GetAllCommand) -> Result<(), CommandError> {
    let artifacts: Vec<ArtifactResponse> = command
        .artifacts
        .iter()
        .map(|artifact| ArtifactResponse::from(artifact.clone()))
        .collect();
    std::thread::sleep(std::time::Duration::from_secs(4));
    if let Err(_err) = command.responder.send(artifacts) {
        return Err(CommandError::InternalServerError);
    }
    Ok(())
}
