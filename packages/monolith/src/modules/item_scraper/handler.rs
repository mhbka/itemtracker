use crate::{config::ItemScraperConfig, messages::{ItemAnalysisSender, StateTrackerSender}};
use super::scrapers::ItemScraper;

/// Coordinates the internal workings of the module.
pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender,
    img_analysis_sender: ItemAnalysisSender,
    scraper: ItemScraper
}

impl Handler {
    /// Initialize the handler.
    pub fn init(
        state_tracker_sender: StateTrackerSender,
        img_analysis_sender: ItemAnalysisSender,
        config: &ItemScraperConfig
    ) -> Self {
        Self {
            state_tracker_sender,
            img_analysis_sender,
            scraper: ItemScraper::new(config)
        }
    }
    
    /// Scrape items for a gallery.
    pub async fn scrape_items(&mut self) {

    }
}

