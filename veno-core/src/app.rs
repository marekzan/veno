use anyhow::{Context, Result};
use futures::future::join_all;
use serde::Deserialize;
use tracing::trace;

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

    // TODO we can move the check_all_artifacts function from web to core and use it here
    // for that we need to change it to check_artifacts with a &[Artifact] slice as argument
    pub async fn notify(&self) {
        for notifier in &self.notifiers {
            if notifier.artifact_ids.len() > 0 {
                // get all artifacts that match this notifier
                let matched_artifacts = self
                    .artifacts
                    .iter()
                    .filter(|artifact| notifier.artifact_ids.contains(&artifact.id))
                    .collect::<Vec<&Artifact>>();

                let check_futures = matched_artifacts
                    .iter()
                    .map(|artifact| async move { (*artifact, artifact.is_version_behind().await) });
                // TODO do we want to also return the successful and failed artifacts in the NotifierResponse?
                let checked_artifacts = join_all(check_futures).await;

                let notification = generate_notification(&checked_artifacts).await;
                match notifier.sink.send(&notification).await {
                    // TODO return a NotifierResponse with the status (Email: success, artifacts: { artifact1: success } ; GoogleChat: failed ; Slack: partialSuccess, artifacts: {artifact1: success, artifact2: failed})
                    Ok(()) => {}
                    Err(e) => trace!("{e}"),
                }
            }
        }
    }
}

struct NotifierResponse {
    notifier_result: Vec<NotifierResult>,
}

struct NotifierResult {
    name: String,
    status: NotifierStatus,
    artifacts: Vec<ArtifactResult>,
}

struct ArtifactResult {
    name: String,
    status: ArtifactStatus,
}

enum NotifierStatus {
    Success,
    PartialSuccess,
    Failed,
}

enum ArtifactStatus {
    Success,
    Failed,
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
