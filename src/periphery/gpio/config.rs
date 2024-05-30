use serde::Deserialize;
use crate::periphery::gpio::model::{GpioEntityKind, GpioEntityMetadata};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GpioKindConfig {
    Light,
    Fan,
    Generic,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GpioConfig {
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

impl From<&GpioConfig> for GpioEntityMetadata {
    fn from(value: &GpioConfig) -> GpioEntityMetadata {
        GpioEntityMetadata {
            id: value.id.clone(),
            name: value.name.clone(),
            kind: value.kind.into(),
            pin: value.pin,
        }
    }
}