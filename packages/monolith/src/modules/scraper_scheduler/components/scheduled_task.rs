use std::sync::Arc;
use chrono::Utc;
use tokio::sync::Mutex;
use crate::{
    galleries::pipeline_states::GalleryInitializationState, 
    messages::{
        message_types::scraper::{
            ScraperMessage, 
            StartScrapingGallery, 
            StartScrapingGalleryMessage
        },
    ScraperSender}};

/// A wrapper representing the actual running scheduler task for a gallery, which starts on `run()`.
pub struct ScheduledGalleryTask {
    gallery: GalleryInitializationState,
    scraper_msg_sender: Arc<Mutex<ScraperSender>>
}

impl ScheduledGalleryTask {
    /// Initialize a `ScheduledGalleryTask`.
    pub fn new(
        gallery: GalleryInitializationState,
        scraper_msg_sender: Arc<Mutex<ScraperSender>>
    ) -> Self
    {
        Self { gallery, scraper_msg_sender }
    }

    /// Runs the task indefinitely (or until the Cron schedule says to stop, if that's ever the case).
    /// 
    /// Returns with an `Err` whenever the Cron schedule is unable to return the next occurrence.
    pub async fn run(&mut self) -> Result<(), ()>  {   
        loop {
            let gallery = self.gallery
                .clone()
                .to_next_stage(); // TODO: Should `previous_scraped_item_datetime` be set in the scraping stage for better accuracy?
            let scraping_job = StartScrapingGallery { gallery };
            let msg = StartScrapingGalleryMessage::new(scraping_job);
            if let Err(err) =  self.scraper_msg_sender
                .lock()
                .await
                .send(ScraperMessage::StartScrapingGallery(msg))
                .await {
                    tracing::error!(
                        "Error attempting to send a message to the scraper to start scraping (gallery {}): {}",
                        &self.gallery.gallery_id,
                        err
                    );
                }
            let cur_time = Utc::now();
            match self.gallery.scraping_periodicity
                .get_cron()
                .find_next_occurrence(&cur_time, false) {
                Ok(next_time) => {
                    let time_to_next_schedule = (next_time - cur_time)
                        .to_std()
                        .expect("Should never fail as this time should logically always be greater than 0");
                    tokio::time::sleep(time_to_next_schedule).await;
                },
                Err(err) => {
                    // TODO: should this notify the scheduler so that the task can be rebooted/removed? since it's terminated here
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