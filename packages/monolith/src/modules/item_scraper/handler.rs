use crate::{config::ItemScraperConfig, galleries::{domain_types::GalleryId, pipeline_states::{GalleryItemScrapingState, GalleryPipelineStates}}, messages::{message_types::{item_scraper::ItemScraperError, state_tracker::{StateTrackerMessage, TakeGalleryStateMessage}}, ItemAnalysisSender, StateTrackerSender}};
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

    /// Scrape items for a gallery in state.
    pub async fn scrape_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), ItemScraperError> { 

    }
    
    /// Scrape items for a gallery.
    pub async fn scrape_new_gallery(&mut self, gallery: GalleryItemScrapingState) -> Result<(), ItemScraperError> {
        
    }

    /// Fetches a gallery from state.
    /// 
    /// Returns an `Err` if:
    /// - the gallery is not in state, 
    /// - the gallery is not in the expected state, 
    /// - the state has been taken,
    /// - the state tracker is not contactable
    async fn fetch_gallery_state(&mut self, gallery_id: GalleryId) -> Result<GalleryItemScrapingState, ItemScraperError> {
        let (state_msg, receiver) = TakeGalleryStateMessage::new(gallery_id.clone());
        self.state_tracker_sender
            .send(StateTrackerMessage::TakeGalleryState(state_msg))
            .await;
        let state = receiver.await
            .map_err(|err| 
                ItemScraperError::Other { gallery_id: gallery_id.clone(), message: format!("Could not receive response from state tracker: {err}") }
            )?
            .map_err(|_| 
                ItemScraperError::Other { gallery_id: gallery_id.clone(), message: "Gallery's state doesn't exist, or was already taken (this should not happen)".into() }
            )?;
        match state {
            GalleryPipelineStates::ItemScraping(gallery_state) => Ok(gallery_state),
            _ => Err(
                ItemScraperError::Other { gallery_id: gallery_id.clone(), message: "Gallery is not in expected state".into() }
                )
        }
    }
}

