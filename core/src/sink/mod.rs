use anyhow::Result;
use email::EmailSink;
use google_chat::GoogleChatSink;
use serde::Deserialize;
use slack::SlackSink;
use std::{future::Future, pin::Pin};

pub mod email;
pub mod google_chat;
pub mod slack;

static DEFAULT_MESSAGE_PREFIX: &str = "New version available for";

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
    pub fn get(&self) -> Box<dyn SinkSender<Output = String>> {
        match self {
            Sink::Slack(sender) => Box::new(sender.clone()),
            Sink::Email(sender) => Box::new(sender.clone()),
            Sink::GoogleChat(sender) => Box::new(sender.clone()),
        }
    }
}

pub trait SinkSender: Send + Sync {
    type Output;
    fn send<'a>(
        &'a self,
        message: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + 'a>>;
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
