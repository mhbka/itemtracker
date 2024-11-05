mod config;
mod galleries;
mod modules;
mod messages;
mod routes;

use axum::Router;
use config::{AppConfig, AxumConfig};
use modules::AppModules;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let mut router= Router::new();
    let app_config = AppConfig::load().unwrap();
    let _app_modules = AppModules::new(&mut router, app_config.clone()).await.unwrap();

    start_app(router, app_config.axum_config.clone()).await;
}

async fn start_app(router: Router, axum_config: AxumConfig) {
    let listener = TcpListener::bind(axum_config.host_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
