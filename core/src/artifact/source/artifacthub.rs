use crate::{artifact::version_checker::find_newer_version, get};

use super::SourceChecker;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ArtifactHubSource {
    pub identifier: String,
}

#[derive(Deserialize, Debug)]
struct Package {
    pub version: String,
}

const ARTIFACTHUB_API: &str = "https://artifacthub.io/api/v1/packages";

impl SourceChecker for ArtifactHubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        let source = format!("{}/{}", ARTIFACTHUB_API, &self.identifier);

        let response = get(&source)
            .await
            .context("Failed to fetch latest release")?;

        // Check for successful HTTP status
        if !response.status().is_success() {
            return Err(anyhow!("Request failed: {:?}", response));
        }

        // Parse the JSON response
        let package: Package = response
            .json()
            .await
            .context("Failed to parse JSON response")?;

        Ok(find_newer_version(current_version, &package.version))
    }
}
