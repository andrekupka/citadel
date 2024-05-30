use serde::Deserialize;
use crate::periphery::gpio::config::GpioConfig;

#[derive(Clone, Debug, Deserialize)]
pub struct PeripheryConfig {
    #[serde(default)]
    pub gpio: Vec<GpioConfig>,
}
