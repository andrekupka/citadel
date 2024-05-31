use serde::Deserialize;

use crate::periphery::pin::config::{PinEntityKindConfig, PinEntityMetadataConfig};
use crate::periphery::pwm::model::PwmEntityKind;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PwmEntityKindConfig {
    Light,
    Fan,
    Generic,
}

impl PinEntityKindConfig for PwmEntityKindConfig {}

#[derive(Debug, Deserialize)]
pub struct PwmConfig {
    pub entities: Vec<PwmEntityConfig>,
}

pub type PwmEntityMetadataConfig = PinEntityMetadataConfig<PwmEntityKindConfig>;

#[derive(Debug, Deserialize)]
pub struct PwmEntityConfig {
    #[serde(flatten)]
    pub metadata: PwmEntityMetadataConfig,
}

impl From<PwmEntityKindConfig> for PwmEntityKind {
    fn from(value: PwmEntityKindConfig) -> Self {
        match value {
            PwmEntityKindConfig::Light => Self::Light,
            PwmEntityKindConfig::Fan => Self::Fan,
            PwmEntityKindConfig::Generic => Self::Generic,
        }
    }
}