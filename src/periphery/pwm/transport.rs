use serde::{Deserialize, Serialize};

use crate::periphery::pin::transport::{PinEntityContainerDto, PinEntityDto, PinEntityUpdateDto};
use crate::periphery::pwm::model::{PwmEntityKind, PwmState};

pub type PwmEntityDto = PinEntityDto<PwmEntityKindDto, PwmStateDto>;

pub type PwmEntityContainerDto = PinEntityContainerDto<PwmEntityKindDto, PwmStateDto>;

pub type PwmEntityUpdateDto = PinEntityUpdateDto<PwmStateDto>;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum PwmEntityKindDto {
    Light,
    Fan,
    Generic,
}

impl From<PwmEntityKind> for PwmEntityKindDto {
    fn from(value: PwmEntityKind) -> Self {
        match value {
            PwmEntityKind::Light => Self::Light,
            PwmEntityKind::Fan => Self::Fan,
            PwmEntityKind::Generic => Self::Generic,
        }
    }
}

impl From<PwmEntityKindDto> for PwmEntityKind {
    fn from(value: PwmEntityKindDto) -> Self {
        match value {
            PwmEntityKindDto::Light => Self::Light,
            PwmEntityKindDto::Fan => Self::Fan,
            PwmEntityKindDto::Generic => Self::Generic,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PwmStateDto(f64);


impl From<PwmState> for PwmStateDto {
    fn from(value: PwmState) -> Self {
        Self(value)
    }
}

impl From<PwmStateDto> for PwmState {
    fn from(value: PwmStateDto) -> Self {
        value.0
    }
}