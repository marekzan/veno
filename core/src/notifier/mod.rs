pub mod email;
pub mod google_chat;
pub mod slack;

use anyhow::Result;
use email::EmailSink;
use google_chat::GoogleChatSink;
use serde::Deserialize;
use slack::SlackSink;
use std::{future::Future, pin::Pin};

static DEFAULT_MESSAGE_PREFIX: &str = "New version available for";

#[derive(Deserialize, Debug, Clone)]
pub struct Notifier {
    pub name: String,
    pub sink: Sink,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")] // Use tag-based enum for sink type
pub enum Sink {
    #[serde(rename = "slack")]
    Slack(SlackSink),
    #[serde(rename = "email")]
    Email(EmailSink),
    #[serde(rename = "google_chat")]
    GoogleChat(GoogleChatSink),
}

impl Sink {
    pub fn unwrap(&self) -> Box<dyn SinkSender> {
        match self {
            Sink::Slack(sender) => Box::new(sender.clone()),
            Sink::Email(sender) => Box::new(sender.clone()),
            Sink::GoogleChat(sender) => Box::new(sender.clone()),
        }
    }
}

// Send + Sync is required for Axum Handler
pub trait SinkSender: Send + Sync {
    fn send<'a>(
        &'a self,
        message: &'a str,
        // This is needed to allow async methods in the trait
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + Sync + 'a>>;
}

pub fn create_default_message(artifact_name: &str, latest_version: &str) -> String {
    format!(
        "{} {}: {}",
        DEFAULT_MESSAGE_PREFIX, artifact_name, latest_version
    )
}

pub fn create_custom_message(prefix: &str, artifact_name: &str, latest_version: &str) -> String {
    format!("{} {}: {}", prefix, artifact_name, latest_version)
}
