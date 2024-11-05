use axum::Router;
use scraper_scheduler::ScraperSchedulerModule;

use crate::config::AppConfig;

pub mod web_backend;
pub mod scraper_scheduler;
pub mod scraper;
pub mod image_analysis;
pub mod image_classifier;
pub mod storage;

/// Holds all the modules for the monolith.
pub struct AppModules {
    scheduler_module: ScraperSchedulerModule
}

impl AppModules {
}