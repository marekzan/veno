use std::{error::Error, fmt::Display};

use crate::CLIENT;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};

use super::SinkSender;

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookSink {
  pub webhook: String,
}

impl SinkSender for WebhookSink {
  async fn send(&self, message: &str) -> super::Result<()> {
    // TODO here we will build a default google chat card
    let payload = json!({
        "text:": message.to_string(),
    });

    call(&self.webhook, &payload).await?;

    Ok(())
  }
}

// TODO this needs to bubble so we can return an error to the user
pub async fn call(webhook: &str, payload: &Value) -> Result<()> {
  match CLIENT
    .post(webhook)
    .header("Content-Type", "application/json")
    .json(payload)
    .send()
    .await
  {
    Ok(response) if response.status().is_success() => Ok(()),
    Ok(response) => {
      let status_code = response.status();
      let body = response.json().await.ok();
      Err(WebhookError::UnsuccesfulResponse { status_code, body })
    }
    Err(err) => Err(WebhookError::FailedSend(err)),
  }
}

type Result<T> = std::result::Result<T, WebhookError>;

#[derive(Debug)]
pub enum WebhookError {
  UnsuccesfulResponse {
    status_code: StatusCode,
    body: Option<String>,
  },
  FailedSend(reqwest::Error),
}

impl Error for WebhookError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      WebhookError::UnsuccesfulResponse { .. } => None,
      WebhookError::FailedSend(e) => Some(e),
    }
  }
}

impl Display for WebhookError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      WebhookError::UnsuccesfulResponse { status_code, body } => {
        write!(f, "Unsuccsessful response from webhook: {status_code}")?;
        if let Some(text) = body {
          if !text.is_empty() {
            write!(f, "{text}")?
          }
        }
        Ok(())
      }
      WebhookError::FailedSend(err) => {
        write!(f, "An error occured while sending the payload {}", err)
      }
    }
  }
}

impl From<reqwest::Error> for WebhookError {
  fn from(value: reqwest::Error) -> Self {
    WebhookError::FailedSend(value)
  }
}
