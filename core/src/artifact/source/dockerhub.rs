use crate::get;

use super::SourceChecker;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct DockerHubSource {
    pub repo: String,
}

const DOCKERHUB_API: &str = "https://hub.docker.com/v2/repositories";

impl SourceChecker for DockerHubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        let repository = if self.repo.contains("/") {
            &self.repo
        } else {
            &format!("library/{}", self.repo)
        };

        let source = format!("{}/{}/tags", DOCKERHUB_API, repository);

        let response = get(&source)
            .await
            .context("Failed to fetch latest release")?;

        // Check for successful HTTP status
        if !response.status().is_success() {
            return Err(anyhow!("Request failed: {:?}", response));
        }

        let response: DockerHubResponse = response
            .json()
            .await
            .context("Failed to parse JSON response")?;

        println!("Current version: {}", current_version);
        for result in response.results {
            println!("Latest version: {}", result.name);
            println!(
                "text-distance: {}",
                textdistance::str::levenshtein(current_version, &result.name)
            );
            println!("----")
        }

        Ok(Some("latest_version".to_string()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DockerHubResponse {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<DockerHubResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DockerHubResult {
    pub last_updated: String,
    pub name: String,
}
