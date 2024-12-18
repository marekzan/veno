use anyhow::{Context, Result};
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

use crate::{artifact::Artifact, notifier::Notifier};

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub artifacts: Vec<Artifact>,
    pub notifiers: Vec<Notifier>,
}

impl AppConfig {
    pub fn load(file_path: &str) -> Result<Self> {
        let config = Config::builder()
            .add_source(File::new(file_path, FileFormat::Json))
            .build()?;

        let mut app_config: AppConfig = config
            .try_deserialize()
            .context("Could not deserialize config to app_config")?;
        app_config.notifiers_to_artifact_sink();
        Ok(app_config)
    }

    fn notifiers_to_artifact_sink(&mut self) {
        for artifact in &mut self.artifacts {
            for notifier_name in &artifact.notifier {
                let sink = self
                    .notifiers
                    .iter()
                    .find(|notifier| notifier.name == *notifier_name)
                    .map(|notifier| notifier.sink.clone());
                if let Some(sink) = sink {
                    artifact.sink.push(sink);
                }
            }
        }
    }

    pub async fn check_artifacts(&self) -> Result<String> {
        let mut new_versions = Vec::new();
        for artifact in &self.artifacts {
            if let Ok(Some(latest_version)) = artifact.is_version_behind().await {
                new_versions.push(CheckedArtifact {
                    name: artifact.name.clone(),
                    current_version: artifact.current_version.clone(),
                    latest_version,
                });
            }
        }
        let new_versions = serde_json::to_string(&new_versions)?;
        Ok(new_versions)
    }
}

#[derive(Debug, Serialize)]
struct CheckedArtifact {
    name: String,
    current_version: String,
    latest_version: String,
}
