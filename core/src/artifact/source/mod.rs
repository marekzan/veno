use anyhow::Result;
use artifacthub::ArtifactHubSource;
use dockerhub::DockerHubSource;
use github::GitHubSource;
use serde::Deserialize;

pub mod artifacthub;
pub mod dockerhub;
pub mod github;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")] // Use tag-based enum for source type
pub enum Source {
    #[serde(rename = "github")]
    GitHub(GitHubSource),
    #[serde(rename = "dockerhub")]
    DockerHub(DockerHubSource),
    #[serde(rename = "artifacthub")]
    ArtifactHub(ArtifactHubSource),
}

impl Source {
    pub(super) async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        match self {
            Source::GitHub(source) => source.is_version_behind(current_version).await,
            Source::DockerHub(source) => source.is_version_behind(current_version).await,
            Source::ArtifactHub(source) => source.is_version_behind(current_version).await,
        }
    }
}

trait SourceChecker: Send {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>>;
}
