use crate::CLIENT;
use serde::Deserialize;
use serde_json::{json, Value};

use super::SinkSender;

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookSink {
    pub webhook: String,
}

impl SinkSender for WebhookSink {
    async fn send(&self, message: &str) {
        // here we will build a default google chat card
        let payload = json!({
            "text:": message.to_string(),
        });

        call(&self.webhook, &payload).await;
    }
}

// TODO this needs to bubble so we can return an error to the user
pub async fn call(webhook: &str, payload: &Value) {
    match CLIENT
        .post(webhook)
        .header("Content-Type", "application/json")
        .json(payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => (),
        Ok(response) => eprintln!("Error sending message: {:?}", response),
        Err(err) => eprintln!("Error sending message: {:?}", err),
    }
}
