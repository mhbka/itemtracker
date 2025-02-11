use std::{collections::{HashMap, HashSet}, sync::Arc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::{config::ScraperConfig, galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, eval_criteria::EvaluationCriteria, items::item_data::MarketplaceItemData, pipeline_states::GalleryScrapingState}, messages::{message_types::{scraper::{IngestScrapedItems, IngestScrapedSearch, ScraperError}, state_tracker::{AddNewGalleryMessage, PutGalleryStateMessage, RemoveGalleryMessage, StateTrackerMessage, TakeGalleryStateMessage}}, ItemAnalysisSender, MarketplaceItemsStorageSender, StateTrackerSender}, modules::state_tracker::gallery_state::GalleryState};

use super::{item_scraper::ItemScraper, output_processor::OutputProcessor, search_scraper::SearchScraper};

/// In charge of tracking and managing the state of scraped galleries.
/// 
/// It has a few primary functions:
/// - To ensure that a gallery only has 1 scraping job (and reject requests for galleries that are currently in-state),
/// - To track the status of each marketplace's scraping within each gallery,
/// - To trigger the sending of a gallery's scraped data, when all its marketplaces are scraped (and remove it from the state)
/// 
/// All messages that "do something" related to scraping come through here.
pub struct StateManager {
    state_tracker_msg_sender: StateTrackerSender,
    search_scraper: SearchScraper,
    item_scraper: ItemScraper,
    output_processor: OutputProcessor
}

impl StateManager {
    /// Instantiate the state.
    /// 
    /// TODO: Be able to instantiate the state from (and persist to) a DB or something
    pub fn new(
        config: &ScraperConfig,
        state_tracker_msg_sender: StateTrackerSender,
        item_storage_msg_sender: MarketplaceItemsStorageSender,
        img_analysis_msg_sender: ItemAnalysisSender
    ) -> Self {
        let search_scraper = SearchScraper::new(config);
        let item_scraper = ItemScraper::new(config);
        let output_processor = OutputProcessor::new(
            item_storage_msg_sender,
            img_analysis_msg_sender.clone(), 
        );
        Self {
            state_tracker_msg_sender,
            search_scraper,
            item_scraper,
            output_processor
        }
    }

    /// Perform the entire scraping of a gallery.
    pub async fn scrape_gallery(&mut self, gallery: GalleryScrapingState) -> Result<(), ScraperError> {
        self.register_new_gallery_state(&gallery).await?;
        tracing::trace!("Gallery {} successfully registered in state; starting scrape...", gallery.gallery_id);

        let scraped_search_result = self.search_scraper
            .scrape_search(&gallery)
            .await;
        self.update_search_scraped_gallery_state(gallery.gallery_id.clone(), scraped_search_result.clone()).await?;

        let valid_scraped_search_ids = scraped_search_result
            .into_iter()
            .filter_map(|(marketplace, result)| result.ok().map(|ids| (marketplace, ids)))
            .collect();
        let scraped_items_result = self.item_scraper
            .scrape_items(valid_scraped_search_ids)
            .await;
        self.update_item_scraped_gallery_state(gallery.gallery_id.clone(), scraped_items_result).await?; // TODO: work on this

        // TODO: consolidate items and send to output

        Ok(())
    }

    /// Attempt to register a requested gallery in the state tracker module.
    async fn register_new_gallery_state(&mut self, gallery: &GalleryScrapingState) -> Result<(), ScraperError> {
        let (state_msg, receiver) = AddNewGalleryMessage::new(gallery.gallery_id.clone());
        self.state_tracker_msg_sender
            .send(StateTrackerMessage::AddNewGallery(state_msg))
            .await;
        match receiver.await {
            Err(err) => return Err(
                ScraperError::Other { 
                    gallery_id: gallery.gallery_id.clone(), 
                    message: format!("Could not receive response from state tracker: {err}") 
                }),
            Ok(res) => match res {
                Err(()) => return Err(
                    ScraperError::GalleryAlreadyExists { gallery_id: gallery.gallery_id.clone() }
                ),
                Ok(()) => Ok(())
            }
        }
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
    ) -> Result<(), ScraperError> {
        let (state_msg, receiver) = TakeGalleryStateMessage::new(gallery_id.clone());
        self.state_tracker_msg_sender
            .send(StateTrackerMessage::TakeGalleryState(state_msg))
            .await;
        match receiver.await {
            Err(err) => return Err(
                ScraperError::Other { gallery_id, message: format!("Could not receive response from state tracker: {err}") }
            ),
            Ok(res) => match res {
                Err(()) => return Err(
                    ScraperError::Other { gallery_id, message: "Gallery's state doesn't exist, or was already taken (this should not happen)".into() }
                ),
                Ok(state) => {
                    match state {
                        GalleryState::SearchScraping { 
                            scraped_item_ids,
                            mut updated_up_to, 
                            mut failed_marketplace_reasons, 
                            eval_criteria 
                        } => {
                            match scraped_search_result
                                .iter()
                                .all(|(_, result)| result.is_err())
                                {
                                    true => {
                                        let (state_msg, _) = RemoveGalleryMessage::new(gallery_id.clone());
                                        self.state_tracker_msg_sender
                                            .send(StateTrackerMessage::RemoveGallery(state_msg))
                                            .await;
                                        return Err(ScraperError::TotalSearchScrapeFailure { gallery_id });
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
                                        self.state_tracker_msg_sender
                                            .send(StateTrackerMessage::PutGalleryState(state_msg))
                                            .await;
                                        Ok(())
                                    }
                                }
                        },
                        _ => { 
                            return Err(
                                ScraperError::Other { gallery_id, message: "Gallery's state is not SearchScraping".into() }
                            );
                        },
                    }
                }
            }
        }
    }

    /// Update the state for an item-scraped gallery.
    /// 
    /// Returns an `Err` if:
    /// - all marketplaces failed to scrape (also removing the gallery from state)
    /// - the gallery's state is wrong/doesn't exist/was already taken
    /// - the state tracker module couldn't be contacted.
    async fn update_item_scraped_gallery_state(
        &mut self,
        gallery_id: GalleryId,
        scraped_items_result: HashMap<Marketplace, Vec<Result<MarketplaceItemData, String>>>
    ) -> Result<(), ScraperError> {
        todo!()
    }
}
