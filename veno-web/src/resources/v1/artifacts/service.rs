use anyhow::Result;
use futures::future::join_all;
use veno_core::artifact::Artifact;

use super::model::CheckedArtifact;

pub async fn check_all_artifacts(
    artifacts: &Vec<Artifact>,
) -> Result<Option<Vec<CheckedArtifact>>> {
    let mut new_versions = Vec::new();

    let check_futures = artifacts
        .iter()
        .map(|artifact| async move { (artifact, artifact.is_version_behind().await) });

    let checked_artifacts = join_all(check_futures).await;

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
            Err(err) => return Err(err),
        }
    }

    if new_versions.is_empty() {
        return Ok(None);
    }

    Ok(Some(new_versions))
}
