use chrono::Utc;
use crate::{domain::pipeline_states::{GalleryPipelineStates, GallerySchedulerState, GallerySearchScrapingState}, messages::{message_types::{scraper_scheduler::SchedulerError, search_scraper::SearchScraperMessage, state_tracker::StateTrackerError}, SearchScraperSender, StateTrackerSender}};

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

    /// Get a ref to the task's gallery.
    pub fn gallery(&self) -> &GallerySchedulerState {
        &self.gallery
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
            if let Err(err) = self.start_scrape().await {
                match err {
                    SchedulerError::MessageErr { gallery_id, err } => {
                        tracing::error!(
                            "Got a message failure trying to start a scrape for gallery {}; returning as this is critical: {}",
                            gallery_id, err
                        );
                        return Err(());
                    },
                    err => tracing::warn!("Got an error trying to start a scrape for gallery {}; continuing...", err)
                }
            }
            tracing::debug!("Successfully started scrape for gallery {}", self.gallery.gallery_id);
            self.sleep_to_next_time().await?;
        }
    }

    /// Update the gallery that will be sent to the scraper.
    pub fn update_gallery(&mut self, updated_gallery: GallerySchedulerState) -> Result<(), SchedulerError> {
        if updated_gallery.gallery_id != self.gallery.gallery_id {
            return Err(SchedulerError::Internal { gallery_id: self.gallery.gallery_id.clone() })
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
    async fn start_scrape(&mut self) -> Result<(), SchedulerError> {
        let gallery_id = self.gallery.gallery_id.clone();

        let gallery = self.gallery.clone();
        let new_gallery_state = GallerySearchScrapingState {
            gallery_id: gallery.gallery_id,
            search_criteria: gallery.search_criteria,
            marketplace_previous_scraped_datetimes: gallery.marketplace_previous_scraped_datetimes,
            evaluation_criteria: gallery.evaluation_criteria
        };

        self.state_tracker_sender
            .add_gallery(
                gallery_id.clone(), 
                GalleryPipelineStates::SearchScraping(new_gallery_state)
            )
            .await
            .map_err(|err| SchedulerError::MessageErr { 
                gallery_id: gallery_id.clone(),
                err 
            })?
            .map_err(|err| SchedulerError::StateErr { 
                gallery_id: gallery_id.clone(), 
                err
            })?;

        self.search_scraper_sender
            .send(SearchScraperMessage::ScrapeSearch { gallery_id: gallery_id.clone() })
            .await
            .map_err(|err| SchedulerError::MessageErr { 
                gallery_id, err
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