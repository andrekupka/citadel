use serde::{Deserialize, Serialize};

use crate::periphery::gpio::model::{GpioEntityKind, GpioState};
use crate::periphery::pin::transport::{PinEntityContainerDto, PinEntityDto, PinEntityUpdateDto};

pub type GpioEntityDto = PinEntityDto<GpioEntityKindDto, GpioStateDto>;

pub type GpioEntityContainerDto = PinEntityContainerDto<GpioEntityKindDto, GpioStateDto>;

pub type GpioEntityUpdateDto = PinEntityUpdateDto<GpioStateDto>;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum GpioEntityKindDto {
    Light,
    Fan,
    Generic,
}

impl From<GpioEntityKind> for GpioEntityKindDto {
    fn from(value: GpioEntityKind) -> Self {
        match value {
            GpioEntityKind::Light => Self::Light,
            GpioEntityKind::Fan => Self::Fan,
            GpioEntityKind::Generic => Self::Generic,
        }
    }
}

impl From<GpioEntityKindDto> for GpioEntityKind {
    fn from(value: GpioEntityKindDto) -> Self {
        match value {
            GpioEntityKindDto::Light => Self::Light,
            GpioEntityKindDto::Fan => Self::Fan,
            GpioEntityKindDto::Generic => Self::Generic,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpioStateDto {
    Low,
    High,
}

impl From<GpioState> for GpioStateDto {
    fn from(value: GpioState) -> Self {
        match value {
            GpioState::Low => Self::Low,
            GpioState::High => Self::High,
        }
    }
}

impl From<GpioStateDto> for GpioState {
    fn from(value: GpioStateDto) -> Self {
        match value {
            GpioStateDto::Low => Self::Low,
            GpioStateDto::High => Self::High,
        }
    }
}