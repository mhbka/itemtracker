use std::env::{self, VarError};
use serde::{Deserialize, Serialize};

pub use item_analysis::ItemAnalysisConfig;
pub use image_classifier::ItemEmbedderConfig;
pub use search_scraper::SearchScraperConfig;
pub use item_scraper::ItemScraperConfig;
pub use scraper_scheduler::ScraperSchedulerConfig;
use state_tracker::StateTrackerConfig;
pub use storage::StorageConfig;

pub mod state_tracker;
pub mod scraper_scheduler;
pub mod search_scraper;
pub mod item_scraper;
pub mod item_analysis;
pub mod image_classifier;
pub mod storage;

/// Holds all types of configs for the app.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub axum_config: AxumConfig,
    pub state_tracker_config: StateTrackerConfig,
    pub scraper_scheduler_config: ScraperSchedulerConfig,
    pub search_scraper_config: SearchScraperConfig,
    pub item_scraper_config: ItemScraperConfig,
    pub item_analysis_config: ItemAnalysisConfig,
    pub img_classifier_config: ItemEmbedderConfig,
    pub storage_config: StorageConfig
}

impl AppConfig {
    /// Load all configs from a .env file. Returns a `VarError` if any are missing.
    pub fn load() -> Result<Self, VarError> {
        Ok(
            AppConfig {
                axum_config: AxumConfig::load()?,
                state_tracker_config: StateTrackerConfig::load()?,
                scraper_scheduler_config: ScraperSchedulerConfig::load()?,
                search_scraper_config: SearchScraperConfig::load()?,
                item_scraper_config: ItemScraperConfig::load()?,
                item_analysis_config: ItemAnalysisConfig::load()?,
                img_classifier_config: ItemEmbedderConfig::load()?,
                storage_config: StorageConfig::load()?,
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











