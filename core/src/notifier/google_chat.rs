use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
use std::{future::Future, pin::Pin};

use crate::CLIENT;

use super::SinkSender;

#[derive(Debug, Clone, Deserialize)]
pub struct GoogleChatSink {
    pub webhook: String,
}

// #[derive(Serialize)]
// struct Payload {
//     text: String,
// }

impl SinkSender for GoogleChatSink {
    fn send<'a>(
        &'a self,
        message: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + Sync + 'a>> {
        Box::pin(async move {
            // let payload = Payload {
            //     text: message.to_string(),
            // };

            // let payload = serde_json::to_string(&payload)?;
            let payload = json!({
                "text:": message.to_string(),
            });

            let response = CLIENT
                .post(&self.webhook)
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await?;
            Ok(response.text().await?)
        })
    }
}
