use super::SinkNotifier;
use anyhow::Result;

pub struct GoogleChatNotifier {
    pub webhook: String,
}

impl SinkNotifier for GoogleChatNotifier {
    fn send(&self, message: &str) -> Result<()> {
        println!("Sending message to Google Chat: {}", message);
        println!("Webhook: {}", self.webhook);
        Ok(())
    }

    fn type_name(&self) -> &str {
        "Google Chat"
    }
}
