# Notifiers

## Email

```
{
  "name": "private_email",
  "sink": {
    "type": "email",
    "host": "smtp.gmail.com",
    "port": 587, // optional - default: 587
    "username": "username",
    "password": "${EMAIL_PASSWORD}",
    "to": "recipient@somemail.com",
    "subject": "New version available" // optional - default: "New version available"
  },
  "artifact_ids": ["rust"]
}
```

## Webhook

The webhook value is just the `url` to the webhook. `Slack` and `Google Chat` use the webhook logic under the hood but they will offer a default chat card in the future.

```
{
  "name": "generic_webhook",
  "sink": {
    "type": "webhook",
    "webhook": ".."
  },
  "artifact_ids": ["rust", "nginx_dockerhub"]
}
```

## Slack

```
{
  "name": "team_slack",
  "sink": {
    "type": "slack",
    "webhook": ".."
  },
  "artifact_ids": ["keycloak_helm_chart", "nginx_dockerhub"]
}
```

## Google Chat

```
{
  "name": "team_google_chat",
  "sink": {
    "type": "google_chat",
    "webhook": "..."
  },
  "artifact_ids": ["rust", "keycloak_helm_chart", "nginx_dockerhub"]
}
```
