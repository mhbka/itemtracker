use serde::{Serialize, Deserialize};
use crate::galleries::scraping_pipeline::GalleryScrapedState;
use super::ModuleMessageWithReturn;


/// Types of messages the image analysis module can take.
#[derive(Debug)]
pub enum ImgAnalysisMessage {
    StartAnalysis(StartAnalysisJobMessage)
}

/// Message to start analysis of newly scraped items.
pub type StartAnalysisJobMessage = ModuleMessageWithReturn<StartAnalysisJob, ()>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartAnalysisJob {
    pub gallery: GalleryScrapedState
}