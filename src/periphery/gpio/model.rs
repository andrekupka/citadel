use crate::periphery::pin::model::{PinEntity, PinEntityKind, PinEntityMetadata, PinState};

#[derive(Clone, Copy, Debug)]
pub enum GpioState {
    Low,
    High,
}

impl Default for GpioState {
    fn default() -> Self {
        GpioState::Low
    }
}

impl PinState for GpioState {
}

impl GpioState {
    pub fn invert(&self) -> Self {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GpioEntityKind {
    Light,
    Fan,
    Generic,
}

impl PinEntityKind for GpioEntityKind {
}

pub type GpioEntity = PinEntity<GpioEntityKind, GpioState>;

pub type GpioEntityMetadata = PinEntityMetadata<GpioEntityKind>;
