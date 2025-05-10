mod config;
mod domain;
mod pipeline;
mod stores;
mod routes;
mod utils;
mod schema;
mod models;
mod auth;
mod app_state;

use std::error::Error;

use app_state::AppState;
use config::AppConfig;
use pipeline::Pipeline;
use stores::AppStores;
use tokio::net::TcpListener;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_config = AppConfig::load().unwrap();
    let axum_config = app_config.axum.clone();
    let mut stores = AppStores::new(&app_config.store); 
    let pipeline = Pipeline::init(app_config, &mut stores).await;

    let app_state = AppState {
        stores: stores.clone(),
        pipeline
    };
    
    let router = routes::build_router(app_state);
    
    tracing::info!("App fully initialized");

    let listener = TcpListener::bind("0.0.0.0:443")
        .await
        .unwrap();
    axum::serve(listener, router)
        .await
        .unwrap();
}
