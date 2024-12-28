use serde::Deserialize;

use super::SinkSender;

#[derive(Deserialize, Clone, Debug)]
pub struct EmailSink {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl SinkSender for EmailSink {
    async fn send(&self, message: &str) {
        println!("Email sent: {}", message);
    }
}
