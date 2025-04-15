use crate::{messages::{ScraperSchedulerSender, SearchScraperSender}, stores::AppStores};

/// The application state.
#[derive(Clone)]
pub struct AppState {
    /// For accessing storage of app data.
    pub stores: AppStores,
    /// For talking to the scraping pipeline scheduler
    /// (ie, changing the data of a scheduled gallery).
    pub scheduler_sender: ScraperSchedulerSender,
    /// For talking to the search scraper
    /// (ie, starting the scrape of a gallery on the spot).
    pub search_scraper_sender: SearchScraperSender
}