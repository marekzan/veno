use std::{error::Error, fmt::Display};

use lettre::{
    address::AddressError, message::header::ContentType,
    transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport,
};
use serde::Deserialize;

use super::SinkSender;

#[derive(Deserialize, Clone, Debug)]
pub struct EmailSink {
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub to: Vec<String>,
    pub subject: Option<String>,
}

impl SinkSender for EmailSink {
    async fn send(&self, message: &str) -> super::Result<()> {
        let mailer = create_mailer(&self.host, self.port, &self.username, &self.password)?;

        for to in &self.to {
            let email = create_message(&self.username, to, &self.subject, message)?;
            // NOTE it seems as if the ? operator only does a direct conversion of error.
            // even though we have a smtperror -> emailerror -> sinkerror conversion via the from trait,
            // we still need to explicitly call the from function.
            // maybe it's better to return the original error until we want to handle the errors and wrap them there for further
            // usage. this would reduce the From<> implementations
            mailer.send(&email).map_err(EmailError::from)?;
        }

        Ok(())
    }
}

fn create_message(
    from: &str,
    to: &str,
    subject: &Option<String>,
    message: &str,
) -> Result<Message> {
    let email = Message::builder()
        .from(format!("VENO <{}>", from).parse()?)
        .to(to.parse()?)
        .subject(subject.as_deref().unwrap_or("New versions available"))
        .header(ContentType::TEXT_PLAIN)
        .body(message.to_string())?;

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

type Result<T> = std::result::Result<T, EmailError>;

#[derive(Debug)]
pub enum EmailError {
    Address(AddressError),
    Build(lettre::error::Error),
    Send(lettre::transport::smtp::Error),
}

impl Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::Address(e) => write!(f, "Address parsing error: {e}"),
            EmailError::Build(e) => write!(f, "Message build error: {e}"),
            EmailError::Send(e) => write!(f, "Sending email error: {e}"),
        }
    }
}

impl Error for EmailError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EmailError::Address(e) => Some(e),
            EmailError::Build(e) => Some(e),
            EmailError::Send(e) => Some(e),
        }
    }
}

impl From<AddressError> for EmailError {
    fn from(value: AddressError) -> Self {
        EmailError::Address(value)
    }
}

impl From<lettre::error::Error> for EmailError {
    fn from(value: lettre::error::Error) -> Self {
        EmailError::Build(value)
    }
}

impl From<lettre::transport::smtp::Error> for EmailError {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        EmailError::Send(value)
    }
}
