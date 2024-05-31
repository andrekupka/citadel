use crate::periphery::pin::model::{PinEntity, PinEntityKind, PinState};

pub type PwmState = f64;

impl PinState for PwmState {
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PwmEntityKind {
    Light,
    Fan,
    Generic,
}

impl PinEntityKind for PwmEntityKind {
}

pub type PwmEntity = PinEntity<PwmEntityKind, PwmState>;
