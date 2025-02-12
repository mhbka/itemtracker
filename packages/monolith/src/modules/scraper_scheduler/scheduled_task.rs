use chrono::Utc;
use crate::{galleries::pipeline_states::GalleryInitializationState, messages::{message_types::{search_scraper::SearchScraperMessage, state_tracker::{AddNewGalleryMessage, StateTrackerMessage}}, SearchScraperSender, StateTrackerSender}};

/// A wrapper representing the actual running scheduler task for a gallery, which starts on `run()`.
pub struct ScheduledGalleryTask {
    gallery: GalleryInitializationState,
    state_tracker_sender: StateTrackerSender,
    search_scraper_sender: SearchScraperSender
}

impl ScheduledGalleryTask {
    /// Initialize a `ScheduledGalleryTask`.
    pub fn new(
        gallery: GalleryInitializationState,
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
            let (state_msg, receiver) = AddNewGalleryMessage::new(self.gallery.gallery_id.clone());
            if let Err(err) = self.state_tracker_sender
                .send(StateTrackerMessage::AddNewGallery(state_msg))
                .await {
                    tracing::error!("Unable to send a message to the state tracker: {err}");
                    return Err(());
                }
            match receiver.await
                .map_err(|err| {
                    tracing::error!("Unable to receive response from state tracker: {err}");
                    ()
                })? {
                    Err(err) => tracing::error!("Gallery {} is already in state", self.gallery.gallery_id),
                    Ok(ok) => {
                        let gallery = self.gallery
                            .clone()
                            .to_next_stage();
                        if let Err(err) =  self.search_scraper_sender
                            .send(SearchScraperMessage::ScrapeSearch { gallery })
                            .await {
                                tracing::error!(
                                    "Error attempting to send a message to the scraper to start scraping (gallery {}): {}",
                                    &self.gallery.gallery_id,
                                    err
                                );
                            }               
                    }
                };
            let cur_time = Utc::now();
            match self.gallery.scraping_periodicity
                .get_cron()
                .find_next_occurrence(&cur_time, false) {
                Ok(next_time) => {
                    let time_to_next_schedule = (next_time - cur_time)
                        .to_std()
                        .expect("Should never fail, as this time should logically always be greater than 0");
                    tokio::time::sleep(time_to_next_schedule).await;
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

    /// Update the gallery that will be sent to the scraper.
    pub fn update_gallery(&mut self, gallery: GalleryInitializationState) {
        self.gallery = gallery;
    }
}