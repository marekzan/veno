use crate::get;

use super::SourceChecker;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    pub packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArtifactHubSource {
    pub name: String,
    pub kind: Option<ArtifactKind>,
    pub org: Option<String>,
    pub user: Option<String>,
}

impl ArtifactHubSource {
    fn handle_org_user(&self) -> String {
        match (&self.org, &self.user) {
            (Some(org), Some(user)) => format!("&org={}&user={}", org, user),
            (Some(org), None) => format!("&org={}", org),
            (None, Some(user)) => format!("&user={}", user),
            (None, None) => "".to_string(),
        }
    }
}

impl SourceChecker for ArtifactHubSource {
    async fn is_version_behind(&self, current_version: &str) -> Result<Option<String>> {
        let kind = match self.kind {
            Some(kind) => format!("&kind={}", kind.to_value()),
            None => String::from(""),
        };

        let org_user = self.handle_org_user();

        let source = format!(
            "https://artifacthub.io/api/v1/packages/search?limit=1{}{}&ts_query_web={}",
            kind, org_user, self.name
        );

        let response = get(&source)
            .await
            .context("Failed to fetch latest release")?;

        // Check for successful HTTP status
        if !response.status().is_success() {
            return Err(anyhow!("Request failed: {:?}", response));
        }

        // Parse the JSON response
        let release: Release = response
            .json()
            .await
            .context("Failed to parse JSON response")?;

        if let Some(package) = release.packages.first() {
            let latest_version = package.version.trim_start_matches('v');
            if latest_version > current_version {
                Ok(Some(latest_version.to_string()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ArtifactKind {
    HelmCharts = 0,
    FalcoRules = 1,
    OpaPolicies = 2,
    OlmOperators = 3,
    TinkerbellActions = 4,
    KrewKubectlPlugins = 5,
    HelmPlugins = 6,
    TektonTasks = 7,
    KedaScalers = 8,
    CoreDnsPlugins = 9,
    KeptnIntegrations = 10,
    TektonPipelines = 11,
    ContainerImages = 12,
    KubewardenPolicies = 13,
    GatekeeperPolicies = 14,
    KyvernoPolicies = 15,
    KnativeClientPlugins = 16,
    BackstagePlugins = 17,
    ArgoTemplates = 18,
    KubearmorTemplates = 19,
    KclPackages = 20,
    HeadlampPlugins = 21,
    InspektorGadgets = 22,
    TektonStepactions = 23,
    MesheryDesigns = 24,
    OpencostPlugins = 25,
    RadiusRecipes = 26,
}

impl ArtifactKind {
    pub fn to_value(&self) -> u8 {
        *self as u8
    }
}
