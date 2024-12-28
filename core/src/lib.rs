pub mod artifact;
pub mod config;
pub mod notifier;

use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .user_agent("veno-checker")
        .build()
        .expect("Could not create reqwest client")
});

pub fn pretty_json(body: &str) -> Result<String, serde_json::Error> {
    let json: serde_json::Value = serde_json::from_str(body)?;
    serde_json::to_string_pretty(&json)
}
