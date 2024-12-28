pub mod source;

use anyhow::{Context, Result};
use serde::Deserialize;
use source::Source;

use crate::notifier::{create_custom_message, create_default_message, Sink};

#[derive(Deserialize, Debug, Clone)]
pub struct Artifact {
    pub name: String,
    pub message_prefix: Option<String>,
    pub source: Source,
    pub current_version: String,
    pub notifier: Vec<String>,
    #[serde(skip)]
    pub sink: Vec<Sink>,
}

impl Artifact {
    pub async fn is_version_behind(&self) -> Result<Option<String>> {
        self.source.is_version_behind(&self.current_version).await
    }

    pub async fn notify_on_latest_version(&self) -> Result<()> {
        if let Some(latest_version) = self
            .is_version_behind()
            .await
            .context("Failed to check version")?
        {
            for sink in &self.sink {
                let message = match &self.message_prefix {
                    Some(prefix) => {
                        create_custom_message(prefix, &self.name, latest_version.as_str())
                    }
                    None => create_default_message(&self.name, latest_version.as_str()),
                };
                sink.send(&message).await
            }
        }
        Ok(())
    }
}
