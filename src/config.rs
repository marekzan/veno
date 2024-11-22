use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub source: String,
    pub current_version: String,
}

impl AppConfig {
    pub fn load(file_path: &str) -> Result<Self, ConfigError> {
        let builder = Config::builder().add_source(File::new(file_path, FileFormat::Json));

        match builder.build() {
            Ok(config) => match config.try_deserialize() {
                Ok(app_config) => Ok(app_config),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}
