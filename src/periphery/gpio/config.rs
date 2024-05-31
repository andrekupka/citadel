use serde::Deserialize;

use crate::periphery::gpio::model::GpioEntityKind;
use crate::periphery::pin::config::{PinConfig, PinEntityConfig, PinEntityKindConfig};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpioEntityKindConfig {
    Light,
    Fan,
    Generic,
}

impl PinEntityKindConfig for GpioEntityKindConfig {}

pub type GpioConfig = PinConfig<GpioEntityKindConfig>;

pub type GpioEntityConfig = PinEntityConfig<GpioEntityKindConfig>;

impl From<GpioEntityKindConfig> for GpioEntityKind {
    fn from(value: GpioEntityKindConfig) -> Self {
        match value {
            GpioEntityKindConfig::Light => Self::Light,
            GpioEntityKindConfig::Fan => Self::Fan,
            GpioEntityKindConfig::Generic => Self::Generic,
        }
    }
}