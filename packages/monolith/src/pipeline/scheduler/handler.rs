use std::{collections::HashMap, sync::Arc};
use chrono::{TimeDelta, Utc};
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use crate::domain::domain_types::GalleryId;
use crate::pipeline::instance::PipelineInstance;
use crate::{
    domain::pipeline_states::GallerySchedulerState, 
};
use super::error::SchedulerError;
use super::scheduled_task::ScheduledGalleryTask;

/// A map of gallery IDs to their scheduling task.
/// 
/// Aliased since the signature is pretty long.
type GallerySchedulingHandles = Arc<RwLock<HashMap<GalleryId, (Arc<Mutex<ScheduledGalleryTask>>, JoinHandle<()>)>>>;

/// The actual scheduler for the scraper.
pub struct Handler {
    galleries: GallerySchedulingHandles,
    pipeline_instance: PipelineInstance
}

impl Handler {
    /// Instantiate the scheduler.
    pub async fn new(initial_state: Vec<GallerySchedulerState>, pipeline_instance: PipelineInstance) -> Self {
        let mut handler = Self {
            galleries: Arc::new(RwLock::new(HashMap::new())),
            pipeline_instance
        };

        handler.initialize_tasks(initial_state).await;

        handler
    }

    /// Add a gallery to the scheduler.
    /// 
    /// It will wait until it is next due, based on the latest datetime in `marketplace_latest_scraped_datetimes` + `scraping_periodicity`,
    /// unless that time has already passed, in which case it will immediately begin running.
    pub async fn add_gallery(&self, new_gallery: GallerySchedulerState) -> Result<(), SchedulerError>
    {
        let gallery_id = new_gallery.gallery_id.clone();
        let mut galleries = self.galleries.write().await;
        if galleries.contains_key(&gallery_id) {
            return Err(SchedulerError::GalleryAlreadyExists);
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
            Err(SchedulerError::GalleryDoesntExist)
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
            Err(SchedulerError::GalleryDoesntExist)
        }
    }

    /// Spawns a task to repeatedly trigger scraper requests for the input gallery 
    /// (with interval based on the gallery's `scraping_periodicity`), 
    /// returning a handle to the task + an `Arc Mutex` handle to the task struct.
    /// 
    /// ### Note
    /// If, based on the latest datetime in the gallery's `marketplace_last_scraped_datetimes`,
    /// the gallery isn't due to be scraped yet,
    /// the task will sleep for that time difference.
    /// 
    /// ### Examples
    /// Will sleep first:
    /// - The scraping periodicity is once a day
    /// - The latest datetime in `marketplace_last_scraped_datetimes` is 12 hours ago
    /// - The task will sleep for 12 more hours, then begin running
    /// 
    /// Will immediately run:
    /// - The scraping periodicity is once a day
    /// - The latest datetime is 48 hours ago
    /// - The task will immediately begin running
    async fn generate_gallery_task(&self, gallery: GallerySchedulerState) 
    -> (Arc<Mutex<ScheduledGalleryTask>>, JoinHandle<()>) 
    {   
        let task = ScheduledGalleryTask::new(
            gallery, 
            self.pipeline_instance.clone()
        );
        let task = Arc::new(Mutex::new(task));
        let cloned_task = task.clone();

        let task_handle = tokio::spawn(
            async move {
                let mut task = cloned_task
                    .lock()
                    .await;

                // see if we still have time till the next schedule; if so, sleep...
                let gallery = task.gallery();
                let cur_time = Utc::now();
                let next_time = match gallery.scraping_periodicity
                    .get_cron()
                    .find_next_occurrence(&cur_time, false)
                {
                    Ok(next_time) => next_time,
                    Err(err) => cur_time
                };
                let wait_time = next_time - cur_time;
                if wait_time > TimeDelta::zero() {
                    let wait_time = wait_time
                        .to_std()
                        .expect("Should not fail as time is greater than zero");

                    tracing::debug!(
                        "Gallery {} task initialization: will sleep for {:?} till next schedule ({})",
                        gallery.gallery_id, wait_time, next_time
                    );

                    sleep(wait_time).await;
                }

                // ...then begin running indefinitely
                tracing::debug!("Now running task for gallery {}", gallery.gallery_id);

                let task_run_result = task
                    .run()
                    .await;
                if let Err(err) = task_run_result {
                    tracing::error!(
                        "The scheduling task for the following gallery got an unexpected error and returned: {:#?}", 
                        task.gallery()
                    );
                }
            }
        );

        (task, task_handle)
    }

    /// Start up tasks for all present galleries.
    async fn initialize_tasks(&mut self, galleries: Vec<GallerySchedulerState>) {
        let total = galleries.len();
        let mut successful = 0;
        let mut failed = 0;

        for gallery in galleries {
            let gallery_id = gallery.gallery_id.clone();
            match self.add_gallery(gallery).await {
                Ok(_) => successful += 1,
                Err(err) => {
                    tracing::warn!("Failed to initialize task for gallery {gallery_id}: {err}");
                    failed += 1;
                }
            }
        }

        tracing::info!("Finished initializing {total} galleries in the scheduler ({successful} successful, {failed} failed)");
    }
}