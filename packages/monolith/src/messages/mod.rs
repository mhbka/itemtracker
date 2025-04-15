use message_buses::{MessageError, MessageReceiver, MessageSender};
use message_types::{
    item_analysis::ItemAnalysisMessage, item_embedder::ItemEmbedderMessage, item_scraper::ItemScraperMessage, scraper_scheduler::SchedulerMessage, search_scraper::SearchScraperMessage, state_tracker::{AddGalleryMessage, CheckGalleryDoesntExistMessage, RemoveGalleryMessage, StateTrackerError, StateTrackerMessage, GetGalleryStateMessage, UpdateGalleryStateMessage}, storage::StorageMessage
};
use crate::domain::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates}};

pub mod message_buses;
pub mod message_types;

/// Handle for sending the scraper scheduler messages.
pub type ScraperSchedulerSender = MessageSender<SchedulerMessage>;
/// Handle for the scraper scheduler to receive messages.
pub type ScraperSchedulerReceiver = MessageReceiver<SchedulerMessage>;

/// Handle for sending messages to the search scraper.
pub type SearchScraperSender = MessageSender<SearchScraperMessage>;
/// Handle for the search scraper module to receive messages.
pub type SearchScraperReceiver = MessageReceiver<SearchScraperMessage>;

/// Handle for sending messages to the item scraper.
pub type ItemScraperSender = MessageSender<ItemScraperMessage>;
/// Handle for the item scraper module to receive messages.
pub type ItemScraperReceiver = MessageReceiver<ItemScraperMessage>;

/// Handle for sending the item analysis module messages.
pub type ItemAnalysisSender = MessageSender<ItemAnalysisMessage>;
/// Handle for the item analysis module to receive messages.
pub type ItemAnalysisReceiver = MessageReceiver<ItemAnalysisMessage>;

/// Handle for sending the image classifier module messages.
pub type ItemEmbedderSender = MessageSender<ItemEmbedderMessage>;
/// Handle for the image classifier module to receive messages.
pub type ItemEmbedderReceiver = MessageReceiver<ItemEmbedderMessage>;

/// Handle for sending the marketplace items storage module messages.
pub type StorageSender = MessageSender<StorageMessage>;
/// Handle for the marketplace items storage storage module to receive messages.
pub type StorageReceiver = MessageReceiver<StorageMessage>;

/// Handle for the scraper scheduler to receive messages.
pub type StateTrackerReceiver = MessageReceiver<StateTrackerMessage>;

/// Handle for sending the scraper scheduler messages.
/// 
/// Wraps messaging with functions for ease of use.
#[derive(Clone, Debug)]
pub struct StateTrackerSender { sender: MessageSender<StateTrackerMessage> }

impl StateTrackerSender {
    /// Initialize the message sender.
    pub fn new(sender: MessageSender<StateTrackerMessage>) -> Self {
        Self { sender }
    }

    /// Add a gallery to the state.
    /// 
    /// Returns an `Err` if the gallery already exists.
    pub async fn add_gallery(
        &mut self,
        gallery_id: GalleryId, 
        state: GalleryPipelineStates
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = AddGalleryMessage::new((gallery_id, state));
        self.sender
            .send(StateTrackerMessage::AddGallery(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }

    /// Verify that a gallery doesn't exist.
    /// Useful for modules to check before processing new galleries.
    /// 
    /// Returns an `Err` if it does.
    pub async fn check_gallery_doesnt_exist(
        &mut self,
        gallery_id: GalleryId
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = CheckGalleryDoesntExistMessage::new(gallery_id);
        self.sender
            .send(StateTrackerMessage::CheckGalleryDoesntExist(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }
    
    /// Take a gallery's state, leaving it stored as `None`.
    /// 
    /// Returns an `Err` if it doesn't exist, its state is wrong, or its state is already taken.
    pub async fn get_gallery_state(
        &mut self,
        gallery_id: GalleryId,
        state_type: GalleryPipelineStateTypes
    ) -> Result<Result<GalleryPipelineStates, StateTrackerError>, MessageError> {
        let (msg, receiver) = GetGalleryStateMessage::new((gallery_id, state_type));
        self.sender
            .send(StateTrackerMessage::GetGalleryState(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }

    /// Update a gallery's state.
    /// 
    /// Returns an `Err` if it doesn't exist, its state is wrong, or its state isn't taken.
    pub async fn update_gallery_state(
        &mut self,
        gallery_id: GalleryId,
        state: GalleryPipelineStates
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = UpdateGalleryStateMessage::new((gallery_id, state));
        self.sender
            .send(StateTrackerMessage::UpdateGalleryState(msg))
            .await?;
        receiver
            .await
            .map_err(Into::into)
    }

    /// Remove a gallery from state.
    /// 
    /// Returns an `Err` if it doesn't exist.
    pub async fn remove_gallery(
        &mut self,
        gallery_id: GalleryId
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = RemoveGalleryMessage::new(gallery_id);
        self.sender
            .send(StateTrackerMessage::RemoveGallery(msg))
            .await?;
        receiver
            .await
            .map_err(Into::into)
    }
}
