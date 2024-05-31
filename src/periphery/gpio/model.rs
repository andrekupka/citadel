use crate::periphery::pin::model::{PinEntity, PinEntityMetadata};

#[derive(Clone, Copy, Debug)]
pub enum GpioState {
    Low,
    High,
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

pub type GpioEntity = PinEntity<GpioEntityKind, GpioState>;

pub type GpioEntityMetadata = PinEntityMetadata<GpioEntityKind>;
