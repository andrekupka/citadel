use serde::Deserialize;
use crate::periphery::gpio::config::GpioConfig;
use crate::periphery::pwm::config::PwmConfig;

#[derive(Debug, Deserialize)]
pub struct PeripheryConfig {
    pub gpio: GpioConfig,
    pub pwm: PwmConfig,
}
