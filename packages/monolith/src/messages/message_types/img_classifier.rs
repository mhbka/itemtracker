use serde::{Serialize, Deserialize};
use crate::galleries::pipeline_states::GalleryAnalyzedState;
use super::ModuleMessage;

#[derive(Debug)]
pub enum ImgClassifierMessage {
    StartClassification(StartClassificationJobMessage)
}

/// Message to start classifying analyzed items.
pub type StartClassificationJobMessage = ModuleMessage<StartClassificationJob>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartClassificationJob {
    pub gallery: GalleryAnalyzedState
}