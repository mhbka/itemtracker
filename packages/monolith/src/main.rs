mod config;
mod domain;
mod scraping_pipeline;
mod messages;
mod stores;
mod routes;
mod utils;
mod schema;
mod models;
mod auth;

use config::AppConfig;
use scraping_pipeline::{AppModuleConnections, AppModules};
use stores::AppStores;
use tokio::net::TcpListener;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_config = AppConfig::load().unwrap();
    let axum_config = app_config.axum_config.clone();

    let stores = AppStores::new(&app_config.store_config); 

    let module_connections = AppModuleConnections::new();
    let router = routes::build_router(&app_config.axum_config, &module_connections, &stores);
    let app_modules = AppModules::init(app_config, module_connections, &stores).await.run();

    tracing::info!("App started");

    let listener = TcpListener::bind(axum_config.host_addr.clone())
        .await
        .unwrap();
    axum::serve(listener, router)
        .await
        .unwrap();
}
