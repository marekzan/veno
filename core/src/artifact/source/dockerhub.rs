use super::SourceChecker;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DockerHubSource {
    pub repo: String,
}

impl SourceChecker for DockerHubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        todo!()
    }
}
