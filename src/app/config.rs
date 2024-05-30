use std::io;

use serde::Deserialize;
use thiserror::Error;

use crate::app::rest::ServerConfig;
use crate::periphery::config::PeripheryConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub periphery: PeripheryConfig,
}

pub async fn read_config_from_path<T>(path: T) -> Result<AppConfig, ConfigError>
    where T: Into<String> {
    let path_as_string = path.into();
    let content = tokio::fs::read_to_string(path_as_string.clone()).await.map_err(|e|
        ConfigError::IoError {
            path: path_as_string.clone(),
            cause: e,
        }
    )?;

    toml::from_str(content.as_str()).map_err(|e| {
        ConfigError::TomlError {
            path: path_as_string.clone(),
            cause: e,
        }
    })
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config from '{path}':\ncause: {cause}")]
    IoError {
        path: String,
        cause: io::Error,
    },
    #[error("failed to parse config from '{path}':\ncause: {cause}")]
    TomlError {
        path: String,
        cause: toml::de::Error,
    },
}
