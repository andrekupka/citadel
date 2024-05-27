use std::path::Path;
use crate::config::error::ConfigError;
use crate::config::model::Config;

pub async fn read_config_from_path<T>(path: T) -> Result<Config, ConfigError>
    where T : Into<String> + AsRef<Path> {
    let content = tokio::fs::read_to_string(path).await?;
    parse_config(content.as_str())
}

fn parse_config(content: &str) -> Result<Config, ConfigError> {
    let config_result: Result<Config, toml::de::Error> = toml::from_str(content);

    match config_result {
        Ok(config) => Ok(config),
        Err(e) => Err(ConfigError::TomlError(e))
    }
}
