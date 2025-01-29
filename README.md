# Veno

**VE**rsion **NO**tifier

## Description

What if you could stay effortlessly informed about new versions of your essential artifacts (repositories, Helm charts, and more)?  
Veno is a simple yet powerful tool that makes it easy to track updates using a single configuration file.

With Veno, you can define **artifacts** and attach multiple **notifiers** to them.  
Veno will notify you on your schedule whenever new versions of your artifacts are available.

So, what are **artifacts** and **notifiers**?

### Artifacts

An Artifact is a definition of a software package or service that you want to track. See the example [configuration file](examples/config.json) to get an understanding of the structure of an artifact.

#### Source

A source specifies where Veno should look for new versions. Currently, we support the following sources:

- **GitHub**
- **Dockerhub**
- **ArtifactHub**

### Notifiers

A notifier specifies where notifications should be sent. Currently, we support the following sinks:

- **Email**
- **Google Chat**
- **Slack**
- **Webhook**

Additional sinks, such as Microsoft Teams and Rocket Chat, are planned.

See the example [configuration file](examples/config.json) to get an understanding of the structure of a notifier.

## Usage

Veno will be able to be operated in three modes:

1. **CLI** (currently prioritized)
   The CLI mode executes a single operation each time it is run.

2. **Web Service** (when most features are implemented, the web service will be prioritized)
   The web service uses Axum to provide endpoints for triggering checks, retrieving version lists, and other useful actions in the future.

3. **Daemon (with scheduling)** (not yet implemented)

### Configuration

All operation modes require a `config.json` file, which must be passed using the `--config` parameter. An example configuration file is included in the `example` folder [configuration file](examples/config.json).
You can use Environment Variables to set values in the config file.

## Features & Roadmap

You can find the roadmap [here](ROADMAP.md).

## Contributing

Contributions are welcome! Feel free to fork the repository and submit a pull request.

## License

This project is licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

You may choose either license.

---

Let me know if there’s anything else you’d like adjusted!
