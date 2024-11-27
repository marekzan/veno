use super::SinkNotifier;
use anyhow::Result;

pub struct EmailSink {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl SinkNotifier for EmailSink {
    fn send(&self, message: &str) -> Result<()> {
        // Send a message to Google Chat
        println!("Sending message to Google Chat: {}", message);
        Ok(())
    }
}
