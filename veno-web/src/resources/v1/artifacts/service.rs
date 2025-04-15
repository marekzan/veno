use anyhow::Result;
use futures::future::join_all;
use serde::Serialize;
use veno_core::artifact::Artifact;

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
            Ok(None) => return Ok(None),
            Err(err) => return err,
        }
    }
    Ok(Some(new_versions))
}

#[derive(Debug, Serialize)]
pub struct CheckedArtifact {
    name: String,
    current_version: String,
    latest_version: String,
}
