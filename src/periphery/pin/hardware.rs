use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::periphery::pin::model::PinState;

pub trait PinHardwareService<S>: Send + Sync
    where S: PinState {
    fn get_state(&self, pin: u8) -> S;

    fn set_state(&self, pin: u8, state: S);
}

pub struct FakePinHardwareService<S>
    where S: PinState {
    states: Mutex<HashMap<u8, S>>,
}

impl<S> FakePinHardwareService<S>
    where S: PinState {
    pub fn new() -> Arc<FakePinHardwareService<S>> {
        Arc::new(FakePinHardwareService {
            states: Mutex::new(
                HashMap::new()
            )
        })
    }
}

impl<S> PinHardwareService<S> for FakePinHardwareService<S>
    where S: PinState {
    fn get_state(&self, pin: u8) -> S {
        let states = self.states.lock();
        states.unwrap()
            .get(&pin)
            .copied()
            .unwrap_or(Default::default())
    }

    fn set_state(&self, pin: u8, state: S) {
        let states = self.states.lock();
        states.unwrap()
            .insert(pin, state);
    }
}