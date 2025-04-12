use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use crate::domain::domain_types::GalleryId;
use crate::messages::{SearchScraperSender, StateTrackerSender};
use crate::{
    domain::pipeline_states::GallerySchedulerState, 
    messages::message_types::scraper_scheduler::SchedulerError
};

use super::scheduled_task::ScheduledGalleryTask;

/// A map of gallery IDs to their scheduling task.
/// 
/// Aliased since the signature is pretty long.
type GallerySchedulingHandles = Arc<RwLock<HashMap<GalleryId, (Arc<Mutex<ScheduledGalleryTask>>, JoinHandle<()>)>>>;

/// The actual scheduler for the scraper.
pub struct SchedulerHandler {
    galleries: GallerySchedulingHandles, 
    scraper_msg_sender: SearchScraperSender,
    state_tracker_sender: StateTrackerSender
}

impl SchedulerHandler {
    /// Instantiate the scheduler.
    /// 
    /// TODO: be able to instantiate from a Vec of galleries here
    pub fn new(scraper_msg_sender: SearchScraperSender, state_tracker_sender: StateTrackerSender) -> Self {
        Self {
            galleries: Arc::new(RwLock::new(HashMap::new())),
            scraper_msg_sender,
            state_tracker_sender
        }
    }

    /// Add a new gallery to the scheduler.
    pub async fn add_gallery(&self, new_gallery: GallerySchedulerState) -> Result<(), SchedulerError>
    {
        let gallery_id = new_gallery.gallery_id.clone();
        let mut galleries = self.galleries.write().await;
        if galleries.contains_key(&gallery_id) {
            return Err(SchedulerError::GalleryAlreadyExists{ gallery_id });
        }
        let handle = self.generate_gallery_task(new_gallery).await;
        galleries.insert(gallery_id, handle);
        Ok(())
    }

    /// Delete a gallery from the scheduler.
    pub async fn delete_gallery(&self, gallery_id: GalleryId) -> Result<(), SchedulerError> 
    {
        let mut galleries = self.galleries.write().await;
        if let Some(task) = galleries.remove(&gallery_id) {
            task.1.abort();
            Ok(())
        } 
        else {
            Err(SchedulerError::GalleryNotFound{ gallery_id })
        }
    }

    /// Update a gallery in the scheduler.
    pub async fn update_gallery(&self, updated_gallery: GallerySchedulerState) -> Result<(), SchedulerError>
    {   
        let mut galleries = self.galleries.write().await;
        if let Some(task) = galleries.get_mut(&updated_gallery.gallery_id) {
            let mut scheduled_gallery = task.0.lock().await;
            scheduled_gallery.update_gallery(updated_gallery)?;
            Ok(())
        } 
        else {
            Err(SchedulerError::GalleryNotFound{ gallery_id: updated_gallery.gallery_id})
        }
    }

    /// Spawns a task to periodically trigger scraper requests for the input gallery,
    /// returning a handle to the task, and an Arc Mutex handle to the task struct.
    async fn generate_gallery_task(&self, gallery: GallerySchedulerState) 
    -> (Arc<Mutex<ScheduledGalleryTask>>, JoinHandle<()>) 
    {
        let task = ScheduledGalleryTask::new(
            gallery, 
            self.state_tracker_sender.clone(),
            self.scraper_msg_sender.clone()
        );
        let task = Arc::new(Mutex::new(task));
        let cloned_task = task.clone();
        let task_handle = tokio::spawn(
            async move {
                cloned_task
                    .lock()
                    .await
                    .run()
                    .await;
            }
        );
        (task, task_handle)
    }
}