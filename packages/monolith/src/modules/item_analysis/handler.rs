use std::collections::HashMap;
use crate::{
    config::ItemAnalysisConfig, 
    galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, items::pipeline_items::MarketplaceAnalyzedItems, pipeline_states::{GalleryClassifierState, GalleryItemAnalysisState, GalleryPipelineStateTypes, GalleryPipelineStates}}, 
    messages::{
        message_types::{img_classifier::ImageClassifierMessage, item_analysis::ItemAnalysisError, item_scraper::ItemScraperMessage
        }, ImageClassifierSender, StateTrackerSender
    }
};

use super::analyzer::Analyzer;

/// Coordinates the internal workings of the module.
pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender,
    image_classifier_sender: ImageClassifierSender,
    analyzer: Analyzer
}

impl Handler {
    /// Instantiate the state.
    pub fn new(
        config: &ItemAnalysisConfig,
        state_tracker_sender: StateTrackerSender,
        image_classifier_sender: ImageClassifierSender
    ) -> Self {
        let analyzer = Analyzer::new(config.clone());
        Self {
            state_tracker_sender,
            image_classifier_sender,
            analyzer
        }
    }

    /// Perform the scraping of a gallery in state.
    pub async fn analyze_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), ItemAnalysisError> {
        let gallery = self.fetch_gallery_state(gallery_id).await?;
        self.analyze_gallery(gallery).await
    }

    /// Perform the entire scraping of a new gallery.
    pub async fn analyze_new_gallery(&mut self, gallery: GalleryItemAnalysisState) -> Result<(), ItemAnalysisError> {
        self.check_gallery_doesnt_exist(gallery.gallery_id.clone()).await?;
        self.analyze_gallery(gallery).await
    }

    /// Scrapes the search for a gallery and sends it to the item scraper.
    async fn analyze_gallery(&mut self, gallery: GalleryItemAnalysisState) -> Result<(), ItemAnalysisError> {
        let analyzed_items = self.analyzer
            .analyze_gallery(gallery.items, &gallery.evaluation_criteria)
            .await;
        let gallery_id = gallery.gallery_id.clone();
        self.update_gallery_state(
            gallery.gallery_id,
            analyzed_items,
            gallery.marketplace_updated_datetimes,
            gallery.failed_marketplace_reasons,
        ).await?;
        self.image_classifier_sender
            .send(ImageClassifierMessage::Classify { gallery_id })
            .await;
            Ok(())
    }
    
    /// Ensure the gallery doesn't exist.
    /// 
    /// Returns an `Err` if it exists, or the state tracker is not contactable.
    async fn check_gallery_doesnt_exist(&mut self, gallery_id: GalleryId) -> Result<(), ItemAnalysisError> {
        self.state_tracker_sender
            .check_gallery_doesnt_exist(gallery_id.clone())
            .await
            .map_err(|err| ItemAnalysisError::Other { 
                gallery_id: gallery_id.clone(), 
                message: format!("Could not receive response from state tracker: {err}") 
            })?
            .map_err(|err| ItemAnalysisError::StateErr { 
                gallery_id, 
                err 
            })
    }

    /// Fetches a gallery from state.
    /// 
    /// Returns an `Err` if:
    /// - the gallery is not in state/is in the wrong state/has already been taken 
    /// - the state tracker is not contactable
    async fn fetch_gallery_state(&mut self, gallery_id: GalleryId) -> Result<GalleryItemAnalysisState, ItemAnalysisError> {
        let state = self.state_tracker_sender
            .take_gallery_state(gallery_id.clone(), GalleryPipelineStateTypes::SearchScraping)
            .await
            .map_err(|err| ItemAnalysisError::Other { 
                gallery_id: gallery_id.clone(),
                message: format!("Could not receive response from state tracker: {err}") 
            })?
            .map_err(|err| ItemAnalysisError::StateErr { 
                gallery_id: gallery_id.clone(), 
                err
            })?;
        match state {
            GalleryPipelineStates::ItemAnalysis(gallery_state) => Ok(gallery_state),
            _ => Err(
                    ItemAnalysisError::Other { 
                        gallery_id: gallery_id.clone(), 
                        message: "Gallery is not in expected state".into()
                    }
                )
        }
    }

    /// Updates the state for a search-scraped gallery.
    /// 
    /// Returns an `Err` if:
    /// - all marketplaces failed to scrape (also removing the gallery from state),
    /// - the gallery is not in state/is in the wrong state/has already been taken,
    /// - the state tracker module couldn't be contacted.
    async fn update_gallery_state(
        &mut self, 
        gallery_id: GalleryId,
        analyzed_items: HashMap<Marketplace, MarketplaceAnalyzedItems>,
        marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
        failed_marketplace_reasons: HashMap<Marketplace, String>,
    ) -> Result<(), ItemAnalysisError> {
        let new_state = self.process_to_next_state(
            gallery_id.clone(), 
            analyzed_items, 
            marketplace_updated_datetimes, 
            failed_marketplace_reasons
        );
        self.state_tracker_sender
            .update_gallery_state(
                gallery_id.clone(), 
                GalleryPipelineStates::Classification(new_state)
            )
            .await
            .map_err(|err| ItemAnalysisError::Other { 
                gallery_id: gallery_id.clone(), 
                message: format!("Got an error messaging the state tracker: {err}")
            })?
            .map_err(|err| ItemAnalysisError::StateErr { 
                gallery_id, 
                err 
            })
    }

    /// Process the gallery's state into the next state.
    fn process_to_next_state(
        &self,
        gallery_id: GalleryId,
        analyzed_items: HashMap<Marketplace, MarketplaceAnalyzedItems>,
        marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
        failed_marketplace_reasons: HashMap<Marketplace, String>,
    ) -> GalleryClassifierState {
        GalleryClassifierState {
            gallery_id,
            items: analyzed_items,
            marketplace_updated_datetimes,
            failed_marketplace_reasons,
        }
    }
}
