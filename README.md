# Veno

**VE**rsion **NO**tifier

If you need to regularly track and update your 3rd-party artifacts, Veno might be just what you need. With Veno, you can define artifacts and attach multiple notifiers to them. Veno will notify you whenever new versions of your artifacts are available.

## Description

So, what are **artifacts** and **notifiers**?

### Artifacts

An artifact is any (GitHub) repository that uses tags to assign versions to its releases. You can define an artifact like this:

```json
{
  "artifacts": [
    {
      "name": "Rust",
      "message_prefix": "There is a new version of:", // optional
      "current_version": "1.83.0",
      "source": {
        "type": "github",
        "repo": "rust-lang/rust"
      },
      "notifier": ["team_slack"]
    }
  ]
}
```

#### Source

A source specifies where Veno should look for new versions. Currently, we support the following sources:

- **GitHub**

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

Veno can/will be operated in three modes:

1. **CLI** (currently prioritized)
   The CLI mode executes a single operation each time it is run.

2. **Web Service** (when most features are implemented, the web service will be prioritized)
   The web service uses Axum to provide endpoints for triggering checks, retrieving version lists, and other useful actions in the future.

3. **Daemon (with scheduling)** (not yet implemented)

### Configuration

Both operation modes require a `config.json` file, which must be passed using the `--config` parameter. An example configuration file is included in the `example` folder [here](examples/config.json).
You can use Environment Variables to set values in the config file.

## Features & Roadmap

### Modes

| **Status** | **Features** |
| ---------- | ------------ |
| working    | cli          |
| planned    | web          |
| planned    | daemon       |

### Sources

| Status      | Features    |
| ----------- | ----------- |
| working     | GitHub      |
| in progress | Docker Hub  |
| in progress | Artifacthub |
| planned     | GitLab      |
| planned     | Bitbucket   |

### Sinks

| Status      | Features        |
| ----------- | --------------- |
| working     | Generic Webhook |
| working     | Email           |
| in progress | Slack           |
| in progress | Google Chat     |
| planned     | Microsoft Teams |
| planned     | Rocket Chat     |

### Features

| Status      | Features                                                                                                              |
| ----------- | --------------------------------------------------------------------------------------------------------------------- |
| working     | environment variables in config file                                                                                  |
| in progress | make notifiers have references to artifacts so that only one message is being sent to the sink for multiple artifacts |

### Ideas

| Status  | Features                                                                                              |
| ------- | ----------------------------------------------------------------------------------------------------- |
| planned | yaml config support                                                                                   |
| planned | toml config support                                                                                   |
| planned | Web endpoint to silence notifications for a specific artifact                                         |
| planned | update the current_version field in the configuration file automatically                              |
| planned | decide on which version update you want to be notified (major, minor, patch or other version regex)   |
| planned | include alpha, beta, and release candidate versions and allow users to specify which version to track |
| planned | scheduling for periodic checks (daemon mode)                                                          |
| planned | custom Chat Cards for Sinks (if supported)                                                            |
| planned | alpha, beta, and release candidate versions                                                           |

## Contributing

Contributions are welcome! Feel free to fork the repository and submit a pull request.

## License

This project is licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

You may choose either license.

---

Let me know if there’s anything else you’d like adjusted!
