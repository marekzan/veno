# Roadmap and Features

## Modes

| **Status**     | **Features** |
| -------------- | ------------ |
| ✅ Supported   | cli          |
| 🚧 In Progress | web          |
| 🔜 Planned     | daemon       |

## Sources

| Status       | Features    |
| ------------ | ----------- |
| ✅ Supported | GitHub      |
| ✅ Supported | Artifacthub |
| ✅ Supported | Docker Hub  |
| 🔜 Planned   | GitLab      |
| 🔜 Planned   | Bitbucket   |
| 🔜 Planned   | Quay        |

## Sinks

| Status           | Features        |
| ---------------- | --------------- |
| ✅ Supported     | Generic Webhook |
| ✅ Supported     | Email           |
| 🚧 Basic support | Slack           |
| 🚧 Basic support | Google Chat     |
| 🔜 Planned       | Microsoft Teams |
| 🔜 Planned       | Rocket Chat     |

## Features

| Status       | Features                             |
| ------------ | ------------------------------------ |
| ✅ Supported | multiple notifiers for a sink        |
| ✅ Supported | environment variables in config file |
| ✅ Supported | multiple recipients for email sink   |

## Ideas

| Status     | Features                                                                                                                  |
| ---------- | ------------------------------------------------------------------------------------------------------------------------- |
| 🔜 Planned | make notifiers have references to artifacts so that only one message is being sent to the sink for multiple artifacts     |
| 🔜 Planned | yaml config support                                                                                                       |
| 🔜 Planned | toml config support                                                                                                       |
| 🔜 Planned | implement a config linter (check if notifiers, sources and references are valid)                                          |
| 🔜 Planned | Web endpoint to silence notifications for a specific artifact                                                             |
| 🔜 Planned | update the current_version field in the configuration file automatically                                                  |
| 🔜 Planned | decide on which version update you want to be notified (major, minor, patch or other version regex)                       |
| 🔜 Planned | include alpha, beta, and release candidate versions and allow users to specify which version to track                     |
| 🔜 Planned | scheduling for periodic checks (daemon mode)                                                                              |
| 🔜 Planned | custom Chat Cards for Sinks (if supported)                                                                                |
| 🔜 Planned | alpha, beta, and release candidate versions                                                                               |
| 🔜 Planned | give Veno a repo address where the config file is located. this allows veno to pull and update the config file.           |
| 🔜 Planned | multiple config files for different teams (including repos with config files)                                             |
| 🔜 Planned | script file template to run commands for veno which runs in a k8s pod                                                     |
| 🔜 Planned | Be able to split artifact and notifier definitions and merge them. So that different teams can maintain their own configs |
| 🔜 Planned | Make the version logic more robust (own crate; schema detection; schema logic, etc.)                                      |
| 🔜 Planned | Option to also notify with the changelog or just the link to the changelog                                                |
