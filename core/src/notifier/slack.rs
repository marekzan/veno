use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
use std::{future::Future, pin::Pin};

use crate::CLIENT;

use super::SinkSender;

#[derive(Debug, Clone, Deserialize)]
pub struct SlackSink {
    pub webhook: String,
}

// #[derive(Serialize)]
// struct Payload {
//     text: String,
// }

impl SinkSender for SlackSink {
    fn send<'a>(
        &'a self,
        message: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send + Sync + 'a>> {
        Box::pin(async move {
            // let payload = Payload {
            //     text: message.to_string(),
            // };

            let payload = json!({
                "text": message.to_string(),
            });

            println!("Sending payload: {}", payload);

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
