use super::SinkNotifier;
use anyhow::Result;

pub struct SlackSink {
    pub webhook: String,
}

impl SinkNotifier for SlackSink {
    fn send(&self, message: &str) -> Result<()> {
        // Send a message to Google Chat
        println!("Sending message to Google Chat: {}", message);
        Ok(())
    }
}
