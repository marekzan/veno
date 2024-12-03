use std::{future::Future, pin::Pin, time::Duration};

use anyhow::Result;
use serde::Deserialize;

use crate::CLIENT;

use super::SourceChecker;

#[derive(Deserialize, Debug, Clone)]
pub struct GithubSource {
    pub repo: String,
}

impl SourceChecker for GithubSource {
    fn is_version_behind<'a>(
        &'a self,
        current_version: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>>> + Send + Sync + 'a>> {
        Box::pin(async move {
            let source = format!("https://api.github.com/repos/{}/releases/latest", self.repo);
            let response = CLIENT
                .get(&source)
                .header("User-Agent", "neveno")
                .timeout(Duration::from_secs(10))
                .send()
                .await?;

            let release: Release = response.json().await?;
            let latest_version = release.tag_name.trim_start_matches('v').to_string();

            if latest_version.as_str() > current_version {
                Ok(Some(latest_version))
            } else {
                Ok(None)
            }
        })
    }
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}
