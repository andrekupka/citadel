use serde::{Deserialize, Serialize};
use crate::periphery::gpio::model::{GpioEntity, GpioEntityKind, GpioEntityState};

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

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpioEntityStateDto {
    Low,
    High,
}

impl From<GpioEntityState> for GpioEntityStateDto {
    fn from(value: GpioEntityState) -> Self {
        match value {
            GpioEntityState::Low => Self::Low,
            GpioEntityState::High => Self::High,
        }
    }
}

#[derive(Serialize)]
pub struct GpioEntityDto {
    pub id: String,
    pub name: String,
    pub kind: GpioEntityKindDto,
    pub state: GpioEntityStateDto,
}

#[derive(Serialize)]
pub struct GpioEntityContainerDto {
    pub entities: Vec<GpioEntityDto>,
}

impl From<&GpioEntity> for GpioEntityDto {
    fn from(value: &GpioEntity) -> Self {
        Self {
            id: value.metadata.id.clone(),
            name: value.metadata.name.clone(),
            kind: value.metadata.kind.into(),
            state: value.state.into(),
        }
    }
}
