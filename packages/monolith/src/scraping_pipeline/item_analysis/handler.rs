use std::collections::HashMap;
use crate::{
    config::ItemAnalysisConfig, 
    domain::{domain_types::{GalleryId, Marketplace, UnixUtcDateTime}, eval_criteria::EvaluationCriteria, pipeline_items::MarketplaceAnalyzedItems, pipeline_states::{GalleryItemAnalysisState, GalleryItemEmbedderState, GalleryPipelineStateTypes, GalleryPipelineStates}}, 
    messages::{
        message_types::{item_analysis::ItemAnalysisError, item_embedder::ItemEmbedderMessage
        }, ItemEmbedderSender, StateTrackerSender
    }
};

use super::analyzer::Analyzer;

/// Coordinates the internal workings of the module.
pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender,
    item_embedder_sender: ItemEmbedderSender,
    analyzer: Analyzer
}

impl Handler {
    /// Instantiate the state.
    pub fn new(
        config: &ItemAnalysisConfig,
        state_tracker_sender: StateTrackerSender,
        item_embedder_sender: ItemEmbedderSender
    ) -> Self {
        let analyzer = Analyzer::new(config.clone());
        Self {
            state_tracker_sender,
            item_embedder_sender,
            analyzer
        }
    }
    
    /// Perform the entire scraping of a new gallery.
    pub async fn analyze_new_gallery(&mut self, gallery: GalleryItemAnalysisState) -> Result<(), ItemAnalysisError> {
        let gallery_id = gallery.gallery_id.clone();
        self.add_gallery_to_state(gallery_id.clone(), gallery).await?;
        self.analyze_gallery_in_state(gallery_id).await
    }

    /// Perform the scraping of a gallery in state.
    pub async fn analyze_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), ItemAnalysisError> {
        let gallery = self.fetch_gallery_state(gallery_id).await?;
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
            gallery.evaluation_criteria
        ).await?;
        self.item_embedder_sender
            .send(ItemEmbedderMessage::Classify { gallery_id: gallery_id.clone() })
            .await
            .map_err(|err| ItemAnalysisError::MessageErr { gallery_id, err })?;
            Ok(())
    }
    
    /// Add a new gallery to the state.
    /// 
    /// Returns an `Err` if it already exists.
    async fn add_gallery_to_state(
        &mut self, 
        gallery_id: GalleryId, 
        gallery: GalleryItemAnalysisState
    ) -> Result<(), ItemAnalysisError> {
        self.state_tracker_sender
            .add_gallery(gallery_id.clone(), GalleryPipelineStates::ItemAnalysis(gallery))
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
            .get_gallery_state(gallery_id.clone(), GalleryPipelineStateTypes::ItemAnalysis)
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
        evaluation_criteria: EvaluationCriteria
    ) -> Result<(), ItemAnalysisError> {
        let new_state = self.process_to_next_state(
            gallery_id.clone(), 
            analyzed_items, 
            marketplace_updated_datetimes, 
            failed_marketplace_reasons,
            evaluation_criteria
        );
        self.state_tracker_sender
            .update_gallery_state(
                gallery_id.clone(), 
                GalleryPipelineStates::ItemEmbedding(new_state)
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
        evaluation_criteria: EvaluationCriteria
    ) -> GalleryItemEmbedderState {
        GalleryItemEmbedderState {
            gallery_id,
            items: analyzed_items,
            marketplace_updated_datetimes,
            failed_marketplace_reasons,
            used_evaluation_criteria: evaluation_criteria
        }
    }
}
