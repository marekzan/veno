pub mod source;
mod version_checker;

use serde::Deserialize;
use source::Source;

#[derive(Deserialize, Debug, Clone)]
pub struct Artifact {
    pub id: String,
    pub name: String,
    pub message_prefix: Option<String>,
    pub source: Source,
    pub current_version: String,
}
