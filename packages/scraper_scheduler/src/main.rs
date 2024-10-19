mod input;
mod scheduler;
mod output;
mod galleries;
mod state;
mod config;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use state::AppState;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new().await.unwrap()));
    let app = input::build_routes(app_state.clone());

    let host_addr = &app_state.lock().await.config.host_addr.clone();
    let listener = TcpListener::bind(host_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}