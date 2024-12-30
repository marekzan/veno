use crate::CLIENT;
use serde_json::Value;

pub async fn call(webhook: &str, payload: &Value) {
    match CLIENT
        .post(webhook)
        .header("Content-Type", "application/json")
        .json(payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => (),
        Ok(response) => eprintln!("Error sending message: {:?}", response),
        Err(err) => eprintln!("Error sending message: {:?}", err),
    }
}
