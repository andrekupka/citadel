use axum::Router;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
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

pub trait RouteContributor {

    fn contribute_routes(&self, router: Router) -> Router;
}

pub async fn run(host: String, port: u16, route_providers: &Vec<Box<dyn RouteContributor>>) {
    let listener = tokio::net::TcpListener::bind((host, port)).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    let router = route_providers.iter().fold(
        Router::new(),
        |router, contributor| {
            contributor.contribute_routes(router)
        }
    );

    axum::serve(listener, router).await.unwrap();
}
