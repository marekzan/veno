# Example Config

```{
  "artifacts": [],
  "notifiers": []
}
```

## Artifacts

### GitHub

For the repo value you just take the part of the url with the `username/reponame`

```
{
  "name": "Rust",
  "message_prefix": "There is a new version of:", // optional
  "source": {
    "type": "github",
    "repo": "rust-lang/rust"
  },
  "current_version": "",
  "notifier": ["team_slack", "private_email"]
},
```

### Artifacthub

For the package value you just take the part of the url with the `kind/organization/packagename`

```
{
  "name": "Keycloak Helm Chart",
  "message_prefix": "There is a new version of:", // optional
  "current_version": "24.3.1",
  "source": {
    "type": "artifacthub",
    "package": "helm/bitnami/keycloak"
  },
  "notifier": ["generic_webhook", "team_google_chat"]
},
```

### Dockerhub

For the repo you just take the part of the url with the `username/imagename`.
Official images have a special syntax `_/nginx` for example. For these you just use the image name `nginx`.

```
{
  "name": "nginx",
  "message_prefix": "There is a new version of:", // optional
  "current_version": "1.25.2-alpine3.20-perl",
  "source": {
    "type": "dockerhub",
    "repo": "nginx"
  },
  "notifier": ["private_email"]
}
```

## Notifiers

### Email

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
  }
}
```

### Webhook

The webhook value is just the `url` to the webhook. `Slack` and `Google Chat` use the webhook logic under the hood but they will offer a default chat card in the future.

```
{
  "name": "generic_webhook",
  "sink": {
    "type": "webhook",
    "webhook": ".."
  }
}
```

### Slack

```
{
  "name": "team_slack",
  "sink": {
    "type": "slack",
    "webhook": ".."
  }
}
```

### Google Chat

```
{
  "name": "team_google_chat",
  "sink": {
    "type": "google_chat",
    "webhook": "..."
  }
}
```
