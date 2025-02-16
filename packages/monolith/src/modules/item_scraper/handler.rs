use std::collections::HashMap;
use crate::{
    config::ItemScraperConfig, 
    galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, items::item_data::MarketplaceItemData, pipeline_states::{GalleryItemAnalysisState, GalleryItemScrapingState, GalleryPipelineStateTypes, GalleryPipelineStates}}, 
    messages::{
        message_types::{item_analysis::ItemAnalysisMessage, item_scraper::{ItemScraperError, ItemScraperMessage}, state_tracker::{
                CheckGalleryDoesntExistMessage, RemoveGalleryMessage, StateTrackerMessage, TakeGalleryStateMessage, UpdateGalleryStateMessage
            }
        }, ItemAnalysisSender, StateTrackerSender
    }
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

    /// Perform the scraping of a gallery in state.
    pub async fn scrape_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), ItemScraperError> {
        let gallery = self.fetch_gallery_state(gallery_id).await?;
        self.scrape_gallery(gallery).await
    }

    /// Perform the entire scraping of a new gallery.
    pub async fn scrape_new_gallery(&mut self, gallery: GalleryItemScrapingState) -> Result<(), ItemScraperError> {
        self.check_gallery_doesnt_exist(gallery.gallery_id.clone()).await?;
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
            .send(ItemAnalysisMessage::AnalyzeGallery { gallery_id })
            .await;
            Ok(())
    }
    
    /// Ensure the gallery doesn't exist.
    /// 
    /// Returns an `Err` if it exists, or the state tracker is not contactable.
    async fn check_gallery_doesnt_exist(&mut self, gallery_id: GalleryId) -> Result<(), ItemScraperError> {
        let (state_msg, receiver) = CheckGalleryDoesntExistMessage::new(gallery_id.clone());
        self.state_tracker_sender
            .send(StateTrackerMessage::CheckGalleryDoesntExist(state_msg))
            .await;
        receiver.await
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
        let (state_msg, receiver) = TakeGalleryStateMessage::new(
            (
                gallery_id.clone(),
                GalleryPipelineStateTypes::SearchScraping
            )
        );
        self.state_tracker_sender
            .send(StateTrackerMessage::TakeGalleryState(state_msg))
            .await;
        let state = receiver.await
            .map_err(|err| 
                ItemScraperError::Other { gallery_id: gallery_id.clone(), message: format!("Could not receive response from state tracker: {err}") }
            )?
            .map_err(|_| 
                ItemScraperError::Other { gallery_id: gallery_id.clone(), message: "Gallery's state doesn't exist, doesn't match requested state type, or was already taken".into() }
            )?;
        match state {
            GalleryPipelineStates::ItemScraping(gallery_state) => Ok(gallery_state),
            _ => Err(
                    ItemScraperError::Other { gallery_id: gallery_id.clone(), message: "Gallery is not in expected state".into() }
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
                result
                    .iter()
                    .all(|res| res.is_err())
            })
            {
                true => { // if all items are Err, remove gallery from state and return an Err
                    let (state_msg, _) = RemoveGalleryMessage::new(gallery_id.clone());
                    self.state_tracker_sender
                        .send(StateTrackerMessage::RemoveGallery(state_msg))
                        .await;
                    Err(ItemScraperError::TotalScrapeFailure { gallery_id })
                },
                false => {
                    let new_state = self.process_to_next_state(scraped_items, cur_state);
                    let (state_msg, receiver) = UpdateGalleryStateMessage::new(
                        (
                            gallery_id.clone(), 
                            GalleryPipelineStates::ItemAnalysis(new_state)
                        )
                    );
                    self.state_tracker_sender
                        .send(StateTrackerMessage::UpdateGalleryState(state_msg))
                        .await;
                    let x = receiver.await
                        .map_err(|err| 
                            ItemScraperError::Other { gallery_id: gallery_id.clone(), message: format!("Could not receive response from state tracker: {err}") }
                        )
                        .map_err(|err| 
                            ItemScraperError::Other { gallery_id, message: "Could not update gallery state".into() }
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
