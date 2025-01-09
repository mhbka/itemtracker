mod config;
mod galleries;
mod modules;
mod messages;
mod routes;

use axum::Router;
use config::{AppConfig, AxumConfig};
use modules::{AppModuleConnections, AppModules};
use tokio::net::TcpListener;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_config = AppConfig::load().unwrap();
    let module_connections = AppModuleConnections::new();
    let router= routes::build_router(&app_config.axum_config, &module_connections);
    let app_modules = AppModules::init(&app_config, module_connections).run();

    tracing::info!("App started");

    start_app(router, &app_config.axum_config).await;
}

async fn start_app(router: Router, axum_config: &AxumConfig) {
    let listener = TcpListener::bind(axum_config.host_addr.clone()).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
