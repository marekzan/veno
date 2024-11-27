use super::SinkNotifier;
use anyhow::Result;

pub struct SlackNotifier {
    pub webhook: String,
}

impl SinkNotifier for SlackNotifier {
    fn send(&self, message: &str) -> Result<()> {
        println!("Sending message to Slack: {}", message);
        println!("Webhook: {}", self.webhook);
        Ok(())
    }

    fn type_name(&self) -> &str {
        "Slack"
    }
}
