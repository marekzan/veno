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
    pub artifact_ids: Vec<String>,
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

// TODO this could be either fire and forget and log errors or get a result and make a response but then use a join_all
impl Sink {
    pub async fn send(&self, notification: &str) {
        match self {
            Sink::Slack(sink) => sink.send(notification).await,
            Sink::Email(sink) => sink.send(notification).await,
            Sink::GoogleChat(sink) => sink.send(notification).await,
            Sink::Webhook(sink) => sink.send(notification).await,
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
