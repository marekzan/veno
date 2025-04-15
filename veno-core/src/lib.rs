pub mod app;
pub mod artifact;
pub mod config;
pub mod notifier;

use std::time::Duration;

use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder, Response};

static CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .user_agent("veno")
        .build()
        .expect("Could not create reqwest client")
});

async fn get(url: &str) -> Result<Response, reqwest::Error> {
    CLIENT
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await
}
