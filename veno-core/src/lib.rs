pub mod app;
pub mod artifact;
pub mod config;
pub mod notifier;

use std::{sync::LazyLock, time::Duration};

use anyhow::Result;
use reqwest::{Client, ClientBuilder, Response};

// TODO this panics during runtime! it would be better to exit on startup. Maybe use OnceLock?
// or initialize the client in main?
static CLIENT: LazyLock<Client> = LazyLock::new(|| {
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
