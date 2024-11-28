use anyhow::Result;
use std::{future::Future, pin::Pin};

use super::SinkNotifier;

#[derive(Debug)]
pub struct EmailNotifier {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl SinkNotifier for EmailNotifier {
    type Output = String;

    fn send<'a>(
        &'a self,
        message: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + 'a>> {
        Box::pin(async move { Ok(format!("Email sent: {}", message)) })
    }
}
