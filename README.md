# Neveno

NEw VErsion NOtifier

If you need to keep track and update your 3rd party artifacts regularly, then this might be useful to you.
With neveno you can define artifacts and notifiers. Attach different/multiple notifiers to your artifacts
and neveno will notify you if there are new versions of your artifacts.

## Description

So what are artifacts and notifiers?

### Artifacts
An artifact is any (GitHub) repo which uses tags as its way of setting versions to releases.
You can define an artifact like so:

```json
  {
    "artifacts": [
      {
        "source": "https://api.github.com/repos/${username}/${repo_name}/releases/latest",
        "current_version": "23.0.6",
        "notifier": ["team_slack"]
      }
    ],
    ...
  }
```

### Notifier
A notifier is just a name with a sink. A definition where the notification should be send to.
We support currently only the following Sinks:
- Email
- Google Chat
- Slack

New Sinks are already planned (Microsoft Teams, Rocket Chat...)

A notifier looks like this:
```json
{
  ...
  "notifiers": [
    {
      "name": "team_slack",
      "sink": {...}
    },
  ]
}
```

Sinks have different configurations:
- Slack
```json
{
  ...
  "sink": {
    "type": "slack"
    "webhook": "..."
  }
}
```
- Google Chat
```json
{
  ...
  "sink": {
    "type": "google_chat"
    "webhook": "..."
  }
}
```
- Email
```json
{
  ...
  "sink": {
    "type": "email"
    "host": "smtp..."
    "port": "587"
    "username": "..."
    "password": "..."
  }
}
```

## Usage

There are currently two ways of operation:
1. cli
  the cli is just a one time execution programm.
3. web
  the web is an axum service with endpoints to trigger a check, get a list of versions and other usefull triggers in the future.

Every kind of operation needs a config.json to be passed via the `--config` parameter.
An example `config.json` is in the example folder.

## Contributing

Contributions are welcome. Please fork the repository and create a pull request.

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
