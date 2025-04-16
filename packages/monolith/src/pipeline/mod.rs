use instance::PipelineInstance;
use tokio::sync::mpsc::{channel, Sender};
use crate::{config::AppConfig, domain::{domain_types::GalleryId, pipeline_states::GallerySchedulerState}, stores::AppStores};
use {item_analysis::ItemAnalyzer, item_embedder::ItemEmbedder, item_scraper::ItemScraper, scheduler::{error::SchedulerError, messages::SchedulerMessage, Scheduler}, search_scraper::SearchScraper, storage::Storage};

pub mod scheduler;
pub mod search_scraper;
pub mod item_scraper;
pub mod item_analysis;
pub mod item_embedder;
pub mod storage;
pub mod instance;
pub mod error;

static CHANNEL_BUFFER_SIZE: usize = 10000;

/// The pipeline handling all scraping/processing tasks.
#[derive(Clone)]
pub struct Pipeline {
    scheduler_sender: Sender<SchedulerMessage>
}

impl Pipeline {
    /// Initialize the pipeline.
    pub async fn init(config: AppConfig, stores: &mut AppStores) -> Self {
        let (sender, receiver) = channel(CHANNEL_BUFFER_SIZE);

        let initial_state = stores.gallery_store
            .initial_gallery_tasks()
            .await
            .expect("Failure to fetch initial gallery tasks should stop the app");

        let pipeline_instance = PipelineInstance::new(&config, stores);

        let mut scheduler = Scheduler::init(
            config.scraper_scheduler.clone(), 
            pipeline_instance,
            initial_state,
            receiver,
        ).await;

        tokio::spawn(async move { scheduler.run().await; });

        Self {
            scheduler_sender: sender
        } 
    }

    /// Add a gallery to the scheduler.
    pub async fn add_gallery(&mut self, gallery: GallerySchedulerState) -> Result<(), SchedulerError> {
        let (msg, receiver) = SchedulerMessage::add_gallery(gallery);
        self.scheduler_sender
            .send(msg)
            .await
            .map_err(|err| SchedulerError::MessageFailure)?;
        receiver
            .await
            .map_err(|err| SchedulerError::MessageFailure)?
    }

    /// Update a gallery within the scheduler.
    pub async fn update_gallery(&mut self, updated_gallery: GallerySchedulerState) -> Result<(), SchedulerError> {
        let (msg, receiver) = SchedulerMessage::update_gallery(updated_gallery);
        self.scheduler_sender
            .send(msg)
            .await
            .map_err(|err| SchedulerError::MessageFailure)?;
        receiver
            .await
            .map_err(|err| SchedulerError::MessageFailure)?
    }

    /// Delete a gallery from the scheduler.
    pub async fn delete_gallery(&mut self, gallery_id: GalleryId) -> Result<(), SchedulerError> {
        let (msg, receiver) = SchedulerMessage::delete_gallery(gallery_id);
        self.scheduler_sender
            .send(msg)
            .await
            .map_err(|err| SchedulerError::MessageFailure)?;
        receiver
            .await
            .map_err(|err| SchedulerError::MessageFailure)?
    }
}