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
        let curr_v_split = split_version(current_version);
        let curr_v_tokens = infer_tokens(&curr_v_split);

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

        let compatible_versions = response
            .results
            .iter()
            .filter_map(|result| {
                find_compatible_version(&curr_v_split, &curr_v_tokens, &result.name)
            })
            .collect::<Vec<String>>();

        Ok(Some(compatible_versions.join(", ")))
    }
}

fn split_version(version: &str) -> Vec<&str> {
    let separators = ['.', '-', ':']; // Define your separators
    version.split(|c| separators.contains(&c)).collect()
}

// NOTE: this can be used for matching what versions to match. if i only want major and minor i
// could to smth like: y-y-n-n-n on a version like 1.2.0-stable-alpine
#[derive(Debug, PartialEq, Eq)]
enum VersionToken {
    Number,
    String,
}

fn infer_tokens(split_version: &[&str]) -> Vec<VersionToken> {
    split_version
        .iter()
        .map(|part| {
            if part.parse::<i32>().is_ok() {
                VersionToken::Number
            } else {
                VersionToken::String
            }
        })
        .collect()
}

fn find_compatible_version(
    curr_v_split: &[&str],
    curr_v_tokens: &[VersionToken],
    new_v: &str,
) -> Option<String> {
    let new_v_split = split_version(new_v);
    let new_v_tokens = infer_tokens(&new_v_split);

    // check if the version types are equal
    // [Number, Number, Number, String, String] ==
    // [Number, Number, Number, String, String]
    if curr_v_tokens == new_v_tokens {
        // we check if the merged version parts match their syntax
        if versions_compatible(curr_v_split, curr_v_tokens, &new_v_split)
            && version_is_newer(curr_v_split, curr_v_tokens, &new_v_split, &new_v_tokens)
        {
            return Some(new_v.to_string());
        }
    }
    None
}

fn versions_compatible(
    curr_v_split: &[&str],
    curr_v_types: &[VersionToken],
    new_v_split: &[&str],
) -> bool {
    // we merge both splitted versions so we get a tuple to check
    // ["1", "0", "2", "alpine", "otel"] and
    // ["1", "3", "0", "alpine", "otel"] yields
    // [("1", "1"), ("0", "3"), ("2", "0"), ("alpine", "alpine"), ("otel", "otel")]
    let mut version_pairs = curr_v_split.iter().zip(new_v_split).enumerate();

    // closure which checks if the string values are equal -> we have a matching version structure
    // we just use true for number types since we need to check for them later
    let match_structure =
        |(index, (curr_v_value, new_v_value)): (usize, (&_, &_))| match curr_v_types[index] {
            VersionToken::Number => true,
            VersionToken::String => curr_v_value == new_v_value,
        };

    // we check if the version pairs match their structure
    version_pairs.all(match_structure)
}

fn version_is_newer(
    curr_v_split: &[&str],
    curr_v_tokens: &[VersionToken],
    new_v_split: &[&str],
    new_v_tokens: &[VersionToken],
) -> bool {
    let version_pair = curr_v_split.iter().zip(new_v_split).enumerate();
    let token_pair = curr_v_tokens.iter().zip(new_v_tokens).enumerate();
    for (i, (curr_token, new_token)) in token_pair {}
    true
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
