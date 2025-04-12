use anyhow::{Context, Result};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::{
    artifact::Artifact,
    config::AppConfig,
    notifier::{create_custom_message, create_default_message, Notifier},
};

#[derive(Deserialize, Debug, Clone)]
pub struct AppState {
    pub artifacts: Vec<Artifact>,
    pub notifiers: Vec<Notifier>,
}

impl AppState {
    pub fn init(file_path: &str) -> Result<Self> {
        let app_config = AppConfig::load(file_path)?;
        let app_state = app_config
            .try_deserialize()
            .context("Could not deserialize app config")?;
        Ok(app_state)
    }

    pub async fn notify(&self) {
        for notifier in &self.notifiers {
            // get all artifacts that match this notifier
            let matched_artifacts = self
                .artifacts
                .iter()
                .filter(|artifact| notifier.artifact_ids.contains(&artifact.id))
                .collect::<Vec<&Artifact>>();

            let check_futures = matched_artifacts
                .iter()
                .map(|artifact| async move { (*artifact, artifact.is_version_behind().await) });
            let checked_artifacts = join_all(check_futures).await;

            let notification = generate_notification(&checked_artifacts).await;
            notifier.sink.send(&notification).await;
        }
    }

    // This function will check all  artifacts if they are behind the latest version
    pub async fn check_all_artifacts(&self) -> Result<String> {
        let mut new_versions = Vec::new();

        let check_futures = self
            .artifacts
            .iter()
            .map(|artifact| async move { (artifact, artifact.is_version_behind().await) });

        let checked_artifacts = join_all(check_futures).await;

        for (artifact, result) in checked_artifacts {
            if let Ok(Some(latest_version)) = result {
                new_versions.push(CheckedArtifact {
                    name: artifact.name.clone(),
                    current_version: artifact.current_version.clone(),
                    latest_version,
                });
            }
        }
        let new_versions =
            serde_json::to_string(&new_versions).context("Failed to serialize new versions")?;
        Ok(new_versions)
    }
}

async fn generate_notification(artifacts: &Vec<(&Artifact, Result<Option<String>>)>) -> String {
    let mut messages: Vec<String> = vec![];

    for (artifact, result) in artifacts {
        match result {
            Ok(Some(new_version)) => {
                let message = match &artifact.message_prefix {
                    Some(prefix) => create_custom_message(&prefix, &artifact.name, &new_version),
                    None => create_default_message(&artifact.name, &new_version),
                };
                messages.push(message);
            }
            Ok(None) => println!("There is no new version for artifact {}", artifact.name),
            Err(err) => eprintln!(
                "There was an error while checking the new version for artifact {}: {}",
                artifact.name, err
            ),
        };
    }

    messages.join("\n")
}

#[derive(Debug, Serialize)]
struct CheckedArtifact {
    name: String,
    current_version: String,
    latest_version: String,
}
