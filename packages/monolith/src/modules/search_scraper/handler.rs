use std::collections::HashMap;
use crate::{
    config::SearchScraperConfig, 
    galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, 
    pipeline_states::{GalleryItemScrapingState, GalleryPipelineStateTypes, GalleryPipelineStates, GallerySearchScrapingState}}, 
    messages::{
        message_types::{item_scraper::ItemScraperMessage, search_scraper::SearchScraperError, 
            state_tracker::{
                CheckGalleryMessage, RemoveGalleryMessage, StateTrackerMessage, TakeGalleryStateMessage, UpdateGalleryStateMessage
            }
        }, 
        ItemScraperSender, 
        StateTrackerSender
    }
};

use super::scrapers::SearchScraper;

/// Coordinates the internal workings of the module.
pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender,
    item_scraper_sender: ItemScraperSender,
    search_scraper: SearchScraper
}

impl Handler {
    /// Instantiate the state.
    pub fn new(
        config: &SearchScraperConfig,
        state_tracker_sender: StateTrackerSender,
        item_scraper_sender: ItemScraperSender
    ) -> Self {
        let search_scraper = SearchScraper::new(config);
        Self {
            state_tracker_sender,
            item_scraper_sender,
            search_scraper
        }
    }

    /// Perform the scraping of a gallery in state.
    pub async fn scrape_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), SearchScraperError> {
        let gallery = self.fetch_gallery_state(gallery_id).await?;
        self.scrape_gallery(gallery).await
    }

    /// Perform the entire scraping of a new gallery.
    pub async fn scrape_new_gallery(&mut self, gallery: GallerySearchScrapingState) -> Result<(), SearchScraperError> {
        match self.check_gallery_in_state(gallery.gallery_id.clone()).await? {
            true => Err(
                SearchScraperError::GalleryAlreadyExists { gallery_id: gallery.gallery_id }
            ),
            false => self.scrape_gallery(gallery).await,
        }
    }

    /// Scrapes the search for a gallery and sends it to the item scraper.
    async fn scrape_gallery(&mut self, gallery: GallerySearchScrapingState) -> Result<(), SearchScraperError> {
        let scraped_search_result = self.search_scraper
            .scrape_search(&gallery)
            .await;
        let gallery_id = gallery.gallery_id.clone();
        self.update_gallery_state(
            gallery,
            scraped_search_result.clone()
        ).await?;
        self.item_scraper_sender
            .send(ItemScraperMessage::ScrapeItems { gallery_id })
            .await;
            Ok(())
    }
    
    /// Checks if the gallery is in any state.
    /// 
    /// Returns an `Err` if the state tracker is not contactable.
    async fn check_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<bool, SearchScraperError> {
        let (state_msg, receiver) = CheckGalleryMessage::new(gallery_id.clone());
        self.state_tracker_sender
            .send(StateTrackerMessage::CheckGallery(state_msg))
            .await;
        let in_state = receiver.await
            .map_err(|err| 
                SearchScraperError::Other { gallery_id: gallery_id.clone(), message: format!("Could not receive response from state tracker: {err}") }
            )?;
        Ok(in_state)
    }

    /// Fetches a gallery from state.
    /// 
    /// Returns an `Err` if:
    /// - the gallery is not in state/is in the wrong state/has already been taken 
    /// - the state tracker is not contactable
    async fn fetch_gallery_state(&mut self, gallery_id: GalleryId) -> Result<GallerySearchScrapingState, SearchScraperError> {
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
                SearchScraperError::Other { gallery_id: gallery_id.clone(), message: format!("Could not receive response from state tracker: {err}") }
            )?
            .map_err(|_| 
                SearchScraperError::Other { gallery_id: gallery_id.clone(), message: "Gallery's state doesn't exist, doesn't match requested state type, or was already taken".into() }
            )?;
        match state {
            GalleryPipelineStates::SearchScraping(gallery_state) => Ok(gallery_state),
            _ => Err(
                    SearchScraperError::Other { gallery_id: gallery_id.clone(), message: "Gallery is not in expected state".into() }
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
        cur_state: GallerySearchScrapingState, 
        scraped_search_result: HashMap<Marketplace, Result<Vec<ItemId>, String>>
    ) -> Result<(), SearchScraperError> {
        let gallery_id = cur_state.gallery_id.clone();
        match scraped_search_result
            .iter()
            .all(|(_, result)| result.is_err())
            {
                true => { // if all marketplaces returned an Err, remove gallery from state and return an Err
                    let (state_msg, _) = RemoveGalleryMessage::new(gallery_id.clone());
                    self.state_tracker_sender
                        .send(StateTrackerMessage::RemoveGallery(state_msg))
                        .await;
                    Err(SearchScraperError::TotalScrapeFailure { gallery_id })
                },
                false => {
                    let new_state = self.process_to_next_state(scraped_search_result, cur_state);
                    let (state_msg, receiver) = UpdateGalleryStateMessage::new(
                        (
                            gallery_id.clone(), 
                            GalleryPipelineStates::ItemScraping(new_state)
                        )
                    );
                    self.state_tracker_sender
                        .send(StateTrackerMessage::UpdateGalleryState(state_msg))
                        .await;
                    let x = receiver.await
                        .map_err(|err| 
                            SearchScraperError::Other { gallery_id: gallery_id.clone(), message: format!("Could not receive response from state tracker: {err}") }
                        )
                        .map_err(|err| 
                            SearchScraperError::Other { gallery_id, message: "Could not update gallery state".into() }
                        )?;
                    Ok(())
                }
            }
    }

    /// Process the gallery's state into the next state.
    fn process_to_next_state(
        &self,
        scraped_search_result: HashMap<Marketplace, Result<Vec<ItemId>, String>>,
        gallery_state: GallerySearchScrapingState,
    ) -> GalleryItemScrapingState {
        let cur_datetime = UnixUtcDateTime::now();
        let marketplace_updated_datetimes = scraped_search_result
            .iter()
            .filter(|(_, result)| result.is_ok())
            .map(|(marketplace, _)| (marketplace.clone(), cur_datetime.clone()))
            .collect();
        let failed_marketplace_reasons = scraped_search_result
            .iter()
            .map(|(m, r)| (m.clone(), r.clone()))
            .filter_map(|(marketplace, result)| result.err().map(|err| (marketplace, err)))
            .collect();
        let valid_scraped_search_ids = scraped_search_result
            .into_iter()
            .filter_map(|(marketplace, result)| result.ok().map(|ids| (marketplace, ids)))
            .collect();
        GalleryItemScrapingState {
            gallery_id: gallery_state.gallery_id,
            item_ids: valid_scraped_search_ids,
            failed_marketplace_reasons,
            marketplace_updated_datetimes,
            evaluation_criteria: gallery_state.evaluation_criteria
        }
    }
}
