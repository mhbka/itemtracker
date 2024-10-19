use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use uuid::Uuid;
use chrono::Utc;
use crate::{galleries::{Galleries, MercariGallery}, output::{self, OutputRequester}};

/// Provides specific functionality for our job scheduling.
pub struct SchedulerWrapper {
    scheduler: JobScheduler,
    output_requester: Arc<Mutex<OutputRequester>>
}

impl SchedulerWrapper {
    /// Creates a new wrapper for the scheduler.
    pub fn new(scheduler: JobScheduler, output_requester: Arc<Mutex<OutputRequester>>) -> Self {
        SchedulerWrapper { 
            scheduler,
            output_requester
        } 
    }

    /// Adds the gallery to the scheduler as a job and returns the job's Uuid.
    pub async fn add_gallery(&mut self, gallery: &mut Galleries) -> Result<Uuid, JobSchedulerError> {
        let job = self.create_gallery_job(gallery.clone())?;
        self.scheduler.add(job).await
    }
    
    /// Edits the gallery job, returning its new Uuid.
    pub async fn edit_gallery(&mut self, job_id: Uuid, gallery: &mut Galleries) -> Result<Uuid, JobSchedulerError> {
        let time_to_next = match self.scheduler.next_tick_for_job(job_id.clone()).await? {
            Some(next_time) => (Utc::now() - next_time).to_std().unwrap(),
            None => Duration::from_secs(0)
        };
        self.scheduler.remove(&job_id).await?;
        tokio::time::sleep(time_to_next).await;
        let new_job_id = self.add_gallery(gallery).await?;
        Ok(new_job_id)
    }

    /// Creates a job for a gallery.
    fn create_gallery_job(&self, gallery: Galleries) -> Result<Job, JobSchedulerError> {
        let output_requester = self.output_requester.clone(); // so that I can move into first closure...
        Job::new_async(gallery.get_gallery_periodicity(), move |_uuid, mut _l| {
            let gallery = gallery.clone();
            let output_requester = output_requester.clone(); // then another closure...
            Box::pin(async move {
                let mut requester = output_requester
                    .lock()
                    .await;
                match gallery {
                    Galleries::Mercari(mercari_gallery) => requester.schedule_mercari_task(mercari_gallery.clone()).await
                };
            })
        })
    }
}