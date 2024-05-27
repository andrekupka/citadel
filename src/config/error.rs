use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    TomlError(toml::de::Error)
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) =>  {
                write!(f, "Failed to read file: {}", e)
            },
            ConfigError::TomlError(e) => {
                write!(f, "Failed to parse file: {}", e)
            }
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        ConfigError::IoError(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        ConfigError::TomlError(value)
    }
}