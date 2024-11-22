use reqwest::Error;
use serde::Deserialize;

use crate::config::AppConfig;

pub async fn check(config: &AppConfig) -> Result<String, Error> {
    // Make an HTTP GET request
    let client = reqwest::Client::new();
    let response = client
        .get(&config.source)
        .header("User-Agent", "rust-checker")
        .send()
        .await?;

    // Parse the JSON response
    // let release: Release = response.json().await?;
    let release: Release = response.json().await?;

    // Extract the latest version
    let latest_version = release.tag_name.trim_start_matches('v');

    println!("Current version: {}", config.current_version);
    println!("Latest version: {}", latest_version);

    // Compare versions
    if latest_version > config.current_version.as_str() {
        Ok(format!(
            "A new version of Keycloak is available: {}",
            latest_version
        ))
    } else {
        Ok(String::from("You're using the latest version of Keycloak."))
    }
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}
