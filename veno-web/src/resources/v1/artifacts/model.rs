use serde::Serialize;
use utoipa::ToSchema;
use veno_core::artifact::{source::Source, Artifact};

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct ArtifactResponse {
    pub id: String,
    pub name: String,
    pub message_prefix: Option<String>,
    pub source: SourceDto,
    pub current_version: String,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(tag = "type")] // Use tag-based enum for source type
pub enum SourceDto {
    #[serde(rename = "github")]
    GitHub(GitHubSource),
    #[serde(rename = "dockerhub")]
    DockerHub(DockerHubSource),
    #[serde(rename = "artifacthub")]
    ArtifactHub(ArtifactHubSource),
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct GitHubSource {
    pub identifier: String,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct DockerHubSource {
    pub identifier: String,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct ArtifactHubSource {
    pub identifier: String,
}

impl From<Artifact> for ArtifactResponse {
    fn from(value: Artifact) -> Self {
        Self {
            id: value.id,
            name: value.name,
            message_prefix: value.message_prefix,
            source: value.source.into(),
            current_version: value.current_version,
        }
    }
}
impl From<Source> for SourceDto {
    fn from(original_source: Source) -> Self {
        match original_source {
            Source::GitHub(gh_source) => SourceDto::GitHub(GitHubSource {
                identifier: gh_source.identifier,
            }),
            Source::DockerHub(dh_source) => SourceDto::DockerHub(DockerHubSource {
                identifier: dh_source.identifier,
            }),
            Source::ArtifactHub(ah_source) => SourceDto::ArtifactHub(ArtifactHubSource {
                identifier: ah_source.identifier,
            }),
        }
    }
}
