# Artifacts

## GitHub

For the identifier value you just take the part of the url with the `username/reponame`

```
{
  "name": "Rust",
  "message_prefix": "There is a new version of:", // optional
  "source": {
    "type": "github",
    "identifier": "rust-lang/rust"
  },
  "current_version": "",
  "notifier": ["team_slack", "private_email"]
},
```

## Artifacthub

For the identifier value you just take the part of the url with the `kind/organization/packagename`

```
{
  "name": "Keycloak Helm Chart",
  "message_prefix": "There is a new version of:", // optional
  "current_version": "24.3.1",
  "source": {
    "type": "artifacthub",
    "identifier": "helm/bitnami/keycloak"
  },
  "notifier": ["generic_webhook", "team_google_chat"]
},
```

## Dockerhub

For the identifier value you just take the part of the url with the `username/imagename`.
Official images have a special syntax `_/nginx` for example. For these you just use the image name `nginx`.

```
{
  "name": "nginx",
  "message_prefix": "There is a new version of:", // optional
  "current_version": "1.25.2-alpine3.20-perl",
  "source": {
    "type": "dockerhub",
    "identifier": "nginx"
  },
  "notifier": ["private_email"]
}
```
