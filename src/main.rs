use std::process::ExitCode;

use tracing::{debug, error};

use crate::app::config::AppConfig;
use crate::app::info::InfoRouteContributor;
use crate::app::rest::RouteContributor;

mod app;
mod periphery;

async fn read_config() -> Result<AppConfig, ExitCode> {
    let config_result = app::config::read_config_from_path("./example/config.toml").await;
    match config_result {
        Ok(config) => {
            debug!("Read configuration: {:?}", config);
            Ok(config)
        },
        Err(e) => {
            error!("{}", e);
            Err(ExitCode::from(1))
        }
    }
}

async fn try_main() -> Result<(), ExitCode> {
    app::logging::init();

    let config = read_config().await?;

    let contributors: Vec<Box<dyn RouteContributor>> = vec![
        InfoRouteContributor::new(),
        periphery::gpio::initialize_route_contributor(&config.periphery.gpio),
        periphery::pwm::initialize_route_contributor(&config.periphery.pwm)
    ];

    app::rest::run(config.server.host, config.server.port, &contributors).await;

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let result = try_main().await;
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(exit_code) => exit_code,
    }
}
