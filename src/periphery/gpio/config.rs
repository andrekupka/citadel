use serde::Deserialize;

use crate::periphery::gpio::model::{GpioEntityKind, GpioEntityMetadata};

#[derive(Debug, Deserialize)]
pub struct GpioConfig {
    pub entities: Vec<GpioEntityConfig>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpioKindConfig {
    Light,
    Fan,
    Generic,
}

#[derive(Debug, Deserialize)]
pub struct GpioEntityConfig {
    pub id: String,
    pub name: String,
    pub kind: GpioKindConfig,
    pub pin: u8,
}

impl From<GpioKindConfig> for GpioEntityKind {

    fn from(value: GpioKindConfig) -> Self {
        match value {
            GpioKindConfig::Light => Self::Light,
            GpioKindConfig::Fan => Self::Fan,
            GpioKindConfig::Generic => Self::Generic,
        }
    }
}

impl From<&GpioEntityConfig> for GpioEntityMetadata {
    fn from(value: &GpioEntityConfig) -> GpioEntityMetadata {
        GpioEntityMetadata {
            id: value.id.clone(),
            name: value.name.clone(),
            kind: value.kind.into(),
            pin: value.pin,
        }
    }
}