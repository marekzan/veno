pub mod source;
mod version_checker;

use anyhow::Result;
use serde::Deserialize;
use source::Source;

#[derive(Deserialize, Debug, Clone)]
pub struct Artifact {
    pub id: String,
    pub name: String,
    pub message_prefix: Option<String>,
    pub source: Source,
    pub current_version: String,
}

impl Artifact {
    pub(super) async fn is_version_behind(&self) -> Result<Option<String>> {
        self.source.is_version_behind(&self.current_version).await
    }
}
