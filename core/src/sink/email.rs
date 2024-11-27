use super::SinkNotifier;
use anyhow::Result;

#[derive(Debug)]
pub struct EmailNotifier {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl SinkNotifier for EmailNotifier {
    fn send(&self, message: &str) -> Result<()> {
        println!("Sending message as email: {}", message);
        println!("with config: {:?}", self);
        Ok(())
    }

    fn type_name(&self) -> &str {
        "Email"
    }
}
