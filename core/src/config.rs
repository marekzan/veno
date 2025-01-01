use std::{env, fs};

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
        let raw_config = fs::read_to_string(file_path).context("Failed to read config file")?;
        let modified_config_data = replace_env_placeholders(raw_config);

        let config = Config::builder()
            .add_source(File::from_str(&modified_config_data, FileFormat::Json))
            .build()
            .context("Failed to load config file")?;

        let mut app_config: AppConfig = config
            .try_deserialize()
            .context("Could not deserialize config to app_config struct")?;

        app_config.notifiers_to_artifact_sink();

        Ok(app_config)
    }

    // This function will populate the sink field of each artifact with the sink of the notifiers
    fn notifiers_to_artifact_sink(&mut self) {
        for artifact in &mut self.artifacts {
            let sinks: Vec<_> = artifact
                .notifier
                .iter()
                .filter_map(|notifier_name| {
                    self.notifiers
                        .iter()
                        .find(|notifier| notifier.name == *notifier_name)
                        .map(|notifier| notifier.sink.clone())
                })
                .collect();
            artifact.sink.extend(sinks);
        }
    }

    // This function will check if the artifacts are behind the latest version
    pub async fn check_artifacts(&self) -> Result<String> {
        let mut new_versions = Vec::new();
        for artifact in &self.artifacts {
            if let Some(latest_version) = artifact.is_version_behind().await? {
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

// TODO: maybe make the regex a static variable with lazy_static?
//
//  lazy_static! {
//    static ref RE: Regex = Regex::new(r"\$\{([^}]+)\}").unwrap();
//  }
fn replace_env_placeholders(config: String) -> String {
    // Use a regex to find and replace placeholders
    let re = regex::Regex::new(r"\$\{([^}]+)\}").unwrap();
    re.replace_all(&config, |caps: &regex::Captures| {
        env::var(&caps[1]).unwrap_or_else(|_| caps[0].to_string())
    })
    .into_owned()
}

#[derive(Debug, Serialize)]
struct CheckedArtifact {
    name: String,
    current_version: String,
    latest_version: String,
}
