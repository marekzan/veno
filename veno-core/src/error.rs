#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("could not read config from file\n{0}")]
    FailedToRead(std::io::Error),
    #[error("there was an error building the config\n{0}")]
    FailedToBuild(String),
    #[error("could not deserialize config object to app state\n{0}")]
    FailedToSerialize(String),
}
