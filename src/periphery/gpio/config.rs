use serde::Deserialize;
use crate::periphery::gpio::model::GpioEntityKind;
use crate::periphery::pin::config::{PinConfig, PinEntityConfig};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpioKindConfig {
    Light,
    Fan,
    Generic,
}

pub type GpioConfig = PinConfig<GpioKindConfig>;

pub type GpioEntityConfig = PinEntityConfig<GpioKindConfig>;

impl From<GpioKindConfig> for GpioEntityKind {
    fn from(value: GpioKindConfig) -> Self {
        match value {
            GpioKindConfig::Light => Self::Light,
            GpioKindConfig::Fan => Self::Fan,
            GpioKindConfig::Generic => Self::Generic,
        }
    }
}