use std::{future::Future, pin::Pin};

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
    pub fn unwrap(&self) -> Box<dyn SourceChecker> {
        match self {
            Source::Github(source) => Box::new(source.clone()),
        }
    }
}

// Send + Sync is required for Axum Handler
pub trait SourceChecker: Send + Sync {
    fn is_version_behind<'a>(
        &'a self,
        current_version: &'a str,
        // This is needed to allow async methods in the trait
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>>> + Send + Sync + 'a>>;
}
