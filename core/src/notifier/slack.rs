use serde::Deserialize;
use serde_json::json;

use super::{webhook, SinkSender};

#[derive(Debug, Clone, Deserialize)]
pub struct SlackSink {
    pub webhook: String,
}

impl SinkSender for SlackSink {
    async fn send(&self, message: &str) {
        let payload = json!({
            "text": message.to_string(),
        });

        webhook::call(&self.webhook, &payload).await;
    }
}
