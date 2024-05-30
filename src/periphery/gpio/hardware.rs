use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::periphery::gpio::model::GpioState;

pub trait GpioHardwareService : Send + Sync {
    fn get_state(&self, pin: u8) -> GpioState;

    fn set_state(&self, pin: u8, state: GpioState);
}

pub fn create_gpio_hardware_service() -> Arc<dyn GpioHardwareService> {
    Arc::new(
        FakeGpioHardwareService {
            states: Mutex::new(
                HashMap::new()
            )
        }
    )
}

struct FakeGpioHardwareService {
    states: Mutex<HashMap<u8, GpioState>>,
}

impl GpioHardwareService for FakeGpioHardwareService {
    fn get_state(&self, pin: u8) -> GpioState {
        let states = self.states.lock();
        states.unwrap()
            .get(&pin)
            .copied()
            .unwrap_or(GpioState::Low)
    }

    fn set_state(&self, pin: u8, state: GpioState) {
        let states = self.states.lock();
        states.unwrap()
            .insert(pin, state);
    }
}
