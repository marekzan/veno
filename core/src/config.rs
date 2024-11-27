use anyhow::{Context, Result};
use config::{Config, File, FileFormat};
use serde::Deserialize;

use crate::sink::{email::EmailSink, google_chat::GoogleChatSink, slack::SlackSink, SinkNotifier};

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub applications: Vec<Application>,
    pub notifiers: Vec<Notifier>,
}

#[derive(Deserialize, Debug)]
pub struct Application {
    pub source: String,
    pub current_version: String,
    pub notifier: Vec<String>, // List of notifier names
}

#[derive(Deserialize, Debug)]
pub struct Notifier {
    pub name: String,
    pub sink: Sink,
}

#[derive(Deserialize, Debug)]
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
    fn to_notifier(&self) -> Box<dyn SinkNotifier> {
        match self {
            Sink::Slack { webhook } => Box::new(SlackSink {
                webhook: webhook.clone(),
            }),
            Sink::Email {
                host,
                port,
                username,
                password,
            } => Box::new(EmailSink {
                host: host.clone(),
                port: *port,
                username: username.clone(),
                password: password.clone(),
            }),
            Sink::GoogleChat { webhook } => Box::new(GoogleChatSink {
                webhook: webhook.clone(),
            }),
        }
    }
}

impl AppConfig {
    pub fn load(file_path: &str) -> Result<Self> {
        let builder = Config::builder().add_source(File::new(file_path, FileFormat::Json));

        let config = builder.build()?;
        let app_config = config
            .try_deserialize()
            .context("Could not deserialize config to app_config")?;
        Ok(app_config)
    }
}