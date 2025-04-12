use anyhow::{Context, Result};
use std::{borrow::Cow, env, fs};

use config::{Config, File, FileFormat};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

pub struct AppConfig;

impl AppConfig {
    pub fn load(file_path: &str) -> Result<Config> {
        let mut config =
            fs::read_to_string(file_path).context("Could not read config file from filesystem")?;

        replace_env_placeholders(&mut config);

        let config = Config::builder()
            .add_source(File::from_str(&config, FileFormat::Json))
            .build()
            .context("Could not build Config from raw config string")?;

        Ok(config)
    }
}

static RE_ENV: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{([^}]+)\}").expect("Invalid regex"));

fn replace_env_placeholders(config: &mut String) {
    let result = RE_ENV.replace_all(config, |caps: &Captures| {
        // caps[0] is "${VAR}"
        // caps[1] is "VAR"
        let var_name = &caps[1];
        // TODO is there a better way to handle this? It would be better to bubble this up since it is not recoverable
        match env::var(var_name) {
            Ok(val) => val,
            Err(_) => panic!("Could not find env.var = {}", &caps[0]),
        }
    });

    if let Cow::Owned(new_content) = result {
        *config = new_content;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_on_existing_env() {
        env::set_var("EXISTING_VAR", "test_value");
        let mut config = "key=${EXISTING_VAR}".to_string();
        replace_env_placeholders(&mut config);
        assert_eq!(config, "key=test_value");
    }

    #[test]
    #[should_panic]
    fn panic_on_missing_env() {
        let mut config = "key=${NOT_EXISTING_VAR}".to_string();
        replace_env_placeholders(&mut config);
    }
}
