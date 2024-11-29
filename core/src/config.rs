use anyhow::{Context, Result};
use config::{Config, File, FileFormat};
use serde::Deserialize;

use crate::{sink::Sink, source::Artifact};

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub artifacts: Vec<Artifact>,
    pub notifiers: Vec<Notifier>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Notifier {
    pub name: String,
    pub sink: Sink,
}

impl AppConfig {
    pub fn load(file_path: &str) -> Result<Self> {
        let builder = Config::builder().add_source(File::new(file_path, FileFormat::Json));

        let config = builder.build()?;
        let app_config = config
            .try_deserialize()
            .context("Could not deserialize config to app_config")?;
        Ok(app_config)
    }
}
