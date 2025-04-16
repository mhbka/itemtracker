use std::env::{self, VarError};
use serde::{Deserialize, Serialize};

pub use item_analysis::ItemAnalysisConfig;
pub use image_classifier::ItemEmbedderConfig;
pub use search_scraper::SearchScraperConfig;
pub use item_scraper::ItemScraperConfig;
pub use scraper_scheduler::SchedulerConfig;
use state_tracker::StateTrackerConfig;

pub mod state_tracker;
pub mod scraper_scheduler;
pub mod search_scraper;
pub mod item_scraper;
pub mod item_analysis;
pub mod image_classifier;

/// Holds all types of configs for the app.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub axum: AxumConfig,
    pub state_tracker: StateTrackerConfig,
    pub scraper_scheduler: SchedulerConfig,
    pub search_scraper: SearchScraperConfig,
    pub item_scraper: ItemScraperConfig,
    pub item_analysis: ItemAnalysisConfig,
    pub item_embedder: ItemEmbedderConfig,
    pub store: StoreConfig
}

impl AppConfig {
    /// Load all configs from a .env file. Returns a `VarError` if any are missing.
    pub fn load() -> Result<Self, VarError> {
        Ok(
            AppConfig {
                axum: AxumConfig::load()?,
                state_tracker: StateTrackerConfig::load()?,
                scraper_scheduler: SchedulerConfig::load()?,
                search_scraper: SearchScraperConfig::load()?,
                item_scraper: ItemScraperConfig::load()?,
                item_analysis: ItemAnalysisConfig::load()?,
                item_embedder: ItemEmbedderConfig::load()?,
                store: StoreConfig::load()?
            }
        )
    }
}

/// Config for the top-level Axum app:
/// - `host_addr`: The address the app will run on
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxumConfig {
    pub host_addr: String
}

impl AxumConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            AxumConfig {
                host_addr: env::var("HOST_ADDR")?
            }
        )
    }
}

/// Config for stores:
/// - `database_url`: The URL of the database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreConfig {
    pub database_url: String
}

impl StoreConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        let mut database_url = env::var("DATABASE_URL")?;

        // this option isn't supported in Diesel I think
        database_url = database_url.trim_end_matches("?gssencmode=disable").to_string();

        Ok(
            StoreConfig {
                database_url
            }
        )
    }
}











