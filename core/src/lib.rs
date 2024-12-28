pub mod artifact;
pub mod config;
pub mod notifier;

use std::time::Duration;

use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder, Response};

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .user_agent("veno-checker")
        .build()
        .expect("Could not create reqwest client")
});

pub async fn get(url: &str) -> Result<Response, reqwest::Error> {
    CLIENT
        .get(url)
        .header("User-Agent", "veno")
        .timeout(Duration::from_secs(10))
        .send()
        .await
}

pub fn pretty_json(body: &str) -> Result<String, serde_json::Error> {
    let json: serde_json::Value = serde_json::from_str(body)?;
    serde_json::to_string_pretty(&json)
}
