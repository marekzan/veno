use anyhow::{Context, Result};
use futures::future::join_all;
use serde::Deserialize;
use tracing::{error, info};

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

    // TODO implement error handling
    // currently generate_notification and sink::send do not return a result
    pub async fn notify(&self) {
        for notifier in &self.notifiers {
            // get all artifacts that match this notifier
            let matched_artifacts = self
                .artifacts
                .iter()
                .filter(|artifact| notifier.artifact_ids.contains(&artifact.id))
                .collect::<Vec<&Artifact>>();

            let checked_artifacts = Self::check_artifacts(matched_artifacts).await;

            let notification = generate_notification(&checked_artifacts);
            notifier.sink.send(&notification).await;
        }
    }

    pub async fn check_all_artifacts(&self) -> Vec<(&Artifact, Result<Option<String>>)> {
        Self::check_artifacts(&self.artifacts).await
    }

    pub async fn check_artifacts<'a, I>(
        artifacts_iter: I,
    ) -> Vec<(&'a Artifact, Result<Option<String>>)>
    where
        I: IntoIterator<Item = &'a Artifact>,
    {
        let check_futures = artifacts_iter.into_iter().map(|artifact| async move {
            (
                artifact,
                artifact
                    .source
                    .check_new_version(&artifact.current_version)
                    .await,
            )
        });

        join_all(check_futures).await
    }
}

fn generate_notification(artifacts: &Vec<(&Artifact, Result<Option<String>>)>) -> String {
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
            Ok(None) => info!("There is no new version for artifact {}", artifact.name),
            Err(err) => error!(
                "There was an error while checking the new version for artifact {}: {}",
                artifact.name, err
            ),
        };
    }

    messages.join("\n")
}
