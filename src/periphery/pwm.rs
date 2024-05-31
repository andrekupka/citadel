use crate::periphery::pwm::config::PwmConfig;
use crate::periphery::pwm::hardware::create_pwm_hardware_service;
use crate::periphery::pwm::rest::PwmRouteContributor;
use crate::periphery::pwm::service::create_pwm_service;

pub mod config;
pub mod hardware;
pub mod model;
pub mod rest;
pub mod service;
pub mod transport;

pub fn initialize_route_contributor(config: &PwmConfig) -> Box<PwmRouteContributor> {
    let hardware_service = create_pwm_hardware_service();
    let service = create_pwm_service(hardware_service, &config.entities);
    PwmRouteContributor::new(service)
}
