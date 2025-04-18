pub mod email;
pub mod google_chat;
pub mod slack;
pub mod webhook;

use std::fmt::Display;

use email::{EmailError, EmailSink};
use google_chat::GoogleChatSink;
use serde::Deserialize;
use slack::SlackSink;
use webhook::{WebhookError, WebhookSink};

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

impl Sink {
  pub async fn send(&self, notification: &str) -> Result<()> {
    match self {
      Sink::Slack(sink) => sink.send(notification).await,
      Sink::Email(sink) => sink.send(notification).await,
      Sink::GoogleChat(sink) => sink.send(notification).await,
      Sink::Webhook(sink) => sink.send(notification).await,
    }
  }
}

trait SinkSender: Send {
  async fn send(&self, message: &str) -> Result<()>;
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

pub type Result<T> = std::result::Result<T, SinkError>;

#[derive(Debug)]
pub enum SinkError {
  Email(EmailError),
  Webhook(WebhookError),
}

impl Display for SinkError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SinkError::Email(e) => write!(
        f,
        "There was an errror sending the notifications via email: {e}"
      ),
      SinkError::Webhook(e) => write!(
        f,
        "There was an error sending the notifications via webhook: {e}"
      ),
    }
  }
}

impl From<EmailError> for SinkError {
  fn from(value: EmailError) -> Self {
    SinkError::Email(value)
  }
}

impl From<WebhookError> for SinkError {
  fn from(value: WebhookError) -> Self {
    SinkError::Webhook(value)
  }
}
