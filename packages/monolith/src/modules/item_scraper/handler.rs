use std::collections::HashMap;
use crate::{
    config::ItemScraperConfig, 
    galleries::{domain_types::{GalleryId, Marketplace}, items::item_data::MarketplaceItemData, pipeline_states::{GalleryItemAnalysisState, GalleryItemScrapingState, GalleryPipelineStateTypes, GalleryPipelineStates}}, 
    messages::{message_types::{item_analysis::ItemAnalysisMessage, item_scraper::ItemScraperError}, ItemAnalysisSender, StateTrackerSender}
    };

use super::scrapers::ItemScraper;

/// Coordinates the internal workings of the module.
pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender,
    item_analysis_sender: ItemAnalysisSender,
    item_scraper: ItemScraper
}

impl Handler {
    /// Instantiate the state.
    pub fn new(
        config: &ItemScraperConfig,
        state_tracker_sender: StateTrackerSender,
        item_analysis_sender: ItemAnalysisSender
    ) -> Self {
        let item_scraper = ItemScraper::new(config);
        Self {
            state_tracker_sender,
            item_analysis_sender,
            item_scraper
        }
    }
    
    /// Perform the entire scraping of a new gallery.
    pub async fn scrape_new_gallery(&mut self, gallery: GalleryItemScrapingState) -> Result<(), ItemScraperError> {
        let gallery_id = gallery.gallery_id.clone();
        self.add_gallery_to_state(gallery_id.clone(), gallery).await?;
        self.scrape_gallery_in_state(gallery_id).await
    }

    /// Perform the scraping of a gallery in state.
    pub async fn scrape_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), ItemScraperError> {
        let gallery = self.fetch_gallery_state(gallery_id).await?;
        self.scrape_gallery(gallery).await
    }

    /// Scrapes the search for a gallery and sends it to the item scraper.
    async fn scrape_gallery(&mut self, gallery: GalleryItemScrapingState) -> Result<(), ItemScraperError> {
        let scraped_items = self.item_scraper
            .scrape_items(&gallery)
            .await;
        let gallery_id = gallery.gallery_id.clone();
        self.update_gallery_state(
            gallery,
            scraped_items.clone()
        ).await?;
        self.item_analysis_sender
            .send(ItemAnalysisMessage::AnalyzeGallery { gallery_id: gallery_id.clone() })
            .await
            .map_err(|err| ItemScraperError::MessageErr { gallery_id, err })?;
            Ok(())
    }
    
    /// Add a new gallery to the state.
    /// 
    /// Returns an `Err` if it already exists.
    async fn add_gallery_to_state(
        &mut self, 
        gallery_id: GalleryId, 
        gallery: GalleryItemScrapingState
    ) -> Result<(), ItemScraperError> {
        self.state_tracker_sender
            .add_gallery(gallery_id.clone(), GalleryPipelineStates::ItemScraping(gallery))
            .await
            .map_err(|err| ItemScraperError::Other { 
                gallery_id: gallery_id.clone(), 
                message: format!("Could not receive response from state tracker: {err}") 
            })?
            .map_err(|err| ItemScraperError::StateErr { 
                gallery_id, 
                err 
            })
    }

    /// Fetches a gallery from state.
    /// 
    /// Returns an `Err` if:
    /// - the gallery is not in state/is in the wrong state/has already been taken 
    /// - the state tracker is not contactable
    async fn fetch_gallery_state(&mut self, gallery_id: GalleryId) -> Result<GalleryItemScrapingState, ItemScraperError> {
        let state = self.state_tracker_sender
            .get_gallery_state(gallery_id.clone(), GalleryPipelineStateTypes::ItemScraping)
            .await
            .map_err(|err| ItemScraperError::Other { 
                gallery_id: gallery_id.clone(), 
                message: format!("Could not receive response from state tracker: {err}") 
            })?
            .map_err(|err| ItemScraperError::StateErr { 
                gallery_id: gallery_id.clone(), 
                err 
            })?;
        match state {
            GalleryPipelineStates::ItemScraping(gallery_state) => Ok(gallery_state),
            _ => Err(
                    ItemScraperError::Other { 
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
        cur_state: GalleryItemScrapingState, 
        scraped_items: HashMap<Marketplace, Vec<Result<MarketplaceItemData, String>>>
    ) -> Result<(), ItemScraperError> {
        let gallery_id = cur_state.gallery_id.clone();
        match scraped_items
            .iter()
            .all(|(_, result)| {
                result.len() > 0 && // we allow empty results, as long as they aren't all errors
                result.iter().all(|res| res.is_err())
            })
            {
                true => { // if all items are errors, remove gallery from state and return an Err
                    self.state_tracker_sender
                        .remove_gallery(gallery_id.clone())
                        .await
                        .map_err(|err| ItemScraperError::Other { 
                            gallery_id: gallery_id.clone(), 
                            message: format!("Could not receive response from state tracker: {err}") 
                        })?
                        .map_err(|err| ItemScraperError::StateErr { 
                            gallery_id: gallery_id.clone(), 
                            err
                        })?;
                    Err(ItemScraperError::TotalScrapeFailure { gallery_id })
                },
                false => {
                    let new_state = self.process_to_next_state(scraped_items, cur_state);
                    self.state_tracker_sender
                        .update_gallery_state(gallery_id.clone(), GalleryPipelineStates::ItemAnalysis(new_state))
                        .await
                        .map_err(|err| 
                            ItemScraperError::Other { gallery_id: gallery_id.clone(), message: format!("Could not receive response from state tracker: {err}") }
                        )?
                        .map_err(|err| 
                            ItemScraperError::StateErr { gallery_id, err }
                        )?;
                    Ok(())
                }
            }
    }

    /// Process the gallery's state into the next state.
    fn process_to_next_state(
        &self,
        scraped_items: HashMap<Marketplace, Vec<Result<MarketplaceItemData, String>>>,
        gallery_state: GalleryItemScrapingState,
    ) -> GalleryItemAnalysisState {
        let valid_items = scraped_items
            .into_iter()
            .map(|(marketplace, results)| {
                let valid_items = results
                    .into_iter()
                    .filter_map(|res| res.ok())
                    .collect();
                (marketplace, valid_items)
            })
            .collect();
        GalleryItemAnalysisState {
            gallery_id: gallery_state.gallery_id,
            items: valid_items,
            marketplace_updated_datetimes: gallery_state.marketplace_updated_datetimes,
            failed_marketplace_reasons: gallery_state.failed_marketplace_reasons,
            evaluation_criteria: gallery_state.evaluation_criteria,
        }
    }
}
