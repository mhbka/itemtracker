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
mod app_state;

use app_state::AppState;
use config::AppConfig;
use scraping_pipeline::{PipelineConnections, AppModules};
use stores::AppStores;
use tokio::net::TcpListener;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_config = AppConfig::load().unwrap();
    let axum_config = app_config.axum_config.clone();
    let mut stores = AppStores::new(&app_config.store_config); 
    let module_connections = PipelineConnections::new();

    let app_state = AppState {
        stores: stores.clone(),
        scheduler_sender: module_connections.scraper_scheduler.0.clone(),
        search_scraper_sender: module_connections.search_scraper.0.clone()
    };

    let app_modules = AppModules::init(app_config, module_connections, &mut stores).await.run();
    
    let router = routes::build_router(app_state);

    tracing::info!("App started");

    let listener = TcpListener::bind(axum_config.host_addr.clone())
        .await
        .unwrap();
    axum::serve(listener, router)
        .await
        .unwrap();
}
