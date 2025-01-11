use std::{collections::{HashMap, HashSet}, sync::Arc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::{config::ScraperConfig, galleries::{domain_types::{GalleryId, Marketplace, UnixUtcDateTime}, eval_criteria::EvaluationCriteria, scraping_pipeline::GalleryScrapingState}, messages::{message_types::scraper::{IngestScrapedItems, IngestScrapedSearch, ScraperError}, ItemAnalysisSender, MarketplaceItemsStorageSender}};

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
    states: Arc<Mutex<GalleryStates>>,
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
        item_storage_msg_sender: MarketplaceItemsStorageSender,
        img_analysis_msg_sender: ItemAnalysisSender
    ) -> Self {
        let states = Arc::new(Mutex::new(GalleryStates::new()));
        let search_scraper = SearchScraper::new(config);
        let item_scraper = ItemScraper::new(config);
        let output_processor = OutputProcessor::new(
            item_storage_msg_sender,
            img_analysis_msg_sender.clone(), 
        );
        Self {
            states,
            search_scraper,
            item_scraper,
            output_processor
        }
    }

    /// Handle the starting of a gallery's scrape.
    /// 
    /// Returns an `Err` if the gallery is already being scraped.
    pub async fn start_scraping_gallery(&mut self, gallery: GalleryScrapingState) -> Result<(), ScraperError> {
        let init_gallery_result = self.states
            .lock()
            .await
            .init_gallery(
                &gallery.gallery_id, 
                &gallery.marketplaces,
                &gallery.evaluation_criteria
            );
        if init_gallery_result.is_err() 
        {
            return Err(ScraperError::StartScrapingGalleryError { 
                gallery_id: gallery.gallery_id, 
                error: "Gallery already exists".into() 
            });
        }
        tracing::trace!("Scheduling scrape search for gallery: {gallery:#?}");
        self.search_scraper
            .schedule_scrape_search(&gallery, self.states.clone())
            .await;
        Ok(())
    }

    /// Handle the ingestion of scraped search data + starting of the gallery's marketplace's item scrape.
    /// 
    /// Returns an `Err` if the gallery + marketplace's status is not currently `MarketplaceStatus::SearchScrapeInProgress`.
    pub async fn ingest_scraped_search(&mut self, mut data: IngestScrapedSearch) -> Result<(), ScraperError> {
        let mut gallery_states = self.states
            .lock()
            .await;
        match gallery_states.get_status(&data.gallery_id, &data.marketplace) {
                Ok(status) => {
                    match status {
                        MarketplaceStatus::SearchScrapeInProgress(_) => {
                            gallery_states
                                .update_status(&data.gallery_id, &data.marketplace)
                                .expect("Gallery + marketplace should already exist here");
                            data.scraped_item_ids = self.output_processor
                                .fetch_cached_items(
                                    &data.gallery_id, 
                                    &data.marketplace, 
                                    &data.updated_up_to,
                                    data.scraped_item_ids, 
                                )
                                .await;
                            self.item_scraper
                                .schedule_scrape_items(data, self.states.clone())
                                .await;
                            Ok(())
                        },
                        other => {
                            return Err(ScraperError::IngestScrapedSearchError { 
                                gallery_id: data.gallery_id, 
                                marketplace: data.marketplace, 
                                error: format!("Invalid status for gallery + marketplace ({other:#?})") }
                            );
                        }
                    }
                }
                Err(_) => {
                    return Err(ScraperError::IngestScrapedSearchError { 
                        gallery_id: data.gallery_id, 
                        marketplace: data.marketplace, 
                        error: "This gallery + marketplace is not currently being scraped".into() }
                    );
                }
            }
    }

    /// Handle scraped items for a marketplace.
    /// 
    /// If all other marketplaces for the involved gallery are already item-scraped, 
    /// this also sends scraped data to the next stage,
    /// and removes the gallery from the state.
    /// 
    /// Returns an `Err` if the gallery involved is not currently being scraped.
    pub async fn ingest_scraped_items(&mut self, data: IngestScrapedItems) -> Result<(), ScraperError> {
        let mut gallery_states = self.states
            .lock()
            .await;
        match gallery_states.get_status(&data.gallery_id, &data.marketplace) {
                Ok(status) => {
                    match status {
                        MarketplaceStatus::ItemScrapeInProgress(_) => {
                            gallery_states
                                .update_status(&data.gallery_id, &data.marketplace)
                                .expect("Gallery + marketplace should already exist here");
                            self.output_processor
                                .process_scraped_items(
                                    &data.gallery_id, 
                                    &data.marketplace, 
                                    data.scraped_items
                                )
                                .await;
                            if gallery_states.is_ready_to_send(&data.gallery_id) {
                                let gallery_data = gallery_states
                                    .remove_gallery(&data.gallery_id)
                                    .expect("Gallery should definitely exist");
                                return self.output_processor
                                    .send_gallery_items(data.gallery_id, gallery_data.eval_criteria)
                                    .await;
                            }
                            Ok(())
                        },
                        other => {
                            return Err(ScraperError::IngestScrapedSearchError { 
                                gallery_id: data.gallery_id, 
                                marketplace: data.marketplace, 
                                error: format!("Invalid status for gallery + marketplace ({other:#?})") }
                            );
                        }
                    }
                }
                Err(_) => {
                    return Err(ScraperError::IngestScrapedSearchError { 
                        gallery_id: data.gallery_id, 
                        marketplace: data.marketplace, 
                        error: "This gallery + marketplace is not currently being scraped".into() }
                    );
                }
            }
    }     
}

/// The inner state of the state manager.
pub struct GalleryStates {
    states: HashMap<GalleryId, GalleryState>
}

impl GalleryStates {
    /// Initialize the state.
    fn new() -> Self {
        Self { states: HashMap::new() }
    }

    /// Initializes the gallery in the state, with all marketplaces set to `MarketplaceStatus::SearchScrapePending`.
    /// 
    /// Returns an `Err` if the gallery already exists.
    fn init_gallery(
        &mut self, 
        gallery: &GalleryId, 
        marketplaces: &HashSet<Marketplace>,
        eval_criteria: &EvaluationCriteria
    ) -> Result<(), ()> {
        if self.has_gallery(gallery) {
            return Err(());
        }
        let cur_datetime = UnixUtcDateTime::now();
        let marketplace_statuses: HashMap<_, _> = marketplaces
            .into_iter()
            .map(|marketplace| (marketplace.clone(), MarketplaceStatus::SearchScrapePending(cur_datetime.clone())))
            .collect();
        let gallery_state = GalleryState {
            marketplaces: marketplace_statuses,
            data: TempGalleryData { eval_criteria: eval_criteria.clone() }
        };
        self.states.insert(gallery.clone(), gallery_state);
        Ok(())
    }

    /// Return if the gallery exists in the state.
    fn has_gallery(&self, gallery: &GalleryId) -> bool {
        self.states.contains_key(gallery)
    }

    /// Returns the status of a marketplace within a gallery.
    /// 
    /// Returns an `Err` if the gallery + marketplace was not found.
    fn get_status(&self, gallery: &GalleryId, marketplace: &Marketplace) -> Result<&MarketplaceStatus, ()> {
        if let Some(gallery) = self.states.get(gallery) {
            return gallery.marketplaces
                .get(marketplace)
                .ok_or(());
        }
        Err(())
    }

    /// Returns whether all the gallery's marketplaces are `FullyScraped`, ie the gallery is ready to be sent to the next module.
    /// 
    /// Returns `false` if the gallery doesn't exist.
    fn is_ready_to_send(&self, gallery: &GalleryId) -> bool {
        if let Some(state) = self.states.get(gallery) {
            return state.marketplaces
                .values()
                .all(|status| matches!(status, MarketplaceStatus::FullyScraped(_)));
        }
        false
    }

    /// Moves forward the status of a marketplace within a gallery, using the current time as the transition time.
    /// 
    /// The correct event to trigger each update is:
    /// - `SearchScrapePending -> SearchScrapeInProgress`: Search scrape was successfully requested for this gallery's marketplace
    /// - `SearchScrapeInProgress -> ItemScrapePending`: Search scrape data has arrived for this gallery's marketplace
    /// - `ItemScrapePending -> ItemScrapeInProgress`: Item scrape was successfully requested for this gallery's marketplace
    /// - `ItemScrapeInProgress -> FullyScraped`: Item scrape data has arrived for this gallery's marketplace
    /// 
    /// Note that the last update deletes the gallery from state.
    /// 
    /// Returns an `Err` if the gallery + marketplace was not found, or the marketplace's status is already `FullyScraped`
    /// (in which case you should call `remove_gallery()`).
    /// 
    /// **NOTE**: Be careful in ensuring the action representing the status change actually occurs when calling this.
    pub(super) fn update_status(&mut self, gallery: &GalleryId, marketplace: &Marketplace) -> Result<(), ()> {
        let state = match self.states.get_mut(&gallery) {
            Some(state) => state,
            None => {
                tracing::error!("Gallery {gallery} is not being scraped, so its status cannot be updated");
                return Err(());
            }
        };
        let marketplace_status = match state.marketplaces.get_mut(&marketplace) {
            Some(status) => status,
            None => {
                tracing::error!("Gallery {gallery} does not contain {marketplace}, so its status cannot be updated");
                return Err(());
            }
        };
        let cur_datetime = UnixUtcDateTime::now();
        match marketplace_status {
            MarketplaceStatus::SearchScrapePending(_) => {
                *marketplace_status = MarketplaceStatus::SearchScrapeInProgress(cur_datetime);
            },
            MarketplaceStatus::SearchScrapeInProgress(_) => {
                *marketplace_status = MarketplaceStatus::ItemScrapePending(cur_datetime);
            },
            MarketplaceStatus::ItemScrapePending(_) => {
                *marketplace_status = MarketplaceStatus::ItemScrapeInProgress(cur_datetime);
            },
            MarketplaceStatus::ItemScrapeInProgress(_) => {
                *marketplace_status = MarketplaceStatus::FullyScraped(cur_datetime);
            },
            MarketplaceStatus::FullyScraped(_) => {
                return Err(());
            },
        }
        Ok(())
    }

    /// Removes a gallery from the state and returns its temporary data. 
    /// 
    /// Should only be called if all the gallery's marketplaces are fully scraped,
    /// or you purposely want to remove the gallery from state (ie, the scraping has stalled/errored).
    /// 
    /// Returns an `Err` if the given gallery is not found.
    /// 
    /// **NOTE**: Be careful to only call this once the above action has actually occurred.
    pub(super) fn remove_gallery(&mut self, gallery: &GalleryId) -> Result<TempGalleryData, ()> {
        match self.states.remove(gallery) {
            Some(state) => Ok(state.data),
            None => {
                tracing::error!("Gallery {gallery} is not being scraped, so it cannot be removed from state");
                Err(())
            }
        }
    }
}

/// Represents the state of a gallery.
/// 
/// Most importantly tracks the status of each of its marketplaces, but also holds other temporary data,
/// such as the gallery's evaluation criteria.
struct GalleryState {
    marketplaces: HashMap<Marketplace, MarketplaceStatus>,
    data: TempGalleryData
}

/// Any temporary data for a gallery that needs to be stored while it's here.
struct TempGalleryData {
    eval_criteria: EvaluationCriteria
}

/// These are the possible statuses for a marketplace.
/// 
/// Each status also holds the datetime when it was transitioned to. 
/// 
/// This is useful for things like re-triggering a scrape if the marketplace has been "in progress" for too long
/// (though this is not currently a thing).
#[derive(Serialize, Deserialize, Clone, Debug)]
enum MarketplaceStatus {
    SearchScrapePending(UnixUtcDateTime),
    SearchScrapeInProgress(UnixUtcDateTime),
    ItemScrapePending(UnixUtcDateTime),
    ItemScrapeInProgress(UnixUtcDateTime),
    FullyScraped(UnixUtcDateTime)
}