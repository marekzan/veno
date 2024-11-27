use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

use crate::config::Application;

pub async fn check(app: &Application) -> Result<Option<String>> {
    let client = reqwest::Client::new();
    let latest_version = get_latest_version(app, &client).await?;

    if latest_version > app.current_version {
        Ok(Some(latest_version))
    } else {
        Ok(None)
    }
}

async fn get_latest_version(app: &Application, client: &Client) -> Result<String> {
    let response = client
        .get(&app.source)
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
