use axum::Router;
use axum::routing::get;

use crate::web::info::get_info;

fn initialize_web_app() -> Router {
    Router::new()
        .route("/info", get(get_info))
}

pub async fn run(host: String, port: u16) {
    let app = initialize_web_app();

    let listener = tokio::net::TcpListener::bind((host, port)).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
