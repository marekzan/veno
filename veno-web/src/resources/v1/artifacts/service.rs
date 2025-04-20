use anyhow::Result;
use tracing::error;
use veno_core::app::AppState;

use super::model::CheckedArtifact;

pub async fn check_all_artifacts(app: &AppState) -> Result<Option<Vec<CheckedArtifact>>> {
    let mut new_versions = Vec::new();

    let checked_artifacts = app.check_all_artifacts().await;

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
                return Err(err);
            }
        }
    }

    if new_versions.is_empty() {
        return Ok(None);
    }

    Ok(Some(new_versions))
}
