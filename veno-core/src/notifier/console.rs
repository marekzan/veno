use serde::Deserialize;
use serde_json::json;

use super::SinkSender;

#[derive(Debug, Clone, Deserialize)]
pub struct ConsoleSink {}

impl SinkSender for ConsoleSink {
    async fn send(&self, message: &str) {
        println!("{message}")
    }
}
