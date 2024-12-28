use anyhow::Result;
use github::GithubSource;
use serde::Deserialize;

pub mod github;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")] // Use tag-based enum for source type
pub enum Source {
    #[serde(rename = "github")]
    Github(GithubSource),
}

impl Source {
    pub async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        match self {
            Source::Github(source) => source.is_version_behind(current_version).await,
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait SourceChecker: Send {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>>;
}
