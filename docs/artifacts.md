# Artifacts

## GitHub

For the identifier value you just take the part of the url with the `username/reponame`

```
{
  "id": "rust",
  "name": "Rust",
  "message_prefix": "There is a new version of:", // optional
  "source": {
    "type": "github",
    "identifier": "rust-lang/rust"
  },
  "current_version": "1.82.0",
},
```

## Artifacthub

For the identifier value you just take the part of the url with the `kind/organization/packagename`

```
{
  "id": "keycloak_helm_chart",
  "name": "Keycloak Helm Chart",
  "message_prefix": "There is a new version of:", // optional
  "current_version": "24.3.1",
  "source": {
    "type": "artifacthub",
    "identifier": "helm/bitnami/keycloak"
  },
},
```

## Dockerhub

For the identifier value you just take the part of the url with the `username/imagename`.
Official images have a special syntax `_/nginx` for example. For these you just use the image name `nginx`.

```
{
  "id": "nginx_dockerhub",
  "name": "nginx",
  "message_prefix": "There is a new version of:", // optional
  "current_version": "1.25.2-alpine3.20-perl",
  "source": {
    "type": "dockerhub",
    "identifier": "nginx"
  },
}
```
