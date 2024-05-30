use crate::periphery::gpio::config::GpioConfig;
use crate::periphery::gpio::rest::GpioRouteContributor;
use crate::periphery::gpio::service::create_gpio_service;

pub mod config;
pub mod rest;
pub mod service;
mod model;
mod transport;

pub fn initialize_route_contributor(config: &Vec<GpioConfig>) -> Box<GpioRouteContributor> {
    let service = create_gpio_service(config);
    GpioRouteContributor::new(service)
}