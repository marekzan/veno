# **Veno**

**VE**rsion **NO**tifier – Stay updated with the latest versions of your essential software!

[![GitHub Release](https://img.shields.io/github/v/release/marekzan/veno)](https://github.com/marekzan/veno/releases)
[![License](https://img.shields.io/badge/license-Apache%202.0%20or%20MIT-blue.svg)](./LICENSE)

## **Overview**

Veno is a lightweight tool that tracks new versions of software packages and notifies you via email, webhooks, or chat integrations. Define **artifacts** in a simple config file and attach multiple **notifiers**—Veno will do the rest!

## **How It Works**

1. Define software packages (**artifacts**) in `config.json`.
2. Choose notification methods (**notifiers**) like email, Slack, or webhooks.
3. Run Veno via CLI or Web Service to get version updates.

> [!Note]
> See what is currently supported [here](docs/whats_working.md).

## How to Run

**Build it**

`cargo build --release`

**Run via CLI:**

You find the binary in `target/release/veno-cli`.

```sh
veno-cli --config path/to/config.json
```

> [!Tip]
> We recommend to use the binary as a cron job (or any other scheduling task manager) in your cluster, so you can get the updates in the right time.

## **Configuration**

All modes require a `config.json` file. Example configuration can be found [here](docs/config.md).  
You can also set values using environment variables via the `${}` syntax.

## **Features & Roadmap**

For upcoming features, check the [Issues](https://github.com/marekzan/veno/issues?q=is%3Aissue%20state%3Aopen%20label%3Afeature%2Crefactor) or the [Milestones](https://github.com/marekzan/veno/milestones)

## **Contributing**

Contributions are welcome! Fork the repository and submit a pull request.

## **License**

Veno is dual-licensed under:

- **Apache License, Version 2.0** ([LICENSE-APACHE](./LICENSE-APACHE))
- **MIT License** ([LICENSE-MIT](./LICENSE-MIT))
