# Neveno

**NE**w **VE**rsion **NO**tifier

If you need to regularly track and update your 3rd-party artifacts, Neveno might be just what you need. With Neveno, you can define artifacts and attach multiple notifiers to them. Neveno will notify you whenever new versions of your artifacts are available.

## Description

So, what are **artifacts** and **notifiers**?

### Artifacts

An artifact is any (GitHub) repository that uses tags to assign versions to its releases. You can define an artifact like this:

```json
{
  "artifacts": [
    {
      "source": "https://api.github.com/repos/${username}/${repo_name}/releases/latest",
      "current_version": "23.0.6",
      "notifier": ["team_slack"]
    }
  ]
}
```

### Notifiers

A notifier specifies where notifications should be sent. Currently, we support the following sinks:

- **Email**
- **Google Chat**
- **Slack**

Additional sinks, such as Microsoft Teams and Rocket Chat, are planned.

Here’s an example of a notifier configuration:

```json
{
  "notifiers": [
    {
      "name": "team_slack",
      "sink": {
        "type": "slack",
        "webhook": "..."
      }
    }
  ]
}
```

#### Supported Sink Configurations

- **Slack**:

  ```json
  {
    "sink": {
      "type": "slack",
      "webhook": "..."
    }
  }
  ```

- **Google Chat**:

  ```json
  {
    "sink": {
      "type": "google_chat",
      "webhook": "..."
    }
  }
  ```

- **Email**:
  ```json
  {
    "sink": {
      "type": "email",
      "host": "smtp...",
      "port": 587,
      "username": "...",
      "password": "..."
    }
  }
  ```

## Usage

Neveno can be operated in two modes:

1. **CLI**  
   The CLI mode executes a single operation each time it is run.

2. **Web Service**  
   The web service uses Axum to provide endpoints for triggering checks, retrieving version lists, and other useful actions in the future.

### Configuration

Both operation modes require a `config.json` file, which must be passed using the `--config` parameter. An example configuration file is included in the `example` folder.

Here's a sample `config.json`:

```json
{
  "artifacts": [
    {
      "source": "https://api.github.com/repos/${username}/${repo_name}/releases/latest",
      "current_version": "23.0.6",
      "notifier": ["team_slack", "private_email"]
    }
  ],
  "notifiers": [
    {
      "name": "team_slack",
      "sink": {
        "type": "slack",
        "webhook": "..."
      }
    },
    {
      "name": "private_email",
      "sink": {
        "type": "email",
        "host": "smtp.gmail.com",
        "port": 587,
        "username": "username",
        "password": "password"
      }
    },
    {
      "name": "team_google_chat",
      "sink": {
        "type": "google_chat",
        "webhook": "..."
      }
    }
  ]
}
```

## Roadmap

### What’s Working:

- **Sources**: GitHub
- **Sinks**: Slack, Google Chat

### In Progress:

- Email as a sink

### Planned Features:

- More sources: GitLab, Bitbucket, etc.
- Additional sinks: Microsoft Teams, Rocket Chat, etc.
- Scheduling for periodic checks
- update the current_version field in the configuration file automatically

## Contributing

Contributions are welcome! Feel free to fork the repository and submit a pull request.

## License

This project is licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

You may choose either license.

---

Let me know if there’s anything else you’d like adjusted!
