use serde::Deserialize;
use serde_json::json;

use super::{webhook, SinkSender};

#[derive(Debug, Clone, Deserialize)]
pub struct GoogleChatSink {
    pub webhook: String,
}

impl SinkSender for GoogleChatSink {
    async fn send(&self, message: &str) {
        // here we will build a default google chat card
        let payload = json!({
            "text:": message.to_string(),
        });

        webhook::call(&self.webhook, &payload).await;
    }
}
