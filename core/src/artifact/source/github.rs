use super::SourceChecker;
use crate::CLIENT;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct GithubSource {
    pub repo: String,
}

impl SourceChecker for GithubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        let source = format!("https://api.github.com/repos/{}/releases/latest", self.repo);

        // Send the HTTP request
        let response = CLIENT
            .get(&source)
            .header("User-Agent", "veno")
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .context("Failed to fetch latest release")?;

        // Check for successful HTTP status
        if !response.status().is_success() {
            return Err(anyhow!("Request failed: {:?}", response));
        }

        // Parse the JSON response
        let release: Release = response
            .json()
            .await
            .context("Failed to parse JSON response")?;

        // Extract and compare the version
        let latest_version = release.tag_name.trim_start_matches('v');
        if latest_version > current_version {
            Ok(Some(latest_version.to_string()))
        } else {
            Ok(None)
        }
    }
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}
