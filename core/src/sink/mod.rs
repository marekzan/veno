use anyhow::Result;
use email::EmailNotifier;
use google_chat::GoogleChatNotifier;
use serde::Deserialize;
use slack::SlackNotifier;
use std::{future::Future, pin::Pin};

pub mod email;
pub mod google_chat;
pub mod slack;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")] // Use tag-based enum for sink type
pub enum Sink {
    #[serde(rename = "slack")]
    Slack { webhook: String },
    #[serde(rename = "email")]
    Email {
        host: String,
        port: u16,
        username: String,
        password: String,
    },
    #[serde(rename = "google_chat")]
    GoogleChat { webhook: String },
}

impl Sink {
    pub fn to_notifier(&self) -> Box<dyn SinkNotifier<Output = String>> {
        match self {
            Sink::Slack { webhook } => Box::new(SlackNotifier {
                webhook: webhook.clone(),
            }),
            Sink::Email {
                host,
                port,
                username,
                password,
            } => Box::new(EmailNotifier {
                host: host.clone(),
                port: *port,
                username: username.clone(),
                password: password.clone(),
            }),
            Sink::GoogleChat { webhook } => Box::new(GoogleChatNotifier {
                webhook: webhook.clone(),
            }),
        }
    }
}

pub trait SinkNotifier: Send + Sync {
    type Output;
    fn send<'a>(
        &'a self,
        message: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + 'a>>;
}
