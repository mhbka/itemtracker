use std::time::Duration;

use chrono::Utc;
use crate::{domain::pipeline_states::{GallerySchedulerState, GallerySearchScrapingState}, pipeline::instance::PipelineInstance};

use super::error::SchedulerError;

/// A wrapper representing the actual running scheduler task for a gallery, which starts on `run()`.
pub struct ScheduledGalleryTask {
    gallery: GallerySchedulerState,
    pipeline_instance: PipelineInstance
}

impl ScheduledGalleryTask {
    /// Initialize a `ScheduledGalleryTask`.
    pub fn new(
        gallery: GallerySchedulerState,
        pipeline_instance: PipelineInstance
    ) -> Self
    {
        Self { 
            gallery, 
            pipeline_instance
        }
    }

    /// Get a ref to the task's gallery.
    pub fn gallery(&self) -> &GallerySchedulerState {
        &self.gallery
    }

    /// Runs the pipeline for this task once.
    /// 
    /// Returns with an `Err` if:
    /// - we cannot send a message to or receive a response from the state tracker
    /// - the Cron schedule is unable to return the next occurrence
    pub async fn run_once(&mut self) -> Result<(), ()>  {   
        if self.gallery.is_active {
            if let Err(err) = self.start_pipeline().await {
                match err {
                    SchedulerError::MessageFailure => {
                        tracing::error!(
                            "Got a message failure trying to start a scrape for gallery {}; returning as this is critical",
                            self.gallery.gallery_id
                        );
                        return Err(());
                    },
                    err => tracing::warn!(
                        "Got an error trying to start a scrape for gallery {}; continuing: {}", 
                        self.gallery.gallery_id, err
                    )
                }
            }
            tracing::debug!("Successfully ran pipeline for gallery {}", self.gallery.gallery_id);
        }
        else {
            tracing::debug!("Skipping pipeline run for gallery {} (not currently active)", self.gallery.gallery_id)
        }
        Ok(())
    }

    /// Update the gallery that will be sent to the scraper.
    pub fn update_gallery(&mut self, updated_gallery: GallerySchedulerState) -> Result<(), SchedulerError> {
        if updated_gallery.gallery_id != self.gallery.gallery_id {
            return Err(SchedulerError::UpdatedGalleryDoesntMatch)
        }
        self.gallery = updated_gallery;
        Ok(())
    }

    /// Adds a gallery to state and starts the scrape.
    /// 
    /// Returns an Err if the state tracker returned one,
    /// or if the state tracker/search scraper had a message failure.
    /// 
    /// **NOTE**: 
    /// One may continue from a state tracker error,
    /// but should end the task if either had a message failure,
    /// as this is a critical issue of the system.
    async fn start_pipeline(&mut self) -> Result<(), SchedulerError> {
        let gallery_id = self.gallery.gallery_id.clone();

        let gallery = self.gallery.clone();
        let gallery_state = GallerySearchScrapingState {
            gallery_id: gallery.gallery_id,
            search_criteria: gallery.search_criteria,
            marketplace_previous_scraped_datetimes: gallery.marketplace_previous_scraped_datetimes,
            evaluation_criteria: gallery.evaluation_criteria
        };

        let mut pipeline = self.pipeline_instance.clone();
        let pipeline_result = pipeline.run_pipeline(gallery_state).await;
        if let Err(err) = pipeline_result {
            // TODO: persist this error to a store or something?
            tracing::warn!("A pipeline run for gallery {gallery_id} failed (scheduler task will continue): {err}");
        }

        Ok(())
    }

    /// Gets the time till the next schedule for this task.
    ///
    /// Returns an `Err` if the Cron cannot get the next scheduled time (should never happen).
    pub async fn time_to_next_schedule(&mut self) -> Result<Duration, ()> {
        let cur_time = Utc::now();
        let next_time = self.gallery.scraping_periodicity
            .get_cron()
            .find_next_occurrence(&cur_time, false);
        match next_time {
            Ok(next_time) => {
                let time_to_next_schedule = (next_time - cur_time)
                    .to_std()
                    .expect("Should never fail, as this time should logically always be greater than 0");
                Ok(time_to_next_schedule)
            },
            Err(err) => {
                // TODO: pretty critical error, should have some way to persist this info
                tracing::error!(
                    "Error trying to schedule the next scrape for gallery {}; this gallery will now stop: {}",
                    &self.gallery.gallery_id,
                    err
                );
                return Err(());
            },
        }
    }
}