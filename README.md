# **Veno**

📢 **VE**rsion **NO**tifier – Stay updated with the latest versions of your essential software!

[![GitHub Release](https://img.shields.io/github/v/release/marekzan/veno)](https://github.com/marekzan/veno/releases)
[![License](https://img.shields.io/badge/license-Apache%202.0%20or%20MIT-blue.svg)](./LICENSE)

## 🚀 **Overview**

Veno is a lightweight tool that tracks new versions of software packages and notifies you via email, webhooks, or chat integrations. Define **artifacts** in a simple config file and attach multiple **notifiers**—Veno will do the rest!

## 🔍 **How It Works**

1. Define software packages (**artifacts**) in `config.json`.
2. Choose notification methods (**notifiers**) like email, Slack, or webhooks.
3. Run Veno via CLI or Web Service to get version updates.

## 📦 **Artifacts**

Artifacts represent the software packages you want to track. Veno currently supports:
| Source | Status |
|------------|---------|
| **GitHub** | ✅ Supported |
| **Dockerhub** | ✅ Supported |
| **ArtifactHub** | ✅ Supported |
| **GitLab** | 🔜 Planned |
| **Bitbucket** | 🔜 Planned |

## 🔔 **Notifiers**

Choose how to receive notifications:
| Notifier | Status |
|------------|--------|
| **Email** | ✅ Supported |
| **Webhook** | ✅ Supported |
| **Google Chat** | 🚧 Basic support |
| **Slack** | 🚧 Basic support |
| **Microsoft Teams** | 🔜 Planned |

## ⚙️ **Usage**

Veno supports three operation modes:
| Mode | Status | Description |
|------|--------|-------------|
| **CLI** | ✅ Working | Run checks on demand |
| **Web Service** | 🚧 In Progress | REST API for version tracking |
| **Daemon** | 🔜 Planned | Automated background scheduling |

**Build it**

`cargo build --release`

**Run via CLI:**

You find the binary in `target/release/veno-cli`.

```sh
veno-cli --config path/to/config.json
```

## 🔧 **Configuration**

All modes require a `config.json` file. Example configuration can be found [here](examples/CONFIG.md).  
You can also set values using environment variables via the `${}` syntax.

## 🛤️ **Features & Roadmap**

For upcoming features, check the [roadmap](ROADMAP.md).

## 💡 **Contributing**

Contributions are welcome! Fork the repository and submit a pull request.

## 📝 **License**

Veno is dual-licensed under:

- **Apache License, Version 2.0** ([LICENSE-APACHE](./LICENSE-APACHE))
- **MIT License** ([LICENSE-MIT](./LICENSE-MIT))
