use crate::{artifact::version_checker::find_newer_version, get};

use super::SourceChecker;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct DockerHubSource {
    pub identifier: String,
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

const DOCKERHUB_API: &str = "https://hub.docker.com/v2/repositories";
const PAGE_SIZE_QUERY_PARAM: &str = "page_size=200";

impl SourceChecker for DockerHubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        let source = build_dockerhub_url(&self.identifier);
        let response = fetch_dockerhub_tags(&source).await?;

        // TODO: we need to fetch the next results of the paginated response
        // when there were no matches an the current version was not in the first response
        let newer_versions = response
            .results
            .iter()
            .filter_map(|result| find_newer_version(current_version, &result.name))
            .collect::<Vec<String>>();

        if newer_versions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(newer_versions.join(", ")))
        }
    }
}

async fn fetch_dockerhub_tags(source: &str) -> Result<DockerHubResponse> {
    let response = get(source)
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
    Ok(response)
}

fn build_dockerhub_url(repo: &str) -> String {
    let repository = if repo.contains("/") {
        repo
    } else {
        &format!("library/{}", repo)
    };

    format!(
        "{}/{}/tags?{}",
        DOCKERHUB_API, repository, PAGE_SIZE_QUERY_PARAM
    )
}
