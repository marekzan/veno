use serde::Deserialize;
use serde_json::json;

use super::{
  webhook::{self, WebhookError},
  SinkSender,
};

#[derive(Debug, Clone, Deserialize)]
pub struct SlackSink {
  pub webhook: String,
}

impl SinkSender for SlackSink {
  async fn send(&self, notification: &str) -> super::Result<()> {
    // NOTE here we will build a default slack message
    let payload = json!({
        "text": notification.to_string(),
    });

    webhook::call(&self.webhook, &payload)
      .await
      .map_err(WebhookError::from)?;

    Ok(())
  }
}
