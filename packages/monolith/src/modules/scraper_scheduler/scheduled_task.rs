use std::sync::Arc;
use chrono::Utc;
use tokio::sync::Mutex;
use crate::{
    galleries::scraping_pipeline::GalleryInitializationState, 
    messages::{
        message_types::scraper::{
            ScraperMessage, 
            StartScrapingJob, 
            StartScrapingJobMessage
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
    pub async fn run(&mut self) {   
        loop {
            let gallery = self.gallery
                .clone()
                .to_next_stage(); // TODO: Should `previous_scraped_item_datetime` be set in the scraping stage for better accuracy?
            let scraping_job = StartScrapingJob { gallery };
            let (msg, response_receiver) = StartScrapingJobMessage::new(scraping_job);

            self.scraper_msg_sender
                .lock()
                .await
                .send(ScraperMessage::StartScraping(msg))
                .await
                .unwrap(); // TODO: handle this Err case
    
            match response_receiver.await {
                Ok(_) => {},
                Err(err) => todo!(), // TODO: Err case must be handled (at least logged; if I start using state-tracking module, have to let it know too)
            };

            let cur_time = Utc::now();
            match self.gallery.scraping_periodicity
                .get_cron()
                .find_next_occurrence(&cur_time, false) {
                Ok(next_time) => {
                    let time_to_next_schedule = (next_time - cur_time).to_std().unwrap(); // NOTE: can unwrap here as this duration will never be less than 0
                    tokio::time::sleep(time_to_next_schedule).await;
                },
                Err(_) => {
                    // TODO: do logging here, then some way of notifying the scheduler? Quite a critical error if this happens I think
                },
            }
        }
    }

    /// Update the gallery that will be sent to the scraper.
    pub fn update_gallery(&mut self, gallery: GalleryInitializationState) {
        self.gallery = gallery;
    }
}