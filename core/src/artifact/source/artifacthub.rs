use super::SourceChecker;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ArtifactHubSource {
    pub repo: String,
}

impl SourceChecker for ArtifactHubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        println!(
            "Checking Artifacthub for new version of {}. Current version is: {}",
            self.repo, current_version
        );
        // Ok(Some("12.0.1".to_string()))
        // Ok(None)
        // Err(anyhow::anyhow!("Not implemented"))
        todo!("this is currently a test sink")
    }
}
