use anyhow::Result;
use serde_json::json;
use std::{future::Future, pin::Pin};

use crate::CLIENT;

use super::SinkNotifier;

pub struct GoogleChatNotifier {
    pub webhook: String,
}

// #[derive(Serialize)]
// struct Payload {
//     text: String,
// }

impl SinkNotifier for GoogleChatNotifier {
    type Output = String;
    fn send<'a>(
        &'a self,
        message: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + 'a>> {
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
