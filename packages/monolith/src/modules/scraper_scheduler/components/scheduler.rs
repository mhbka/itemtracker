use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use crate::galleries::domain_types::GalleryId;
use crate::messages::ScraperSender;
use crate::{
    galleries::pipeline_states::GalleryInitializationState, 
    messages::message_types::scraper_scheduler::{EditGallery, NewGallery, SchedulerError}
};

use super::scheduled_task::ScheduledGalleryTask;

/// The actual scheduler for the scraper.
pub struct RawScraperScheduler {
    /// A mapping of gallery IDs to their running scheduling tasks.
    /// 
    /// Contains the `JoinHandle` to the actual running task, as well as a
    /// `ScheduledGalleryTaskHandle` which can be used to modify the task's gallery's parameters.
    galleries: Arc<RwLock<HashMap<GalleryId, (Arc<Mutex<ScheduledGalleryTask>>, JoinHandle<()>)>>>,
    
    /// Handle for sending messages to the scraper.
    /// 
    /// This is cloned and passed into each gallery task.
    scraper_msg_sender: Arc<Mutex<ScraperSender>>
}

impl RawScraperScheduler {
    /// Instantiate the scheduler.
    /// 
    /// TODO: be able to instantiate from a Vec of galleries here
    pub fn new(scraper_msg_sender: Arc<Mutex<ScraperSender>>) -> Self {
        Self {
            galleries: Arc::new(RwLock::new(HashMap::new())),
            scraper_msg_sender
        }
    }

    /// Add a new gallery to the scheduler.
    #[tracing::instrument(skip(self))]
    pub async fn add_gallery(&self, new_gallery: NewGallery) -> Result<(), SchedulerError>
    {
        let new_gallery = new_gallery.gallery;
        let gallery_id = new_gallery.gallery_id.clone();
        let mut galleries = self.galleries.write().await;
        if galleries.contains_key(&gallery_id) {
            tracing::error!("Gallery with ID {} already exists", gallery_id);
            return Err(SchedulerError::GalleryAlreadyExists{ gallery_id });
        }
        let handle = self.generate_gallery_task(new_gallery).await;
        galleries.insert(gallery_id, handle);
        Ok(())
    }

    /// Delete a gallery from the scheduler.
    #[tracing::instrument(skip(self))]
    pub async fn delete_gallery(&self, gallery_id: GalleryId) -> Result<(), SchedulerError> 
    {
        let mut galleries = self.galleries.write().await;
        if let Some(task) = galleries.remove(&gallery_id) {
            task.1.abort();
            Ok(())
        } else {
            tracing::error!("Gallery with ID {} not found", gallery_id);
            Err(SchedulerError::GalleryNotFound{ gallery_id })
        }
    }

    /// Update a gallery in the scheduler.
    #[tracing::instrument(skip(self))]
    pub async fn update_gallery(&self, edited_gallery: EditGallery) -> Result<(), SchedulerError>
    {   
        let gallery = edited_gallery.gallery;
        let mut galleries = self.galleries.write().await;
        if let Some(task) = galleries.get_mut(&gallery.gallery_id) {
            let mut scheduled_gallery = task.0.lock().await;
            scheduled_gallery.update_gallery(gallery);
            Ok(())
        } else {
            tracing::error!("Gallery with ID {} not found", gallery.gallery_id);
            Err(SchedulerError::GalleryNotFound{ gallery_id: gallery.gallery_id})
        }
    }

    /// Spawns a task to periodically trigger scraper requests for the input gallery,
    /// returning a handle to the task, and an Arc Mutex handle to the task struct.
    async fn generate_gallery_task(&self, gallery: GalleryInitializationState) 
    -> (Arc<Mutex<ScheduledGalleryTask>>, JoinHandle<()>) 
    {
        let scraper_msg_sender = self.scraper_msg_sender.clone();
        let task = Arc::new(Mutex::new(ScheduledGalleryTask::new(gallery, scraper_msg_sender)));
        let cloned_task = task.clone();
        let join_handle = tokio::spawn(
            async move {
                cloned_task
                    .lock()
                    .await
                    .run()
                    .await;
            }
        );

        (task, join_handle)
    }
}