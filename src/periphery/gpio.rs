use crate::periphery::gpio::config::GpioConfig;
use crate::periphery::gpio::hardware::create_gpio_hardware_service;
use crate::periphery::gpio::rest::GpioRouteContributor;
use crate::periphery::gpio::service::create_gpio_service;

pub mod config;
pub mod hardware;
pub mod model;
pub mod rest;
pub mod service;
pub mod transport;


pub fn initialize_route_contributor(config: &GpioConfig) -> Box<GpioRouteContributor> {
    let hardware_service = create_gpio_hardware_service();
    let service = create_gpio_service(hardware_service, &config.entities);
    GpioRouteContributor::new(service)
}
