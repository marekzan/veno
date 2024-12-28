use serde::Deserialize;
use serde_json::json;

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
    async fn send(&self, message: &str) {
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
            .await;

        match response {
            Ok(response) if response.status().is_success() => (),
            Ok(response) => eprintln!("Error sending message: {:?}", response),
            Err(err) => eprintln!("Error sending message: {:?}", err),
        }
    }
}
