mod common;
mod web;

#[tokio::main]
async fn main() {
    common::logging::init();

    web::app::run().await;
}
