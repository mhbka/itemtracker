use std::collections::HashMap;
use crate::{config::SearchScraperConfig, galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, items::item_data::MarketplaceItemData, pipeline_states::{GalleryPipelineStates, GallerySearchScrapingState}}, messages::{message_types::{search_scraper::SearchScraperError, state_tracker::{AddNewGalleryMessage, PutGalleryStateMessage, RemoveGalleryMessage, StateTrackerMessage, TakeGalleryStateMessage}}, ItemAnalysisSender, ItemScraperSender, MarketplaceItemsStorageSender, StateTrackerSender}};

use super::scrapers::SearchScraper;

/// Coordinates the internal workings of the module.
pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender,
    item_scraper_msg_sender: ItemScraperSender,
    search_scraper: SearchScraper
}

impl Handler {
    /// Instantiate the state.
    pub fn new(
        config: &SearchScraperConfig,
        state_tracker_sender: StateTrackerSender,
        item_scraper_msg_sender: ItemScraperSender
    ) -> Self {
        let search_scraper = SearchScraper::new(config);
        Self {
            state_tracker_sender,
            item_scraper_msg_sender,
            search_scraper
        }
    }

    /// Perform the entire scraping of a gallery.
    pub async fn scrape_gallery(&mut self, gallery: GallerySearchScrapingState) -> Result<(), SearchScraperError> {
        let scraped_search_result = self.search_scraper
            .scrape_search(&gallery)
            .await;
        self.update_search_scraped_gallery_state(gallery.gallery_id.clone(), scraped_search_result.clone()).await?;
        let valid_scraped_search_ids = scraped_search_result
            .into_iter()
            .filter_map(|(marketplace, result)| result.ok().map(|ids| (marketplace, ids)))
            .collect();
        self.send_items(gallery.gallery_id, valid_scraped_search_ids).await;

        // TODO: move below to the item scraper

        let scraped_items_result = self.item_scraper
            .scrape_items(valid_scraped_search_ids)
            .await;
        self.update_item_scraped_gallery_state(gallery.gallery_id.clone(), scraped_items_result.clone()).await?; // TODO: work on this
        
        let valid_scraped_items = scraped_items_result
            .into_iter()
            .map(|(marketplace, results)| {
                // TODO: I'm just deleting error items here, but can consider store the IDs for re-scraping later on?
                let valid_results: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();
                (marketplace, valid_results)
            })
            .collect();
        Ok(())
    }

    /// Update the state for a search-scraped gallery.
    /// 
    /// Returns an `Err` if:
    /// - all marketplaces failed to scrape (also removing the gallery from state)
    /// - the gallery's state is wrong/doesn't exist/was already taken
    /// - the state tracker module couldn't be contacted.
    async fn update_search_scraped_gallery_state(
        &mut self, 
        gallery_id: GalleryId, 
        scraped_search_result: HashMap<Marketplace, Result<Vec<ItemId>, String>>
    ) -> Result<(), SearchScraperError> {
        let (state_msg, receiver) = TakeGalleryStateMessage::new(gallery_id.clone());
        self.state_tracker_sender
            .send(StateTrackerMessage::TakeGalleryState(state_msg))
            .await;
        let state = receiver.await
            .map_err(|err| SearchScraperError::Other { gallery_id: gallery_id.clone(), message: format!("Could not receive response from state tracker: {err}") })?
            .map_err(|_| SearchScraperError::Other { gallery_id: gallery_id.clone(), message: "Gallery's state doesn't exist, or was already taken (this should not happen)".into() })?;
        match state {
            GalleryPipelineStates::SearchScraping(mut state) => {
                match scraped_search_result
                    .iter()
                    .all(|(_, result)| result.is_err())
                    {
                        true => { // if all marketplaces returned an Err, remove gallery from state and return an Err
                            let (state_msg, _) = RemoveGalleryMessage::new(gallery_id.clone());
                            self.state_tracker_sender
                                .send(StateTrackerMessage::RemoveGallery(state_msg))
                                .await;
                            return Err(SearchScraperError::TotalSearchScrapeFailure { gallery_id });
                        },
                        false => {
                            let cur_datetime = UnixUtcDateTime::now();
                            updated_up_to = scraped_search_result
                                .iter()
                                .filter(|(_, result)| result.is_ok())
                                .map(|(marketplace, _)| (marketplace.clone(), cur_datetime.clone()))
                                .collect();
                            failed_marketplace_reasons = scraped_search_result
                                .iter()
                                .map(|(m, r)| (m.clone(), r.clone()))
                                .filter_map(|(marketplace, result)| result.err().map(|err| (marketplace, err)))
                                .collect();
                            let updated_state = GalleryState::SearchScraping { 
                                scraped_item_ids, 
                                updated_up_to, 
                                failed_marketplace_reasons, 
                                eval_criteria 
                            };
                            let (state_msg, receiver) = PutGalleryStateMessage::new((gallery_id, updated_state));
                            self.state_tracker_sender
                                .send(StateTrackerMessage::PutGalleryState(state_msg))
                                .await;
                            Ok(())
                        }
                    }
            },
            _ => { 
                return Err(
                    SearchScraperError::Other { gallery_id, message: "Gallery's state is not SearchScraping".into() }
                );
            },
        }
    }

    /// Send the items to the next stage.
    async fn send_items(
        &mut self,
        gallery_id: GalleryId,
        valid_scraped_search_ids: HashMap<Marketplace, Vec<ItemId>>
    ) {
        self.item_scraper_msg_sender
            .send()
    }
}
