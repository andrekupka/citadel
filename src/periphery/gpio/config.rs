use serde::Deserialize;

use crate::periphery::gpio::model::GpioEntityKind;
use crate::periphery::pin::config::{PinEntityKindConfig, PinEntityMetadataConfig};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpioEntityKindConfig {
    Light,
    Fan,
    Generic,
}

impl PinEntityKindConfig for GpioEntityKindConfig {}

#[derive(Debug, Deserialize)]
pub struct GpioConfig {
    pub entities: Vec<GpioEntityConfig>,
}

pub type GpioEntityMetadataConfig = PinEntityMetadataConfig<GpioEntityKindConfig>;

#[derive(Debug, Deserialize)]
pub struct GpioEntityConfig {
    #[serde(flatten)]
    pub metadata: GpioEntityMetadataConfig,
}

impl From<GpioEntityKindConfig> for GpioEntityKind {
    fn from(value: GpioEntityKindConfig) -> Self {
        match value {
            GpioEntityKindConfig::Light => Self::Light,
            GpioEntityKindConfig::Fan => Self::Fan,
            GpioEntityKindConfig::Generic => Self::Generic,
        }
    }
}