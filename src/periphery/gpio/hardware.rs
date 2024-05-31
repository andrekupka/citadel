use std::sync::Arc;

use crate::periphery::gpio::model::GpioState;
use crate::periphery::pin::hardware::{FakePinHardwareService, PinHardwareService};

pub trait GpioHardwareService: PinHardwareService<GpioState> {
}

impl GpioHardwareService for FakePinHardwareService<GpioState> {
}

pub fn create_gpio_hardware_service() -> Arc<dyn GpioHardwareService> {
    FakePinHardwareService::new()
}
