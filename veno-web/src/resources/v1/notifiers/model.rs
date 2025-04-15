use serde::Serialize;
use veno_core::notifier::{Notifier, Sink};

#[derive(Serialize, Debug, Clone)]
pub struct NotifierResponse {
    pub name: String,
    pub sink: SinkDto,
    pub artifact_ids: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")] // Use tag-based enum for sink type
pub enum SinkDto {
    #[serde(rename = "slack")]
    Slack(SlackSink),
    #[serde(rename = "email")]
    Email(EmailSink),
    #[serde(rename = "google_chat")]
    GoogleChat(GoogleChatSink),
    #[serde(rename = "webhook")]
    Webhook(WebhookSink),
}

#[derive(Debug, Clone, Serialize)]
pub struct SlackSink {
    pub webhook: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct EmailSink {
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub to: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GoogleChatSink {
    pub webhook: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookSink {
    pub webhook: String,
}

impl From<Notifier> for NotifierResponse {
    fn from(value: Notifier) -> Self {
        Self {
            name: value.name,
            sink: value.sink.into(),
            artifact_ids: value.artifact_ids,
        }
    }
}

impl From<Sink> for SinkDto {
    fn from(value: Sink) -> Self {
        match value {
            Sink::Slack(slack) => SinkDto::Slack(SlackSink {
                webhook: slack.webhook,
            }),
            Sink::Email(email) => SinkDto::Email(EmailSink {
                host: email.host,
                port: email.port,
                username: String::from("[REDACTED]"),
                password: String::from("[REDACTED]"),
                to: email.to,
            }),
            Sink::GoogleChat(google) => SinkDto::GoogleChat(GoogleChatSink {
                webhook: google.webhook,
            }),
            Sink::Webhook(webhook) => SinkDto::Webhook(WebhookSink {
                webhook: webhook.webhook,
            }),
        }
    }
}
