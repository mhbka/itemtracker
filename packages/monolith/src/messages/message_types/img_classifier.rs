use serde::{Serialize, Deserialize};
use crate::galleries::pipeline_states::GalleryAnalyzedState;

/// The types of messages the image classifer module can take.
#[derive(Debug)]
pub enum ImgClassifierMessage {
    StartClassification(StartClassificationJob)
}

/// Message to start image classification for a gallery.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartClassificationJob {
    pub gallery: GalleryAnalyzedState
}