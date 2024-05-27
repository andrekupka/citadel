use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
}

#[derive(Deserialize)]
pub struct Server {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}