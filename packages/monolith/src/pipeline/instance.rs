use tokio::sync::mpsc::Sender;

use crate::{config::AppConfig, domain::pipeline_states::GallerySearchScrapingState, stores::{galleries::GalleryStore, AppStores}};
use super::{error::PipelineError, item_analysis::ItemAnalyzer, item_embedder::ItemEmbedder, item_scraper::ItemScraper, scheduler::messages::SchedulerMessage, search_scraper::SearchScraper, storage::Storage};

/// An instance of a running pipeline.
/// 
/// One is created/cloned each time a task runs.
#[derive(Clone)]
pub struct PipelineInstance {
    search_scraper: SearchScraper,
    item_scraper: ItemScraper,
    item_analyzer: ItemAnalyzer,
    item_embedder: ItemEmbedder,
    storage: Storage,
    gallery_store: GalleryStore,
    scheduler_sender: Sender<SchedulerMessage>
}

impl PipelineInstance {
    /// Initialize an instance.
    pub fn new(
        config: &AppConfig, 
        app_store: &AppStores,
        scheduler_sender: Sender<SchedulerMessage>
    ) -> Self {
        let search_scraper = SearchScraper::new(&config.search_scraper);
        let item_scraper = ItemScraper::new(&config.item_scraper);
        let item_analyzer = ItemAnalyzer::new(&config.item_analysis);
        let item_embedder = ItemEmbedder::new(&config.item_embedder);
        let storage = Storage::new(app_store.gallery_sessions_store.clone());
        Self {
            search_scraper,
            item_scraper,
            item_analyzer,
            item_embedder,
            storage,
            gallery_store: app_store.gallery_store.clone(),
            scheduler_sender
        }
    }

    /// Run the pipeline on a gallery, 
    /// returning `Ok` if the pipeline completed and stored its items successfully.
    /// 
    /// Returns an `Err` if the pipeline failed at any point.
    pub async fn run_pipeline(&mut self, state: GallerySearchScrapingState) -> Result<(), PipelineError> {
        let gallery_id = state.gallery_id;

        let state = self.search_scraper.scrape(state).await?;
        let state = self.item_scraper.scrape(state).await?;
        let state = self.item_analyzer.analyze(state).await?;
        let state = self.item_embedder.embed(state).await?;
        let new_session_id = self.storage.store(state).await?;

        // update the gallery in the scheduler
        let updated_gallery = self.gallery_store
            .get_gallery(*gallery_id)
            .await
            .map_err(|err| PipelineError::FailedToUpdate { reason: err.to_string() })?;
        let (msg, receiver) = SchedulerMessage::update_gallery(updated_gallery.to_scheduler_state());
        self.scheduler_sender
            .send(msg)
            .await
            .map_err(|err| PipelineError::FailedToUpdate { reason: err.to_string() })?;
        receiver
            .await
            .map_err(|err| PipelineError::FailedToUpdate { reason: err.to_string() })?
            .map_err(|err| PipelineError::FailedToUpdate { reason: err.to_string() })?;

        Ok(())
    }
}