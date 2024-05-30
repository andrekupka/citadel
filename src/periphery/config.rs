use serde::Deserialize;

use crate::periphery::gpio::config::GpioConfig;

#[derive(Debug, Deserialize)]
pub struct PeripheryConfig {
    pub gpio: GpioConfig,
}
