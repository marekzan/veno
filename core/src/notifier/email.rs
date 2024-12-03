use anyhow::Result;
use serde::Deserialize;
use std::{future::Future, pin::Pin};

use super::SinkSender;

#[derive(Deserialize, Clone, Debug)]
pub struct EmailSink {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl SinkSender for EmailSink {
    type Output = String;

    fn send<'a>(
        &'a self,
        message: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + 'a>> {
        Box::pin(async move { Ok(format!("Email sent: {}", message)) })
    }
}
