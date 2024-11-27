use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

use crate::config::{AppConfig, Application};

pub async fn check(config: &AppConfig) -> Result<String> {
    let client = reqwest::Client::new();
    let latest_version = get_latest_version(&config.applications[0], &client).await?;

    if latest_version > config.applications[0].current_version {
        Ok(format!(
            "A new version of Keycloak is available: {}",
            latest_version
        ))
    } else {
        Ok(String::from("You're using the latest version of Keycloak."))
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
