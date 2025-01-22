use std::vec;

use crate::get;

use super::SourceChecker;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct DockerHubSource {
    pub repo: String,
}

const DOCKERHUB_API: &str = "https://hub.docker.com/v2/repositories";
const PAGE_SIZE_QUERY_PARAM: &str = "page_size=200";

impl SourceChecker for DockerHubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        let split_current_version = split_version(current_version);
        let version_schema = get_schema(&split_current_version);

        let repository = if self.repo.contains("/") {
            &self.repo
        } else {
            &format!("library/{}", self.repo)
        };

        let source = format!(
            "{}/{}/tags?{}",
            DOCKERHUB_API, repository, PAGE_SIZE_QUERY_PARAM
        );

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

        let mut matched_versions: Vec<String> = vec![];
        for result in response.results {
            if let Some(version) =
                match_version(&split_current_version, &version_schema, &result.name)
            {
                matched_versions.push(version);
            }
        }

        // FIXME: this can return older version. we still need to check if the matched versions are
        // newer than the current version
        Ok(Some(matched_versions.join(", ")))
    }
}

fn split_version(version: &str) -> Vec<&str> {
    let separators = ['.', '-', ':']; // Define your separators
    let parts: Vec<&str> = version.split(|c| separators.contains(&c)).collect();

    parts
}

// NOTE: this can be used for matching what versions to match. if i only want major and minor i
// could to smth like: y-y-n-n-n on a version like 1.2.0-stable-alpine
#[derive(Debug, PartialEq, Eq)]
enum VersionPartType {
    Number,
    String,
}

fn get_schema(split_version: &[&str]) -> Vec<VersionPartType> {
    split_version
        .iter()
        .map(|part| {
            if part.parse::<i32>().is_ok() {
                VersionPartType::Number
            } else {
                VersionPartType::String
            }
        })
        .collect()
}

fn match_version(
    split_curr_version: &[&str],
    schema_curr_version: &[VersionPartType],
    new_version: &str,
) -> Option<String> {
    let split_new_version = split_version(new_version);
    let schema_new_version = get_schema(&split_new_version);

    // if the schema of both match and the string parts match then we have the correct version
    if schema_curr_version == schema_new_version
        && split_curr_version
            .iter()
            .zip(&split_new_version)
            .enumerate()
            .all(
                |(i, (curr_version_part, new_version_part))| match schema_curr_version[i] {
                    VersionPartType::Number => true,
                    VersionPartType::String => curr_version_part == new_version_part,
                },
            )
    {
        return Some(new_version.to_string());
    }
    None
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
