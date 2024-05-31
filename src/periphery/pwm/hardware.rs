use std::sync::Arc;

use crate::periphery::pin::hardware::{FakePinHardwareService, PinHardwareService};
use crate::periphery::pwm::model::PwmState;

pub trait PwmHardwareService: PinHardwareService<PwmState> {
}

impl PwmHardwareService for FakePinHardwareService<PwmState> {
}

pub fn create_pwm_hardware_service() -> Arc<dyn PwmHardwareService> {
    FakePinHardwareService::new()
}
