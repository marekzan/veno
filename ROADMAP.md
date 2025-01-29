# Roadmap and Features

## Modes

| **Status** | **Features** |
| ---------- | ------------ |
| working    | cli          |
| planned    | web          |
| planned    | daemon       |

## Sources

| Status  | Features    |
| ------- | ----------- |
| working | GitHub      |
| working | Artifacthub |
| working | Docker Hub  |
| planned | GitLab      |
| planned | Bitbucket   |
| planned | Quay        |

## Sinks

| Status      | Features        |
| ----------- | --------------- |
| working     | Generic Webhook |
| working     | Email           |
| in progress | Slack           |
| in progress | Google Chat     |
| planned     | Microsoft Teams |
| planned     | Rocket Chat     |

## Features

| Status      | Features                                                                                                              |
| ----------- | --------------------------------------------------------------------------------------------------------------------- |
| working     | environment variables in config file                                                                                  |
| working     | multiple recipients for email sink                                                                                    |
| in progress | make notifiers have references to artifacts so that only one message is being sent to the sink for multiple artifacts |

## Ideas

| Status  | Features                                                                                                                  |
| ------- | ------------------------------------------------------------------------------------------------------------------------- |
| planned | yaml config support                                                                                                       |
| planned | toml config support                                                                                                       |
| planned | implement a config linter (check if notifiers, sources and references are valid)                                          |
| planned | Web endpoint to silence notifications for a specific artifact                                                             |
| planned | update the current_version field in the configuration file automatically                                                  |
| planned | decide on which version update you want to be notified (major, minor, patch or other version regex)                       |
| planned | include alpha, beta, and release candidate versions and allow users to specify which version to track                     |
| planned | scheduling for periodic checks (daemon mode)                                                                              |
| planned | custom Chat Cards for Sinks (if supported)                                                                                |
| planned | alpha, beta, and release candidate versions                                                                               |
| planned | give Veno a repo address where the config file is located. this allows veno to pull and update the config file.           |
| planned | multiple config files for different teams (including repos with config files)                                             |
| planned | script file template to run commands for veno which runs in a k8s pod                                                     |
| planned | Be able to split artifact and notifier definitions and merge them. So that different teams can maintain their own configs |
