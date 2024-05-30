use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PeripheryConfig {
    #[serde(default)]
    pub gpio: Vec<GpioConfig>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpioKind {
    Light,
    Fan,
    Generic,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GpioConfig {
    id: String,
    name: String,
    kind: GpioKind,
    pin: u8,
}