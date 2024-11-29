use serde::Deserialize;

pub mod checker;

#[derive(Deserialize, Debug, Clone)]
pub struct Artifact {
    pub name: String,
    pub message_prefix: Option<String>,
    pub source: String,
    pub current_version: String,
    pub notifier: Vec<String>,
}
