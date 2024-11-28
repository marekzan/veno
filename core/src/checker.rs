use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

use crate::config::Artifact;

pub async fn check(artifact: &Artifact) -> Result<Option<String>> {
    let client = reqwest::Client::new();
    let latest_version = get_latest_version(artifact, &client).await?;

    if latest_version > artifact.current_version {
        Ok(Some(latest_version))
    } else {
        Ok(None)
    }
}

async fn get_latest_version(artifact: &Artifact, client: &Client) -> Result<String> {
    let response = client
        .get(&artifact.source)
        .header("User-Agent", "neveno-checker")
        .send()
        .await?;

    let release: Release = response.json().await?;
    let latest_version = release.tag_name.trim_start_matches('v');
    Ok(latest_version.to_string())
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}
