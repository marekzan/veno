use anyhow::Result;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use serde::Deserialize;

use super::SinkSender;

#[derive(Deserialize, Clone, Debug)]
pub struct EmailSink {
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub to: String,
}

impl SinkSender for EmailSink {
    async fn send(&self, message: &str) {
        let email = match create_message(&self.username, &self.to, message) {
            Ok(email) => email,
            Err(e) => {
                eprintln!("Failed to create email: {:?}", e);
                return;
            }
        };

        let mailer = match create_mailer(&self.host, self.port, &self.username, &self.password) {
            Ok(mailer) => mailer,
            Err(e) => {
                eprintln!("Failed to create mailer: {:?}", e);
                return;
            }
        };

        if let Err(e) = mailer.send(&email) {
            eprintln!("Failed to close mailer: {:?}", e);
        }
    }
}

fn create_message(from: &str, to: &str, message: &str) -> Result<Message> {
    let email = Message::builder()
        .from(format!("VENO <{}>", from).parse()?)
        .to(to.parse()?)
        .subject("VENO: New version available!")
        .header(ContentType::TEXT_PLAIN)
        .body(message.to_string())
        .unwrap();

    Ok(email)
}

fn create_mailer(
    host: &str,
    port: Option<u16>,
    username: &str,
    password: &str,
) -> Result<SmtpTransport> {
    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer = SmtpTransport::starttls_relay(host)?
        .credentials(creds)
        .port(port.unwrap_or(587))
        .build();

    Ok(mailer)
}
