use std::time::Duration;

use anyhow::Result;
use serde::Deserialize;

use crate::{source::Artifact, CLIENT};

pub async fn check(artifact: &Artifact) -> Result<Option<String>> {
    let latest_version = get_latest_version(artifact).await?;

    if latest_version > artifact.current_version {
        Ok(Some(latest_version))
    } else {
        Ok(None)
    }
}

async fn get_latest_version(artifact: &Artifact) -> Result<String> {
    let response = CLIENT
        .get(&artifact.source)
        .header("User-Agent", "neveno")
        .timeout(Duration::from_secs(10))
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
