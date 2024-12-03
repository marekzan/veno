use anyhow::Result;
use serde::Deserialize;

use crate::sink::{create_custom_message, create_default_message, Sink};

pub mod checker;

#[derive(Deserialize, Debug, Clone)]
pub struct Artifact {
    pub name: String,
    pub message_prefix: Option<String>,
    pub source: String,
    pub current_version: String,
    pub notifier: Vec<String>,
    #[serde(skip)]
    pub sink: Vec<Sink>,
}

impl Artifact {
    pub async fn check_version(&self) -> Result<Option<String>> {
        checker::check(self).await
    }

    pub async fn notify(&self, latest_version: &str) -> Result<()> {
        for sink in &self.sink {
            match &self.message_prefix {
                Some(prefix) => {
                    let message = create_custom_message(prefix, &self.name, latest_version);
                    sink.get().send(&message).await?
                }
                None => {
                    let message = create_default_message(&self.name, latest_version);
                    sink.get().send(&message).await?
                }
            };
        }
        Ok(())
    }
}
