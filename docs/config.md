```json
{
  "artifacts": [
    {
      "id": "rust",
      "name": "Rust",
      "current_version": "1.82.0",
      "source": {
        "type": "github",
        "identifier": "rust-lang/rust"
      },
    }
    {
      "id": "kubernetes",
      "name": "Kubernetes",
      "current_version": "1.0.0",
      "source": {
        "type": "github",
        "identifier": "kubernetes/kubernetes"
      },
    }
  ],
  "notifiers": [
    {
      "name": "private_email",
      "sink": {
        "type": "email",
        "host": "smtp.gmail.com",
        "port": 587,
        "username": "${EMAIL_USERNAME}",
        "password": "${EMAIL_PASSWORD}",
        "to": ["my_email@account.com"],
        "subject": "New version of"
      }
      "artifact_ids": ["rust"]
    },
    {
      "name": "team_google_chat",
      "sink": {
        "type": "google_chat",
        "webhook": "https://google_chat.com/webhook"
      }
      "artifact_ids": ["rust", "kubernetes"]
    }
  ]
}
```
