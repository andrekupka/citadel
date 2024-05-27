use std::process::ExitCode;
use tracing::error;
use crate::config::model::Config;

mod common;
mod web;
mod config;

async fn read_config() -> Result<Config, ExitCode> {
    let config_result = config::reader::read_config_from_path("./example/config.toml").await;
    match config_result {
        Ok(config) => Ok(config),
        Err(e) => {
            error!("{}", e);
            Err(ExitCode::from(1))
        }
    }
}

async fn try_main() -> Result<(), ExitCode> {
    common::logging::init();

    let config = read_config().await?;

    web::app::run(config.server.host, config.server.port).await;

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
