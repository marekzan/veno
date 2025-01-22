pub mod email;
pub mod google_chat;
pub mod slack;
pub mod webhook;

use email::EmailSink;
use google_chat::GoogleChatSink;
use serde::Deserialize;
use slack::SlackSink;
use webhook::WebhookSink;

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
    #[serde(rename = "webhook")]
    Webhook(WebhookSink),
}

impl Sink {
    pub async fn send(&self, message: &str) {
        match self {
            Sink::Slack(sender) => sender.send(message).await,
            Sink::Email(sender) => sender.send(message).await,
            Sink::GoogleChat(sender) => sender.send(message).await,
            Sink::Webhook(sender) => sender.send(message).await,
        }
    }
}

trait SinkSender: Send {
    async fn send(&self, message: &str);
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
