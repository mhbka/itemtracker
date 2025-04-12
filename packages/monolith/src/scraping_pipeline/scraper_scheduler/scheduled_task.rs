use chrono::Utc;
use crate::{domain::pipeline_states::{GalleryPipelineStates, GallerySchedulerState, GallerySearchScrapingState}, messages::{message_types::{scraper_scheduler::SchedulerError, state_tracker::StateTrackerError}, SearchScraperSender, StateTrackerSender}};

/// A wrapper representing the actual running scheduler task for a gallery, which starts on `run()`.
pub struct ScheduledGalleryTask {
    gallery: GallerySchedulerState,
    state_tracker_sender: StateTrackerSender,
    search_scraper_sender: SearchScraperSender
}

impl ScheduledGalleryTask {
    /// Initialize a `ScheduledGalleryTask`.
    pub fn new(
        gallery: GallerySchedulerState,
        state_tracker_sender: StateTrackerSender,
        search_scraper_sender: SearchScraperSender
    ) -> Self
    {
        Self { 
            gallery, 
            state_tracker_sender,
            search_scraper_sender
        }
    }

    /// Schedules the gallery at the appointed periodicity, and registers the gallery in the state tracker.
    /// 
    /// If the gallery is already registered with the state tracker, it won't be scheduled.
    /// 
    /// Returns with an `Err` if:
    /// - we cannot send a message to or receive a response from the state tracker
    /// - the Cron schedule is unable to return the next occurrence
    pub async fn run(&mut self) -> Result<(), ()>  {   
        loop {
            match self.add_gallery_to_state().await {
                Ok(res) => {
                    if let Err(err) = res {
                        tracing::warn!("Could not add gallery {} to state; it already exists", self.gallery.gallery_id);
                    }
                },
                Err(err) => {
                    tracing::error!("Error adding new gallery to state: {err}");
                    return Err(());
                } 
            }
            self.sleep_to_next_time().await?;
        }
    }

    /// Update the gallery that will be sent to the scraper.
    pub fn update_gallery(&mut self, updated_gallery: GallerySchedulerState) -> Result<(), SchedulerError> {
        if updated_gallery.gallery_id != self.gallery.gallery_id {
            return Err(SchedulerError::GalleryUpdateHasWrongId { gallery_id: self.gallery.gallery_id.clone() })
        }
        self.gallery = updated_gallery;
        Ok(())
    }

    /// Adds a gallery to state.
    /// 
    /// Returns an Err if unable to contact the state tracker.
    /// Inside, returns an `Err` if the gallery already exists in state.
    async fn add_gallery_to_state(&mut self) -> Result<Result<(), StateTrackerError>, SchedulerError> {
        let gallery = self.gallery.clone();
        let new_gallery_state = GallerySearchScrapingState {
            gallery_id: gallery.gallery_id,
            search_criteria: gallery.search_criteria,
            marketplace_previous_scraped_datetimes: gallery.marketplace_previous_scraped_datetimes,
            evaluation_criteria: gallery.evaluation_criteria
        };
        self.state_tracker_sender
            .add_gallery(
                new_gallery_state.gallery_id.clone(), 
                GalleryPipelineStates::SearchScraping(new_gallery_state)
            )
            .await
            .map_err(|err| SchedulerError::Other { 
                gallery_id: self.gallery.gallery_id.clone(),
                message: format!("Unable to send message to state tracker: {err}") 
            })
    }

    /// Sleeps till the next scheduled time.
    ///
    /// Returns an `Err` if the Cron cannot get the next scheduled time (should never happen).
    async fn sleep_to_next_time(&mut self) -> Result<(), ()> {
        let cur_time = Utc::now();
        let next_time = self.gallery.scraping_periodicity
            .get_cron()
            .find_next_occurrence(&cur_time, false);
        match next_time {
            Ok(next_time) => {
                let time_to_next_schedule = (next_time - cur_time)
                    .to_std()
                    .expect("Should never fail, as this time should logically always be greater than 0");
                tokio::time::sleep(time_to_next_schedule).await;
                Ok(())
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