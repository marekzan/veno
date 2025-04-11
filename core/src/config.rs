use std::{borrow::Cow, env, fs};

use anyhow::{Context, Result};
use config::{Config, File, FileFormat};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};

use crate::{artifact::Artifact, notifier::Notifier};

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub artifacts: Vec<Artifact>,
    pub notifiers: Vec<Notifier>,
}

impl AppConfig {
    pub fn load(file_path: &str) -> Result<Self> {
        let mut config = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read config file {}", file_path))?;

        replace_env_placeholders(&mut config);

        let config = Config::builder()
            .add_source(File::from_str(&config, FileFormat::Json))
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

static RE_ENV: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{([^}]+)\}").expect("Invalid regex"));

fn replace_env_placeholders(config: &mut String) {
    let result = RE_ENV.replace_all(config, |caps: &Captures| {
        // caps[0] is "${VAR}"
        // caps[1] is "VAR"
        let var_name = &caps[1];
        match env::var(var_name) {
            Ok(val) => val,
            Err(_) => panic!("Could not find env.var = {}", &caps[0]),
        }
    });

    if let Cow::Owned(new_content) = result {
        *config = new_content;
    }
}

#[derive(Debug, Serialize)]
struct CheckedArtifact {
    name: String,
    current_version: String,
    latest_version: String,
}

#[cfg(test)]
mod tests {
    use crate::{
        artifact::source::Source,
        notifier::{google_chat::GoogleChatSink, slack::SlackSink, Sink},
    };

    use super::*;

    #[test]
    fn test_notifiers_to_artifact_sink() {
        let google_chat_sink = GoogleChatSink {
            webhook: "webhook".to_string(),
        };

        let slack_sink = SlackSink {
            webhook: "webhook".to_string(),
        };

        let notifier1 = Notifier {
            name: "notifier1".to_string(),
            sink: Sink::GoogleChat(google_chat_sink),
        };
        let notifier2 = Notifier {
            name: "notifier2".to_string(),
            sink: Sink::Slack(slack_sink),
        };

        let artifact = Artifact {
            name: "artifact1".to_string(),
            message_prefix: None,
            notifier: vec!["notifier1".to_string(), "notifier2".to_string()],
            source: Source::GitHub(crate::artifact::source::github::GitHubSource {
                identifier: "repo".to_string(),
            }),
            current_version: "1.0.0".to_string(),
            sink: vec![],
        };

        let mut config = AppConfig {
            artifacts: vec![artifact],
            notifiers: vec![notifier1, notifier2],
        };

        config.notifiers_to_artifact_sink();

        assert_eq!(config.artifacts[0].sink.len(), 2);
    }

    #[test]
    fn test_notifiers_to_artifact_sink_without_reference() {
        let google_chat_sink = GoogleChatSink {
            webhook: "webhook".to_string(),
        };

        let slack_sink = SlackSink {
            webhook: "webhook".to_string(),
        };

        let notifier1 = Notifier {
            name: "notifier1".to_string(),
            sink: Sink::GoogleChat(google_chat_sink),
        };
        let notifier2 = Notifier {
            name: "notifier2".to_string(),
            sink: Sink::Slack(slack_sink),
        };

        let artifact = Artifact {
            name: "artifact1".to_string(),
            message_prefix: None,
            notifier: vec![],
            source: Source::GitHub(crate::artifact::source::github::GitHubSource {
                identifier: "repo".to_string(),
            }),
            current_version: "1.0.0".to_string(),
            sink: vec![],
        };

        let mut config = AppConfig {
            artifacts: vec![artifact],
            notifiers: vec![notifier1, notifier2],
        };

        config.notifiers_to_artifact_sink();

        assert_eq!(config.artifacts[0].sink.len(), 0);
    }

    // this should probably be an error
    #[test]
    fn test_notifiers_to_artifact_sink_missing() {
        let artifact = Artifact {
            name: "artifact1".to_string(),
            message_prefix: None,
            notifier: vec!["notifier1".to_string(), "notifier2".to_string()],
            source: Source::GitHub(crate::artifact::source::github::GitHubSource {
                identifier: "repo".to_string(),
            }),
            current_version: "1.0.0".to_string(),
            sink: vec![],
        };

        let mut config = AppConfig {
            artifacts: vec![artifact],
            notifiers: vec![],
        };

        config.notifiers_to_artifact_sink();

        assert_eq!(config.artifacts[0].sink.len(), 0);
    }

    #[test]
    fn test_replace_env_placeholders() {
        env::set_var("TEST_VAR", "test_value");
        let mut config = "key=${TEST_VAR}".to_string();
        replace_env_placeholders(&mut config);
        assert_eq!(config, "key=test_value");
    }
}
