use anyhow::Result;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use serde::Deserialize;
use tracing::error;

use super::SinkSender;

#[derive(Deserialize, Clone, Debug)]
pub struct EmailSink {
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub to: Vec<String>,
}

impl SinkSender for EmailSink {
    async fn send(&self, message: &str) {
        let mailer = match create_mailer(&self.host, self.port, &self.username, &self.password) {
            Ok(mailer) => mailer,
            Err(e) => {
                error!("Failed to create mailer: {:?}", e);
                return;
            }
        };

        self.to.iter().for_each(|to| {
            let email = match create_message(&self.username, to, message) {
                Ok(email) => email,
                Err(e) => {
                    error!("Failed to create email: {:?}", e);
                    return;
                }
            };

            if let Err(e) = mailer.send(&email) {
                error!("Failed to close mailer: {:?}", e);
            }
        });
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
