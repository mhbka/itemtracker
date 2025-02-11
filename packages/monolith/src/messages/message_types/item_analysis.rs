use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::galleries::pipeline_states::GalleryScrapedState;

/// Possible errors emitted from the item analysis module.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ItemAnalysisError {
}

/// The types of messages the item analysis module can take.
#[derive(Debug)]
pub enum ItemAnalysisMessage {
    StartAnalysis { gallery: GalleryScrapedState }
}