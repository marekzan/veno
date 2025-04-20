use super::SourceChecker;
use crate::get;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GitHubSource {
    pub identifier: String,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

const GITHUB_API: &str = "https://api.github.com/repos";

impl SourceChecker for GitHubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        let source = format!("{}/{}/releases/latest", GITHUB_API, self.identifier);

        // Send the HTTP request
        let response = get(&source).await.context(format!(
            "Failed to fetch latest release for {} via github api",
            &self.identifier
        ))?;

        // Check for successful HTTP status
        if !response.status().is_success() {
            return Err(anyhow!("Request failed: {:?}", response));
        }

        let release: Release = response.json().await?;

        // Extract and compare the version
        // TODO: change this logic to be used with the version checker but we first need to
        // implement the case where the version starts with a v (v2.1.0) since the version checker
        // will declare the v2 as a string and not as a number
        let latest_version = release.tag_name.trim_start_matches('v');
        if latest_version > current_version {
            Ok(Some(latest_version.to_string()))
        } else {
            Ok(None)
        }
    }
}
